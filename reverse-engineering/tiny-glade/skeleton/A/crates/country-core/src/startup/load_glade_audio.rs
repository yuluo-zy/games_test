// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/load_glade_audio.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 39fc8c0c8404efe11807a0d3ce6d53ca7f057397de9b4fda4fef5b498baa70f3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_glade_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 387, 432, 1306, 1328, 1342
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::load_glade_audio
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":387,"record_offset":9612,"rva":5267216,"va":5373976336}
// 编译实例 RVAs: 0x505f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1226966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e82a47edff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373976657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1227169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e85f46edff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373976876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405af4a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t691713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8018e0a00","size":5,"soft_fail":false,"target_va":5374669984,"va":5373978266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e5f70
//     owner=unresolved_namespace; count=1
//     target_aliases=22 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1963963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8bbf71d00","size":5,"soft_fail":false,"target_va":5375942512,"va":5373978544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f21d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2015422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8bec01e00","size":5,"soft_fail":false,"target_va":5375992272,"va":5373976845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29616664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e818eac301","size":5,"soft_fail":false,"target_va":5403593360,"va":5373976691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29000282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e85a82ba01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373977857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3549280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e860283600","size":5,"soft_fail":false,"target_va":5377527120,"va":5373977835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3386260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e894ab3300","size":5,"soft_fail":false,"target_va":5377364336,"va":5373978071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t29003502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8ee8eba01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373976621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t699199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e83fab0a00","size":5,"soft_fail":false,"target_va":5374675760,"va":5373976556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::set_parameter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t696769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8c1a10a00","size":5,"soft_fail":false,"target_va":5374674256,"va":5373977482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_glade_audio::GladeAudio::load
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8fa0c0000","size":5,"soft_fail":false,"target_va":5373979776,"va":5373976449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_glade_audio::try_stop_and_load_new_stopped
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8dc060000","size":5,"soft_fail":false,"target_va":5373978736,"va":5373976975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8625355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8cb9c8300","size":5,"soft_fail":false,"target_va":5382602288,"va":5373976928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8626127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8cf9f8300","size":5,"soft_fail":false,"target_va":5382602640,"va":5373976508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3463334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373976336,"raw_hex":"e8a6d83400","size":5,"soft_fail":false,"target_va":5377439776,"va":5373976437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":387,"record_offset":9896,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":12300,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":387,"record_offset":12112,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":11472,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":21881,"source_line":301,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_parameter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":12996,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":21885,"source_line":294,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":12184,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":21884,"source_line":387,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":387,"record_offset":9784,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":21871,"source_line":200,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_glade_audio::stop_and_play_new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_glade_audio.rs
//     evidence={"module":387,"record_offset":9952,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":21877,"source_line":49,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":9612,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":10560,"parent_record_offset":10536,"ancestor_record_offset":9612,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":11040,"parent_record_offset":11016,"ancestor_record_offset":9612,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":10512,"parent_record_offset":10476,"ancestor_record_offset":9612,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":11016,"parent_record_offset":10960,"ancestor_record_offset":9612,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10988,"parent_record_offset":10960,"ancestor_record_offset":9612,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10476,"parent_record_offset":10448,"ancestor_record_offset":9612,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10448,"parent_record_offset":10420,"ancestor_record_offset":9612,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10632,"parent_record_offset":10448,"ancestor_record_offset":9612,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":9612,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10384,"parent_record_offset":10356,"ancestor_record_offset":9612,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10356,"parent_record_offset":10328,"ancestor_record_offset":9612,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10300,"parent_record_offset":10272,"ancestor_record_offset":9612,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10328,"parent_record_offset":10300,"ancestor_record_offset":9612,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10272,"parent_record_offset":10244,"ancestor_record_offset":9612,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":387,"record_offset":10244,"parent_record_offset":10144,"ancestor_record_offset":9612,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":387,"record_offset":10216,"parent_record_offset":10144,"ancestor_record_offset":9612,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":387,"record_offset":10420,"parent_record_offset":10384,"ancestor_record_offset":9612,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":387,"record_offset":10752,"parent_record_offset":10144,"ancestor_record_offset":9612,"inlinee":21883,"source_line":647,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":387,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":9612,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":387,"record_offset":10696,"parent_record_offset":10672,"ancestor_record_offset":9612,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":10068,"parent_record_offset":10028,"ancestor_record_offset":9612,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":387,"record_offset":10672,"parent_record_offset":10384,"ancestor_record_offset":9612,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":10824,"parent_record_offset":10784,"ancestor_record_offset":9612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":10588,"parent_record_offset":10560,"ancestor_record_offset":9612,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":387,"record_offset":10100,"parent_record_offset":10028,"ancestor_record_offset":9612,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":10144,"parent_record_offset":9952,"ancestor_record_offset":9612,"inlinee":21881,"source_line":301,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":11296,"parent_record_offset":9952,"ancestor_record_offset":9612,"inlinee":21884,"source_line":387,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":10028,"parent_record_offset":9952,"ancestor_record_offset":9612,"inlinee":21879,"source_line":426,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":387,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":9612,"inlinee":21875,"source_line":44,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":387,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":9612,"inlinee":21873,"source_line":153,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":9860,"parent_record_offset":9808,"ancestor_record_offset":9612,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::stop_and_play_new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5575
// 首条位置: {"module":387,"inlinee":21877}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":10856,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":387,"record_offset":10784,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":10144,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":21881,"source_line":301,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":11296,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":21884,"source_line":387,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":10028,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":21879,"source_line":426,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":9952,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":10560,"parent_record_offset":10536,"ancestor_record_offset":9952,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":11040,"parent_record_offset":11016,"ancestor_record_offset":9952,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":10512,"parent_record_offset":10476,"ancestor_record_offset":9952,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":11016,"parent_record_offset":10960,"ancestor_record_offset":9952,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10988,"parent_record_offset":10960,"ancestor_record_offset":9952,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10476,"parent_record_offset":10448,"ancestor_record_offset":9952,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10448,"parent_record_offset":10420,"ancestor_record_offset":9952,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10632,"parent_record_offset":10448,"ancestor_record_offset":9952,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":9952,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10384,"parent_record_offset":10356,"ancestor_record_offset":9952,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10356,"parent_record_offset":10328,"ancestor_record_offset":9952,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10300,"parent_record_offset":10272,"ancestor_record_offset":9952,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10328,"parent_record_offset":10300,"ancestor_record_offset":9952,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":10272,"parent_record_offset":10244,"ancestor_record_offset":9952,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":387,"record_offset":10244,"parent_record_offset":10144,"ancestor_record_offset":9952,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":387,"record_offset":10216,"parent_record_offset":10144,"ancestor_record_offset":9952,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":387,"record_offset":10420,"parent_record_offset":10384,"ancestor_record_offset":9952,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":387,"record_offset":10752,"parent_record_offset":10144,"ancestor_record_offset":9952,"inlinee":21883,"source_line":647,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":387,"record_offset":10696,"parent_record_offset":10672,"ancestor_record_offset":9952,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":10068,"parent_record_offset":10028,"ancestor_record_offset":9952,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":387,"record_offset":10672,"parent_record_offset":10384,"ancestor_record_offset":9952,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":10824,"parent_record_offset":10784,"ancestor_record_offset":9952,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":10588,"parent_record_offset":10560,"ancestor_record_offset":9952,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":387,"record_offset":10100,"parent_record_offset":10028,"ancestor_record_offset":9952,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::try_stop_and_load_new_stopped
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":387,"record_offset":14300,"rva":5269616,"va":5373978736}
// 编译实例 RVAs: 0x506870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1229248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e8403eedff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373978939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1229414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e89a3dedff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373979121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405af4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t690545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e871890a00","size":5,"soft_fail":false,"target_va":5374669984,"va":5373979434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f21d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2013174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e8f6b71e00","size":5,"soft_fail":false,"target_va":5375992272,"va":5373979093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29614380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e82ce1c301","size":5,"soft_fail":false,"target_va":5403593360,"va":5373978975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28998837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e8b57cba01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373979302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3547826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e8b2223600","size":5,"soft_fail":false,"target_va":5377527120,"va":5373979289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3384849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e811a63300","size":5,"soft_fail":false,"target_va":5377364336,"va":5373979482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29001269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e83586ba01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373978854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t696960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e880a20a00","size":5,"soft_fail":false,"target_va":5374675760,"va":5373978795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8623754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373978736,"raw_hex":"e88a968300","size":5,"soft_fail":false,"target_va":5382602640,"va":5373978881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":15308,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":387,"record_offset":15116,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":14468,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":21881,"source_line":301,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":387,"record_offset":15192,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":21879,"source_line":426,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":14868,"parent_record_offset":14844,"ancestor_record_offset":14300,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":14892,"parent_record_offset":14868,"ancestor_record_offset":14300,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":15520,"parent_record_offset":15496,"ancestor_record_offset":14300,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":14844,"parent_record_offset":14808,"ancestor_record_offset":14300,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":15496,"parent_record_offset":15440,"ancestor_record_offset":14300,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":15468,"parent_record_offset":15440,"ancestor_record_offset":14300,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":14808,"parent_record_offset":14780,"ancestor_record_offset":14300,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":14780,"parent_record_offset":14752,"ancestor_record_offset":14300,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":14964,"parent_record_offset":14780,"ancestor_record_offset":14300,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":15440,"parent_record_offset":15412,"ancestor_record_offset":14300,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":14716,"parent_record_offset":14688,"ancestor_record_offset":14300,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":14688,"parent_record_offset":14660,"ancestor_record_offset":14300,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":14632,"parent_record_offset":14604,"ancestor_record_offset":14300,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":14660,"parent_record_offset":14632,"ancestor_record_offset":14300,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":14604,"parent_record_offset":14576,"ancestor_record_offset":14300,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":387,"record_offset":14576,"parent_record_offset":14468,"ancestor_record_offset":14300,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":387,"record_offset":14548,"parent_record_offset":14468,"ancestor_record_offset":14300,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":387,"record_offset":14752,"parent_record_offset":14716,"ancestor_record_offset":14300,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":387,"record_offset":15084,"parent_record_offset":14468,"ancestor_record_offset":14300,"inlinee":21883,"source_line":647,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":387,"record_offset":15028,"parent_record_offset":15004,"ancestor_record_offset":14300,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":14516,"parent_record_offset":14468,"ancestor_record_offset":14300,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":387,"record_offset":15004,"parent_record_offset":14716,"ancestor_record_offset":14300,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":15160,"parent_record_offset":15116,"ancestor_record_offset":14300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":14920,"parent_record_offset":14892,"ancestor_record_offset":14300,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":387,"record_offset":15260,"parent_record_offset":15192,"ancestor_record_offset":14300,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6191
// 首条位置: {"module":1342,"inlinee":24977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6192
// 首条位置: {"module":1306,"inlinee":24978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1306,"record_offset":302272,"parent_record_offset":302240,"ancestor_record_offset":302240,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1306,"record_offset":302296,"parent_record_offset":302272,"ancestor_record_offset":302240,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1306,"record_offset":302320,"parent_record_offset":302296,"ancestor_record_offset":302240,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1306,"record_offset":302344,"parent_record_offset":302320,"ancestor_record_offset":302240,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFE1
// 首条位置: {"module":1306,"inlinee":61409}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":302052,"parent_record_offset":302020,"ancestor_record_offset":302020,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1306,"record_offset":302212,"parent_record_offset":302084,"ancestor_record_offset":302020,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1306,"record_offset":302296,"parent_record_offset":302272,"ancestor_record_offset":302020,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1306,"record_offset":302320,"parent_record_offset":302296,"ancestor_record_offset":302020,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1306,"record_offset":302344,"parent_record_offset":302320,"ancestor_record_offset":302020,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1306,"record_offset":302272,"parent_record_offset":302240,"ancestor_record_offset":302020,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_glade_audio.rs
//     evidence={"module":1306,"record_offset":302240,"parent_record_offset":302084,"ancestor_record_offset":302020,"inlinee":24978,"source_line":69,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":302128,"parent_record_offset":302084,"ancestor_record_offset":302020,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":302084,"parent_record_offset":302052,"ancestor_record_offset":302020,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":302152,"parent_record_offset":302128,"ancestor_record_offset":302020,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":302176,"parent_record_offset":302152,"ancestor_record_offset":302020,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55C2
// 首条位置: {"module":432,"inlinee":21954}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFDF
// 首条位置: {"module":1306,"inlinee":61407}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1306,"record_offset":302640,"parent_record_offset":301224,"ancestor_record_offset":301224,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1306,"record_offset":304304,"parent_record_offset":301224,"ancestor_record_offset":301224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":304208,"parent_record_offset":301224,"ancestor_record_offset":301224,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1306,"record_offset":304500,"parent_record_offset":301224,"ancestor_record_offset":301224,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":301292,"parent_record_offset":301224,"ancestor_record_offset":301224,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":302668,"parent_record_offset":301224,"ancestor_record_offset":301224,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1306,"record_offset":304448,"parent_record_offset":304424,"ancestor_record_offset":301224,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1306,"record_offset":304424,"parent_record_offset":304400,"ancestor_record_offset":301224,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1306,"record_offset":304824,"parent_record_offset":304796,"ancestor_record_offset":301224,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1306,"record_offset":304400,"parent_record_offset":304376,"ancestor_record_offset":301224,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1306,"record_offset":302212,"parent_record_offset":302084,"ancestor_record_offset":301224,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1306,"record_offset":302296,"parent_record_offset":302272,"ancestor_record_offset":301224,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1306,"record_offset":304328,"parent_record_offset":304304,"ancestor_record_offset":301224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1306,"record_offset":302320,"parent_record_offset":302296,"ancestor_record_offset":301224,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1306,"record_offset":302344,"parent_record_offset":302320,"ancestor_record_offset":301224,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1306,"record_offset":302272,"parent_record_offset":302240,"ancestor_record_offset":301224,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_glade_audio.rs
//     evidence={"module":1306,"record_offset":302240,"parent_record_offset":302084,"ancestor_record_offset":301224,"inlinee":24978,"source_line":69,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_glade_audio.rs
//     evidence={"module":1306,"record_offset":302020,"parent_record_offset":301988,"ancestor_record_offset":301224,"inlinee":61409,"source_line":69,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1306,"record_offset":304572,"parent_record_offset":304548,"ancestor_record_offset":301224,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":304548,"parent_record_offset":304524,"ancestor_record_offset":301224,"inlinee":6868,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":301988,"parent_record_offset":301348,"ancestor_record_offset":301224,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1306,"record_offset":303132,"parent_record_offset":303108,"ancestor_record_offset":301224,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":301596,"parent_record_offset":301444,"ancestor_record_offset":301224,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":302752,"parent_record_offset":302708,"ancestor_record_offset":301224,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":301444,"parent_record_offset":301348,"ancestor_record_offset":301224,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":301484,"parent_record_offset":301444,"ancestor_record_offset":301224,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":301348,"parent_record_offset":301292,"ancestor_record_offset":301224,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":302708,"parent_record_offset":302668,"ancestor_record_offset":301224,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":302084,"parent_record_offset":302052,"ancestor_record_offset":301224,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1306,"record_offset":302052,"parent_record_offset":302020,"ancestor_record_offset":301224,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":301620,"parent_record_offset":301596,"ancestor_record_offset":301224,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":301516,"parent_record_offset":301484,"ancestor_record_offset":301224,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":301644,"parent_record_offset":301620,"ancestor_record_offset":301224,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":301548,"parent_record_offset":301516,"ancestor_record_offset":301224,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFE2
// 首条位置: {"module":1306,"inlinee":61410}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1306,"record_offset":306924,"parent_record_offset":306112,"ancestor_record_offset":306112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1306,"record_offset":306176,"parent_record_offset":306112,"ancestor_record_offset":306112,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":306232,"parent_record_offset":306112,"ancestor_record_offset":306112,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1306,"record_offset":307100,"parent_record_offset":307076,"ancestor_record_offset":306112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1306,"record_offset":307076,"parent_record_offset":307008,"ancestor_record_offset":306112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1306,"record_offset":307044,"parent_record_offset":307008,"ancestor_record_offset":306112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1306,"record_offset":307008,"parent_record_offset":306980,"ancestor_record_offset":306112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1306,"record_offset":306952,"parent_record_offset":306924,"ancestor_record_offset":306112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1306,"record_offset":306888,"parent_record_offset":306272,"ancestor_record_offset":306112,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":306484,"parent_record_offset":306344,"ancestor_record_offset":306112,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":306344,"parent_record_offset":306272,"ancestor_record_offset":306112,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":306380,"parent_record_offset":306344,"ancestor_record_offset":306112,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1306,"record_offset":306272,"parent_record_offset":306232,"ancestor_record_offset":306112,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":306508,"parent_record_offset":306484,"ancestor_record_offset":306112,"inlinee":11484,"source_line":642,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":306408,"parent_record_offset":306380,"ancestor_record_offset":306112,"inlinee":11486,"source_line":637,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":306532,"parent_record_offset":306508,"ancestor_record_offset":306112,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1306,"record_offset":306436,"parent_record_offset":306408,"ancestor_record_offset":306112,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/01306.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::impl$0::default
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55B5
// 首条位置: {"module":387,"record_offset":15784,"rva":5270384,"va":5373979504}
// 编译实例 RVAs: 0x506b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1229853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979504,"raw_hex":"e8e33bedff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373979544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1230052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979504,"raw_hex":"e81c3bedff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373979759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28998439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979504,"raw_hex":"e8277bba01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373979700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":387,"record_offset":15928,"parent_record_offset":15784,"ancestor_record_offset":15784,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":16968,"parent_record_offset":15784,"ancestor_record_offset":15784,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":16236,"parent_record_offset":16212,"ancestor_record_offset":15784,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":16260,"parent_record_offset":16236,"ancestor_record_offset":15784,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":17088,"parent_record_offset":17064,"ancestor_record_offset":15784,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":16212,"parent_record_offset":16184,"ancestor_record_offset":15784,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":17064,"parent_record_offset":17040,"ancestor_record_offset":15784,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":16184,"parent_record_offset":16156,"ancestor_record_offset":15784,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":16156,"parent_record_offset":16128,"ancestor_record_offset":15784,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":16328,"parent_record_offset":16156,"ancestor_record_offset":15784,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":17040,"parent_record_offset":17016,"ancestor_record_offset":15784,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":16096,"parent_record_offset":16068,"ancestor_record_offset":15784,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":16068,"parent_record_offset":16040,"ancestor_record_offset":15784,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":16012,"parent_record_offset":15984,"ancestor_record_offset":15784,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":16040,"parent_record_offset":16012,"ancestor_record_offset":15784,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":387,"record_offset":15984,"parent_record_offset":15956,"ancestor_record_offset":15784,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":387,"record_offset":15956,"parent_record_offset":15928,"ancestor_record_offset":15784,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":387,"record_offset":16128,"parent_record_offset":16096,"ancestor_record_offset":15784,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":387,"record_offset":16388,"parent_record_offset":16364,"ancestor_record_offset":15784,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":387,"record_offset":16364,"parent_record_offset":16096,"ancestor_record_offset":15784,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":16992,"parent_record_offset":16968,"ancestor_record_offset":15784,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":16284,"parent_record_offset":16260,"ancestor_record_offset":15784,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::GladeAudio::try_load
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x557F
// 首条位置: {"module":387,"inlinee":21887}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":387,"record_offset":17440,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":387,"record_offset":17640,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":17696,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":387,"record_offset":18972,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":387,"record_offset":17668,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":387,"record_offset":17356,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":387,"record_offset":18168,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":17608,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":17468,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":17288,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":17912,"parent_record_offset":17856,"ancestor_record_offset":17288,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":387,"record_offset":19260,"parent_record_offset":19232,"ancestor_record_offset":17288,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":17884,"parent_record_offset":17856,"ancestor_record_offset":17288,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":17856,"parent_record_offset":17828,"ancestor_record_offset":17288,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":17288,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":387,"record_offset":17380,"parent_record_offset":17356,"ancestor_record_offset":17288,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":387,"record_offset":17404,"parent_record_offset":17380,"ancestor_record_offset":17288,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":387,"record_offset":17516,"parent_record_offset":17492,"ancestor_record_offset":17288,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":387,"record_offset":17564,"parent_record_offset":17540,"ancestor_record_offset":17288,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":387,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":17288,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":17492,"parent_record_offset":17468,"ancestor_record_offset":17288,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":17288,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":18200,"parent_record_offset":18168,"ancestor_record_offset":17288,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_audio::GladeAudio::load
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":387,"record_offset":17140,"rva":5270656,"va":5373979776}
// 编译实例 RVAs: 0x506c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1665901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979776,"raw_hex":"e89394e6ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5373980104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1230280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979776,"raw_hex":"e8383aedff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373979987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140506b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979776,"raw_hex":"e853fcffff","size":5,"soft_fail":false,"target_va":5373979504,"va":5373980440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1191215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979776,"raw_hex":"e82f2d1200","size":5,"soft_fail":false,"target_va":5375171056,"va":5373979836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d42e30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8634437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979776,"raw_hex":"e845c08300","size":5,"soft_fail":false,"target_va":5382614576,"va":5373980134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3373808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979776,"raw_hex":"e8f07a3300","size":5,"soft_fail":false,"target_va":5377353776,"va":5373979963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3425559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979776,"raw_hex":"e817453400","size":5,"soft_fail":false,"target_va":5377405616,"va":5373980052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3459902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373979776,"raw_hex":"e83ecb3400","size":5,"soft_fail":false,"target_va":5377439776,"va":5373979869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":387,"record_offset":21348,"parent_record_offset":17140,"ancestor_record_offset":17140,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_glade_audio::GladeAudio::try_load
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_glade_audio.rs
//     evidence={"module":387,"record_offset":17288,"parent_record_offset":17140,"ancestor_record_offset":17140,"inlinee":21887,"source_line":87,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":17140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":387,"record_offset":17912,"parent_record_offset":17856,"ancestor_record_offset":17140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":387,"record_offset":19260,"parent_record_offset":19232,"ancestor_record_offset":17140,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":17884,"parent_record_offset":17856,"ancestor_record_offset":17140,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":387,"record_offset":17856,"parent_record_offset":17828,"ancestor_record_offset":17140,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":387,"record_offset":17440,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":387,"record_offset":17640,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":387,"record_offset":17696,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":387,"record_offset":18972,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":387,"record_offset":17380,"parent_record_offset":17356,"ancestor_record_offset":17140,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":387,"record_offset":17404,"parent_record_offset":17380,"ancestor_record_offset":17140,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":387,"record_offset":17668,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":387,"record_offset":17356,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":387,"record_offset":17516,"parent_record_offset":17492,"ancestor_record_offset":17140,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":387,"record_offset":17564,"parent_record_offset":17540,"ancestor_record_offset":17140,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":387,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":17140,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":387,"record_offset":18168,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":17608,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":17492,"parent_record_offset":17468,"ancestor_record_offset":17140,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":17468,"parent_record_offset":17288,"ancestor_record_offset":17140,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00387.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":387,"record_offset":18200,"parent_record_offset":18168,"ancestor_record_offset":17140,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00387.jsonl.gz"}
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
// 完整结构及原始类型记录: load_glade_audio.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_glade_audio::GladeAudio
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_glade_audio::GladeAudio>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_glade_audio::GladeAudio"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::startup::load_glade_audio::GladeAudio>", "constructor": "core::option::Option", "arguments": ["country_core::startup::load_glade_audio::GladeAudio"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::startup::load_glade_audio::GladeAudio> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::startup::load_glade_audio::GladeAudio>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_str<enum2$<core::option::Option<country_core::startup::load_glade_audio::GladeAudio> > >", "constructor": "serde_json::de::from_str", "arguments": ["enum2$<core::option::Option<country_core::startup::load_glade_audio::GladeAudio> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::startup::load_glade_audio::GladeAudio,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["country_core::startup::load_glade_audio::GladeAudio", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::startup::load_glade_audio::GladeAudio,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::startup::load_glade_audio::GladeAudio,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::startup::load_glade_audio::GladeAudio,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::startup::load_glade_audio::GladeAudio,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$16::deserialize<country_core::startup::load_glade_audio::GladeAudio,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >", "constructor": "serde::de::impls::impl$16::deserialize", "arguments": ["country_core::startup::load_glade_audio::GladeAudio", "ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_glade_audio.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_glade_audio.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(ref_mut$<bevy_ecs::world::World>)
// 候选参数类型: ["ref_mut$<bevy_ecs::world::World>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>", "bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>,bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>", "bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::GladeAudio::load
// 信息 ID: 8c63b630ebce17cba7e69728
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_audio::load_glade_audio @ 0x505f81 -> 0x506c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::GladeAudio::try_load
// 信息 ID: 22dcaf95e2a6215bb6b024e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_glade_audio::GladeAudio::load; module=387, inline_record=17288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::_::impl$0::deserialize
// 信息 ID: 04d63dae8ea083b53af76e11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impls::impl$16::deserialize<country_core::startup::load_glade_audio::GladeAudio,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=1342, inline_record=7084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 7ef95c1560f21aee09bc227a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::load_glade_audio::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=302240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 1b9e8b347d34df64371e8b11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::load_glade_audio::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=302020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 35874ab63ae6026c260116cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::startup::load_glade_audio::_::impl$0::deserialize::__Visitor>; module=432, inline_record=90308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: df8e7400b9ecd75165344dbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::load_glade_audio::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=301224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: a5a61fd255bb0e886ce97ada
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::load_glade_audio::_::impl$0::deserialize::__Visitor>; module=1306, inline_record=306112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::impl$0::default
// 信息 ID: 96c47d8eddc0da0a3ae01ebb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_audio::GladeAudio::load @ 0x506f18 -> 0x506b70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::music_audio::music_audio_init_system; module=1328, inline_record=21436; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::load_glade_audio
// 信息 ID: 88cb0ee5f4599cf5bd92e67f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5f057b0cd510a3a61c129b07: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)> @ 0x5c9071 -> 0x505f10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)> > @ 0xe5d8b3 -> 0x505f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::stop_and_play_new
// 信息 ID: 296add4e881680aa8fcbfeb9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_glade_audio::load_glade_audio; module=387, inline_record=9952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_audio::try_stop_and_load_new_stopped
// 信息 ID: 77c8e0ab20de5c229815fa8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_audio::load_glade_audio @ 0x50618f -> 0x506870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
