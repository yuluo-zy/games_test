// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/startup_sheep.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ce0c8c8345197577d28a62bbf6ad8e88f856860c68bdb4b267a8907515839399
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_sheep.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 419, 1321
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_sheep::startup_sheep
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":419,"record_offset":22328,"rva":5863392,"va":5374572512}
// 编译实例 RVAs: 0x5977e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4958796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8b455b4ff","size":5,"soft_fail":false,"target_va":5369614224,"va":5374573015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4955856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e83061b4ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5374573883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1826198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e86a22e4ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374575889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1826538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e81621e4ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374576245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b2850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t105064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8689a0100","size":5,"soft_fail":false,"target_va":5374683216,"va":5374578147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b6e80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t128473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8d9f50100","size":5,"soft_fail":false,"target_va":5374701184,"va":5374572706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1155110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e826a01100","size":5,"soft_fail":false,"target_va":5375731504,"va":5374576389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1359901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e81dc01400","size":5,"soft_fail":false,"target_va":5375936112,"va":5374576206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e5f70
//     owner=unresolved_namespace; count=1
//     target_aliases=22 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1360108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8ecc01400","size":5,"soft_fail":false,"target_va":5375942512,"va":5374582399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e7730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1366541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e80dda1400","size":5,"soft_fail":false,"target_va":5375948592,"va":5374582046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1407541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8357a1500","size":5,"soft_fail":false,"target_va":5375988432,"va":5374580886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc64a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7525846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8d6d57200","size":5,"soft_fail":false,"target_va":5382104224,"va":5374578373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d281b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7929131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e82bfd7800","size":5,"soft_fail":false,"target_va":5382504880,"va":5374575744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d83780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8305773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e86dbc7e00","size":5,"soft_fail":false,"target_va":5382879104,"va":5374573326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29019941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e825cfba01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374573414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: startup_sheep.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29019855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8cfceba01","size":5,"soft_fail":false,"target_va":5403593392,"va":5374573532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28398796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8cc54b101","size":5,"soft_fail":false,"target_va":5402978144,"va":5374579343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8209317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8a5437d00","size":5,"soft_fail":false,"target_va":5382785008,"va":5374575686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2784316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e83c7c2a00","size":5,"soft_fail":false,"target_va":5377364336,"va":5374580015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9796377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8197b9500","size":5,"soft_fail":false,"target_va":5384375280,"va":5374578898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28399665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e83158b101","size":5,"soft_fail":false,"target_va":5402978880,"va":5374579210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28399257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e89956b101","size":5,"soft_fail":false,"target_va":5402979152,"va":5374579890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t28400263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8875ab101","size":5,"soft_fail":false,"target_va":5402979344,"va":5374579076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28400718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e84e5cb101","size":5,"soft_fail":false,"target_va":5402980128,"va":5374579405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-10045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8c3d8ffff","size":5,"soft_fail":false,"target_va":5374566080,"va":5374576120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::try_get_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8b8eeffff","size":5,"soft_fail":false,"target_va":5374568224,"va":5374572643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1114786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8a2021100","size":5,"soft_fail":false,"target_va":5375692416,"va":5374577625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t97877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8557e0100","size":5,"soft_fail":false,"target_va":5374675760,"va":5374577878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::critters::BirdSpotManager::create_spots
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t623167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e83f820900","size":5,"soft_fail":false,"target_va":5375201520,"va":5374578348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::startup_sheep::startup_sheep::panic_cold_explicit
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t28379528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e88809b101","size":5,"soft_fail":false,"target_va":5402958784,"va":5374579251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::debug::sheep_controls::SheepControl::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11281304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e89823ac00","size":5,"soft_fail":false,"target_va":5385860256,"va":5374578947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11271865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374572512,"raw_hex":"e8b9feab00","size":5,"soft_fail":false,"target_va":5385850032,"va":5374578162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":36512,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":419,"record_offset":36664,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":419,"record_offset":23804,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":419,"record_offset":23916,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":419,"record_offset":24204,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":419,"record_offset":36424,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":419,"record_offset":24056,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":419,"record_offset":23832,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":419,"record_offset":35584,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":419,"record_offset":32676,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":419,"record_offset":22528,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":419,"record_offset":31976,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":419,"record_offset":35716,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":21881,"source_line":301,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::BirdSpotManager::gen_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":419,"record_offset":36480,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":24180,"source_line":292,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":419,"record_offset":32032,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":419,"record_offset":22676,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::from_translation_scale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":419,"record_offset":36748,"parent_record_offset":22328,"ancestor_record_offset":22328,"inlinee":24181,"source_line":49,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":32452,"parent_record_offset":32428,"ancestor_record_offset":22328,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":32476,"parent_record_offset":32452,"ancestor_record_offset":22328,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":32912,"parent_record_offset":32888,"ancestor_record_offset":22328,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":32428,"parent_record_offset":32400,"ancestor_record_offset":22328,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":32888,"parent_record_offset":32796,"ancestor_record_offset":22328,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":419,"record_offset":32824,"parent_record_offset":32796,"ancestor_record_offset":22328,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":419,"record_offset":23992,"parent_record_offset":23960,"ancestor_record_offset":22328,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":419,"record_offset":32400,"parent_record_offset":32372,"ancestor_record_offset":22328,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":419,"record_offset":32372,"parent_record_offset":32344,"ancestor_record_offset":22328,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":419,"record_offset":32548,"parent_record_offset":32372,"ancestor_record_offset":22328,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":419,"record_offset":32796,"parent_record_offset":32772,"ancestor_record_offset":22328,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":419,"record_offset":32308,"parent_record_offset":32280,"ancestor_record_offset":22328,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":419,"record_offset":36688,"parent_record_offset":36664,"ancestor_record_offset":22328,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":419,"record_offset":32280,"parent_record_offset":32252,"ancestor_record_offset":22328,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":419,"record_offset":32224,"parent_record_offset":32196,"ancestor_record_offset":22328,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":419,"record_offset":32252,"parent_record_offset":32224,"ancestor_record_offset":22328,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":419,"record_offset":32196,"parent_record_offset":32168,"ancestor_record_offset":22328,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":419,"record_offset":32168,"parent_record_offset":32140,"ancestor_record_offset":22328,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":419,"record_offset":32140,"parent_record_offset":32096,"ancestor_record_offset":22328,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":419,"record_offset":32000,"parent_record_offset":31976,"ancestor_record_offset":22328,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":419,"record_offset":23960,"parent_record_offset":23916,"ancestor_record_offset":22328,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":419,"record_offset":36712,"parent_record_offset":36688,"ancestor_record_offset":22328,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":419,"record_offset":32344,"parent_record_offset":32308,"ancestor_record_offset":22328,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":419,"record_offset":23856,"parent_record_offset":23832,"ancestor_record_offset":22328,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":419,"record_offset":32704,"parent_record_offset":32676,"ancestor_record_offset":22328,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":419,"record_offset":36312,"parent_record_offset":35716,"ancestor_record_offset":22328,"inlinee":21883,"source_line":647,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":419,"record_offset":36448,"parent_record_offset":36424,"ancestor_record_offset":22328,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":419,"record_offset":24108,"parent_record_offset":24080,"ancestor_record_offset":22328,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":419,"record_offset":23604,"parent_record_offset":23576,"ancestor_record_offset":22328,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":419,"record_offset":22932,"parent_record_offset":22908,"ancestor_record_offset":22328,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":419,"record_offset":23016,"parent_record_offset":22992,"ancestor_record_offset":22328,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":419,"record_offset":24416,"parent_record_offset":24392,"ancestor_record_offset":22328,"inlinee":8489,"source_line":696,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":419,"record_offset":24700,"parent_record_offset":24672,"ancestor_record_offset":22328,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":419,"record_offset":24600,"parent_record_offset":24576,"ancestor_record_offset":22328,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":419,"record_offset":24576,"parent_record_offset":24416,"ancestor_record_offset":22328,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":24500,"parent_record_offset":24476,"ancestor_record_offset":22328,"inlinee":8426,"source_line":190,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":24524,"parent_record_offset":24500,"ancestor_record_offset":22328,"inlinee":8427,"source_line":205,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":24476,"parent_record_offset":24452,"ancestor_record_offset":22328,"inlinee":8424,"source_line":183,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":24772,"parent_record_offset":24748,"ancestor_record_offset":22328,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":24648,"parent_record_offset":24624,"ancestor_record_offset":22328,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":24796,"parent_record_offset":24772,"ancestor_record_offset":22328,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":24672,"parent_record_offset":24648,"ancestor_record_offset":22328,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":24844,"parent_record_offset":24796,"ancestor_record_offset":22328,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":419,"record_offset":25236,"parent_record_offset":24796,"ancestor_record_offset":22328,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":419,"record_offset":22956,"parent_record_offset":22932,"ancestor_record_offset":22328,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":419,"record_offset":24136,"parent_record_offset":24080,"ancestor_record_offset":22328,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":419,"record_offset":24080,"parent_record_offset":24056,"ancestor_record_offset":22328,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":419,"record_offset":23880,"parent_record_offset":23856,"ancestor_record_offset":22328,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":419,"record_offset":24160,"parent_record_offset":24136,"ancestor_record_offset":22328,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":419,"record_offset":32852,"parent_record_offset":32824,"ancestor_record_offset":22328,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":419,"record_offset":24944,"parent_record_offset":24844,"ancestor_record_offset":22328,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":419,"record_offset":24912,"parent_record_offset":24844,"ancestor_record_offset":22328,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":419,"record_offset":23136,"parent_record_offset":23112,"ancestor_record_offset":22328,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":419,"record_offset":35760,"parent_record_offset":35716,"ancestor_record_offset":22328,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":419,"record_offset":32584,"parent_record_offset":32308,"ancestor_record_offset":22328,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":419,"record_offset":22572,"parent_record_offset":22528,"ancestor_record_offset":22328,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":419,"record_offset":23508,"parent_record_offset":23484,"ancestor_record_offset":22328,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":419,"record_offset":23400,"parent_record_offset":23376,"ancestor_record_offset":22328,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":419,"record_offset":32504,"parent_record_offset":32476,"ancestor_record_offset":22328,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":419,"record_offset":24024,"parent_record_offset":23916,"ancestor_record_offset":22328,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":419,"record_offset":23688,"parent_record_offset":23660,"ancestor_record_offset":22328,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":419,"record_offset":23716,"parent_record_offset":23688,"ancestor_record_offset":22328,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":419,"record_offset":24232,"parent_record_offset":24204,"ancestor_record_offset":22328,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":419,"record_offset":24260,"parent_record_offset":24232,"ancestor_record_offset":22328,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":419,"record_offset":23632,"parent_record_offset":23604,"ancestor_record_offset":22328,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":419,"record_offset":32096,"parent_record_offset":32032,"ancestor_record_offset":22328,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":419,"record_offset":32068,"parent_record_offset":32032,"ancestor_record_offset":22328,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":419,"record_offset":23576,"parent_record_offset":23548,"ancestor_record_offset":22328,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":419,"record_offset":22700,"parent_record_offset":22676,"ancestor_record_offset":22328,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":419,"record_offset":22724,"parent_record_offset":22700,"ancestor_record_offset":22328,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":419,"record_offset":23548,"parent_record_offset":23328,"ancestor_record_offset":22328,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":419,"record_offset":22756,"parent_record_offset":22724,"ancestor_record_offset":22328,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":23460,"parent_record_offset":23436,"ancestor_record_offset":22328,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":23436,"parent_record_offset":23328,"ancestor_record_offset":22328,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":23376,"parent_record_offset":23352,"ancestor_record_offset":22328,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":23296,"parent_record_offset":22832,"ancestor_record_offset":22328,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":23352,"parent_record_offset":23328,"ancestor_record_offset":22328,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":22808,"parent_record_offset":22784,"ancestor_record_offset":22328,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":22784,"parent_record_offset":22724,"ancestor_record_offset":22328,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":22832,"parent_record_offset":22808,"ancestor_record_offset":22328,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":419,"record_offset":23080,"parent_record_offset":23048,"ancestor_record_offset":22328,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":419,"record_offset":23112,"parent_record_offset":23080,"ancestor_record_offset":22328,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":419,"record_offset":23172,"parent_record_offset":23048,"ancestor_record_offset":22328,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":419,"record_offset":23048,"parent_record_offset":22832,"ancestor_record_offset":22328,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":22880,"parent_record_offset":22832,"ancestor_record_offset":22328,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":419,"record_offset":23328,"parent_record_offset":22832,"ancestor_record_offset":22328,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":419,"record_offset":22908,"parent_record_offset":22832,"ancestor_record_offset":22328,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=10
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":419,"record_offset":22992,"parent_record_offset":22832,"ancestor_record_offset":22328,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":419,"record_offset":23204,"parent_record_offset":22832,"ancestor_record_offset":22328,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":419,"record_offset":24452,"parent_record_offset":24416,"ancestor_record_offset":22328,"inlinee":8422,"source_line":84,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":419,"record_offset":24748,"parent_record_offset":24416,"ancestor_record_offset":22328,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":419,"record_offset":24624,"parent_record_offset":24600,"ancestor_record_offset":22328,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_sheep::startup_sheep::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF156
// 首条位置: {"module":1321,"inlinee":61782}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1321,"record_offset":64396,"parent_record_offset":64360,"ancestor_record_offset":64360,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":64480,"parent_record_offset":64360,"ancestor_record_offset":64360,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1321,"record_offset":64444,"parent_record_offset":64420,"ancestor_record_offset":64360,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1321,"record_offset":64420,"parent_record_offset":64396,"ancestor_record_offset":64360,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":64520,"parent_record_offset":64480,"ancestor_record_offset":64360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_sheep.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_sheep.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_sheep.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_sheep::startup_sheep
// 信息 ID: 4c5c5807adf0cad1f72ff961
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6d8973ae3fbe2f7bf80022ed: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)> > @ 0xe7da54 -> 0x5977e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)> @ 0xf1d1e8 -> 0x5977e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_sheep::startup_sheep::closure$0
// 信息 ID: 5bca46394f57f4133cdb561d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_sheep::startup_sheep", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::startup_sheep::startup_sheep::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::mesh::Mesh,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_sheep::startup_sheep::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_sheep::startup_sheep::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >; module=1321, inline_record=64360; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$19::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >; module=1321, inline_record=64752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
