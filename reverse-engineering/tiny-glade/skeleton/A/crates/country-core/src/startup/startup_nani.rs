// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/startup_nani.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 95f4966116035cb6c5cbcb486332712b175e95acc5b067d4e3f9498a6423126d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_nani.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2829
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
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"PDB_registry_path_only","version":"1.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::startup_nani
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2829,"record_offset":4136,"rva":25254368,"va":5393963488}
// 编译实例 RVAs: 0x18159e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25234685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e803f37efe","size":5,"soft_fail":false,"target_va":5368732256,"va":5393966936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21336431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e8916ebafe","size":5,"soft_fail":false,"target_va":5372630832,"va":5393967258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21216218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e82644bcfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393965925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21216990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e82241bcfe","size":5,"soft_fail":false,"target_va":5372749744,"va":5393966729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=17
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18232441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e887cbe9fe","size":5,"soft_fail":false,"target_va":5375731504,"va":5393963940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ea5b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18005674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e85641edfe","size":5,"soft_fail":false,"target_va":5375960496,"va":5393966165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17679789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e8533af2fe","size":5,"soft_fail":false,"target_va":5376286288,"va":5393966072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d27cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11463223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e8c91551ff","size":5,"soft_fail":false,"target_va":5382503600,"va":5393966818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d27f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11462558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e8621851ff","size":5,"soft_fail":false,"target_va":5382504240,"va":5393966793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8993770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e8ea3b8900","size":5,"soft_fail":false,"target_va":5402958656,"va":5393964881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: startup_nani.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9627474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e852e79200","size":5,"soft_fail":false,"target_va":5403593360,"va":5393965881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9011392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e8c0808900","size":5,"soft_fail":false,"target_va":5402978144,"va":5393966747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16439763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e82d2605ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5393966878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9587846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e87ab36dff","size":5,"soft_fail":false,"target_va":5384377680,"va":5393965521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9012630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e896858900","size":5,"soft_fail":false,"target_va":5402978944,"va":5393966309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9013617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e871898900","size":5,"soft_fail":false,"target_va":5402980128,"va":5393966506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18272992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e8202de9fe","size":5,"soft_fail":false,"target_va":5375692416,"va":5393965403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19606491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393963488,"raw_hex":"e825d4d4fe","size":5,"soft_fail":false,"target_va":5374358896,"va":5393965382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%r10","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5393963488,"raw_hex":"41ffd2","size":3,"soft_fail":false,"va":5393965861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":5852,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2829,"record_offset":5384,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2829,"record_offset":4528,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":4780,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":6164,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":2829,"record_offset":4408,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::assign_subset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":2829,"record_offset":5764,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":26136,"source_line":387,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::update_mesh_atlas_slices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":2829,"record_offset":5440,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":26137,"source_line":391,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::create_world_subset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":2829,"record_offset":4872,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":90347,"source_line":159,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_nani::runtime_check_limits
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":2829,"record_offset":4312,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":90346,"source_line":136,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":2829,"record_offset":4464,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":23004,"source_line":221,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":4136,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":4136,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":4136,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":6028,"parent_record_offset":5936,"ancestor_record_offset":4136,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":5172,"parent_record_offset":5088,"ancestor_record_offset":4136,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$49::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2829,"record_offset":5032,"parent_record_offset":4872,"ancestor_record_offset":4136,"inlinee":8413,"source_line":2033,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2829,"record_offset":5088,"parent_record_offset":5060,"ancestor_record_offset":4136,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":6240,"parent_record_offset":6212,"ancestor_record_offset":4136,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":5936,"parent_record_offset":5908,"ancestor_record_offset":4136,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":5908,"parent_record_offset":5880,"ancestor_record_offset":4136,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":6116,"parent_record_offset":5908,"ancestor_record_offset":4136,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":4136,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2829,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":4136,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":2829,"record_offset":5880,"parent_record_offset":5852,"ancestor_record_offset":4136,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2829,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":4136,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2829,"record_offset":5972,"parent_record_offset":5936,"ancestor_record_offset":4136,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2829,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":4136,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2829,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":4136,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2829,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":4136,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2829,"record_offset":4584,"parent_record_offset":4556,"ancestor_record_offset":4136,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2829,"record_offset":4708,"parent_record_offset":4612,"ancestor_record_offset":4136,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2829,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":4136,"inlinee":13759,"source_line":49,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2829,"record_offset":4732,"parent_record_offset":4708,"ancestor_record_offset":4136,"inlinee":13764,"source_line":53,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2829,"record_offset":4612,"parent_record_offset":4584,"ancestor_record_offset":4136,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2829,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4136,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2829,"record_offset":6268,"parent_record_offset":6240,"ancestor_record_offset":4136,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":4136,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":5060,"parent_record_offset":4872,"ancestor_record_offset":4136,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2829,"record_offset":5624,"parent_record_offset":5520,"ancestor_record_offset":4136,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2829,"record_offset":5668,"parent_record_offset":5440,"ancestor_record_offset":4136,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":5568,"parent_record_offset":5520,"ancestor_record_offset":4136,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::gpu_world_subset_factory
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":2829,"record_offset":4972,"parent_record_offset":4872,"ancestor_record_offset":4136,"inlinee":26132,"source_line":195,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_instances
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":2829,"record_offset":5000,"parent_record_offset":4872,"ancestor_record_offset":4136,"inlinee":26120,"source_line":144,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":2829,"record_offset":5484,"parent_record_offset":5440,"ancestor_record_offset":4136,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":2829,"record_offset":5520,"parent_record_offset":5484,"ancestor_record_offset":4136,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_nani::startup_nani::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_nani.rs
//     evidence={"module":2829,"record_offset":4812,"parent_record_offset":4780,"ancestor_record_offset":4136,"inlinee":90349,"source_line":99,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":2829,"record_offset":4348,"parent_record_offset":4312,"ancestor_record_offset":4136,"inlinee":23004,"source_line":221,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":2829,"record_offset":5596,"parent_record_offset":5520,"ancestor_record_offset":4136,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":2829,"record_offset":4944,"parent_record_offset":4872,"ancestor_record_offset":4136,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::startup_nani::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160ED
// 首条位置: {"module":2829,"inlinee":90349}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2829,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":4812,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::runtime_check_limits
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160EA
// 首条位置: {"module":2829,"inlinee":90346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":2829,"record_offset":4348,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":23004,"source_line":221,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":4312,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_nani::create_world_subset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160EB
// 首条位置: {"module":2829,"inlinee":90347}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$49::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2829,"record_offset":5032,"parent_record_offset":4872,"ancestor_record_offset":4872,"inlinee":8413,"source_line":2033,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":5060,"parent_record_offset":4872,"ancestor_record_offset":4872,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::gpu_world_subset_factory
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":2829,"record_offset":4972,"parent_record_offset":4872,"ancestor_record_offset":4872,"inlinee":26132,"source_line":195,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_instances
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":2829,"record_offset":5000,"parent_record_offset":4872,"ancestor_record_offset":4872,"inlinee":26120,"source_line":144,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":2829,"record_offset":4944,"parent_record_offset":4872,"ancestor_record_offset":4872,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":4872,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":5172,"parent_record_offset":5088,"ancestor_record_offset":4872,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2829,"record_offset":5088,"parent_record_offset":5060,"ancestor_record_offset":4872,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2829,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":4872,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2829,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4872,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/02829.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_nani.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_nani.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_nani.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::create_world_subset
// 信息 ID: a34eca392319a11bd88476b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=2829, inline_record=4872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::runtime_check_limits
// 信息 ID: 2a25185434a8755fce6c15b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=2829, inline_record=4312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::startup_nani
// 信息 ID: 65f73e7561709c498e7e2462
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2a3f8068f7c7fbe09edb845b: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)> > @ 0xe5712d -> 0x18159e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)> @ 0xf1574a -> 0x18159e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_nani::startup_nani::closure$0
// 信息 ID: a484e3e09f19b922861c57fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_nani::startup_nani", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=2829, inline_record=4812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
