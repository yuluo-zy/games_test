// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/startup_critters.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 07b49de8662c81432093952ac97e6cdb842ee15ec6bbca41c261923527b4e709
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_critters.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 477
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_critters::startup_critters
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":477,"record_offset":3496,"rva":6742464,"va":5375451584}
// 编译实例 RVAs: 0x66e1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2702230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e86ac4d6ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375451921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2706727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8d9b2d6ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375456434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b6e80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-751268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e85c89f4ff","size":5,"soft_fail":false,"target_va":5374701184,"va":5375452447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t278647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e877400400","size":5,"soft_fail":false,"target_va":5375731504,"va":5375452852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e5f70
//     owner=unresolved_namespace; count=1
//     target_aliases=22 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t484827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8db650700","size":5,"soft_fail":false,"target_va":5375942512,"va":5375457680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t532197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8e51e0800","size":5,"soft_fail":false,"target_va":5375988432,"va":5375456230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc64a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6652067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8a3806500","size":5,"soft_fail":false,"target_va":5382104224,"va":5375452152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f1eb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9109459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8d3ff8a00","size":5,"soft_fail":false,"target_va":5384563488,"va":5375454024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f1f1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9111776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8e0088b00","size":5,"soft_fail":false,"target_va":5384565232,"va":5375453451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28140755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8d364ad01","size":5,"soft_fail":false,"target_va":5403594144,"va":5375453384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t27522433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e881f5a301","size":5,"soft_fail":false,"target_va":5402978144,"va":5375455706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t1906533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e865171d00","size":5,"soft_fail":false,"target_va":5377364336,"va":5375457798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t8927771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e81b3a8800","size":5,"soft_fail":false,"target_va":5384380192,"va":5375452416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t27524437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e855fda301","size":5,"soft_fail":false,"target_va":5402980128,"va":5375455686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-886713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e84778f2ff","size":5,"soft_fail":false,"target_va":5374566080,"va":5375452788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::add_color_self
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9114141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e81d128b00","size":5,"soft_fail":false,"target_va":5384567760,"va":5375453614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-776129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e83f28f4ff","size":5,"soft_fail":false,"target_va":5374675760,"va":5375451884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t10397911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8d7a89e00","size":5,"soft_fail":false,"target_va":5385850032,"va":5375452116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::radical_inverse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1487775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e89fb31600","size":5,"soft_fail":false,"target_va":5376941136,"va":5375453356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::uniform_sample_disk
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1487316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451584,"raw_hex":"e8d4b11600","size":5,"soft_fail":false,"target_va":5376940464,"va":5375453143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":477,"record_offset":4468,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":477,"record_offset":4356,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":477,"record_offset":3672,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":477,"record_offset":3728,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":21881,"source_line":301,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":477,"record_offset":5308,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":477,"record_offset":4628,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::from_scale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":477,"record_offset":5276,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":27040,"source_line":41,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::from_translation
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":477,"record_offset":4436,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":21466,"source_line":33,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":477,"record_offset":6380,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":477,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3496,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":477,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3496,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":477,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":3496,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":477,"record_offset":4088,"parent_record_offset":4060,"ancestor_record_offset":3496,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":477,"record_offset":5656,"parent_record_offset":5600,"ancestor_record_offset":3496,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":477,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":3496,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":477,"record_offset":4060,"parent_record_offset":4032,"ancestor_record_offset":3496,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":477,"record_offset":4032,"parent_record_offset":4004,"ancestor_record_offset":3496,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":477,"record_offset":4208,"parent_record_offset":4032,"ancestor_record_offset":3496,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":477,"record_offset":5600,"parent_record_offset":5572,"ancestor_record_offset":3496,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":477,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":3496,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":477,"record_offset":3940,"parent_record_offset":3912,"ancestor_record_offset":3496,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":477,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":3496,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":477,"record_offset":3912,"parent_record_offset":3884,"ancestor_record_offset":3496,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":477,"record_offset":3856,"parent_record_offset":3828,"ancestor_record_offset":3496,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":477,"record_offset":3828,"parent_record_offset":3728,"ancestor_record_offset":3496,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":477,"record_offset":4740,"parent_record_offset":4692,"ancestor_record_offset":3496,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":477,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":3496,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":477,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":3496,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":477,"record_offset":4324,"parent_record_offset":3728,"ancestor_record_offset":3496,"inlinee":21883,"source_line":647,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":477,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":3496,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":477,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":3496,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":477,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":3496,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":477,"record_offset":3772,"parent_record_offset":3728,"ancestor_record_offset":3496,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":477,"record_offset":4244,"parent_record_offset":3968,"ancestor_record_offset":3496,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":477,"record_offset":4404,"parent_record_offset":4356,"ancestor_record_offset":3496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":477,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":3496,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":477,"record_offset":4692,"parent_record_offset":4628,"ancestor_record_offset":3496,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":477,"record_offset":4664,"parent_record_offset":4628,"ancestor_record_offset":3496,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":477,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":3496,"inlinee":21928,"source_line":109,"archive":"_pdb/A/modules/00477.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_critters.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_critters.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_critters.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_critters::startup_critters
// 信息 ID: 9511cba0d772c92a1ca6699f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4e1aad1437a3aaa713247c09: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0xe7223f -> 0x66e1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0xf18ce1 -> 0x66e1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
