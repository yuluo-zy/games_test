// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/spawn_brush_previews.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2e813ca14b18cbca3a9fddf5c72be4791e0fb02bece0e88153ccef8ca701a2a3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spawn_brush_previews.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 486
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::spawn_brush_previews::spawn_brush_previews
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":11856,"rva":6913232,"va":5375622352}
// 编译实例 RVAs: 0x697cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2873004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e85429d4ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375622695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2877144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e82819d4ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375626851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b61b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-928407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e869d5f1ff","size":5,"soft_fail":false,"target_va":5374697904,"va":5375626306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b6e80
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-921311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e821f1f1ff","size":5,"soft_fail":false,"target_va":5374701184,"va":5375622490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t109037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e8eda90100","size":5,"soft_fail":false,"target_va":5375731504,"va":5375622462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t357715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e853750500","size":5,"soft_fail":false,"target_va":5375988432,"va":5375630712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc64a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6475300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e824ce6200","size":5,"soft_fail":false,"target_va":5382104224,"va":5375628919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f1f1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8937539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e843608800","size":5,"soft_fail":false,"target_va":5384565232,"va":5375627688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27966733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e80dbdaa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375626622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t27348507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e81b4ea101","size":5,"soft_fail":false,"target_va":5402978144,"va":5375629632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1903620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e8040c1d00","size":5,"soft_fail":false,"target_va":5377529888,"va":5375626263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1897501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e81df41c00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375629614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t1732387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e8236f1a00","size":5,"soft_fail":false,"target_va":5377364336,"va":5375631944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t8756264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e8289c8500","size":5,"soft_fail":false,"target_va":5384383280,"va":5375627011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27348570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e85a4ea101","size":5,"soft_fail":false,"target_va":5402978880,"va":5375630305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t27350519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e8f755a101","size":5,"soft_fail":false,"target_va":5402980128,"va":5375629604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t-1056833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e8bfdfefff","size":5,"soft_fail":false,"target_va":5374566080,"va":5375622908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t65530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e8faff0000","size":5,"soft_fail":false,"target_va":5375692416,"va":5375626881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::add_color_self
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t8945092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375622352,"raw_hex":"e8c47d8800","size":5,"soft_fail":false,"target_va":5384567760,"va":5375622663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":486,"record_offset":18128,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":486,"record_offset":17856,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":486,"record_offset":19144,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":486,"record_offset":18940,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":486,"record_offset":12096,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":486,"record_offset":12040,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":486,"record_offset":12172,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::from_translation_scale
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":486,"record_offset":21580,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":24181,"source_line":49,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":486,"record_offset":19180,"parent_record_offset":11856,"ancestor_record_offset":11856,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":11856,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":11856,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":19096,"parent_record_offset":19072,"ancestor_record_offset":11856,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":12568,"parent_record_offset":12540,"ancestor_record_offset":11856,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":19072,"parent_record_offset":19016,"ancestor_record_offset":11856,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":486,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":11856,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":19044,"parent_record_offset":19016,"ancestor_record_offset":11856,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":12540,"parent_record_offset":12512,"ancestor_record_offset":11856,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":12512,"parent_record_offset":12484,"ancestor_record_offset":11856,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":12688,"parent_record_offset":12512,"ancestor_record_offset":11856,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":19016,"parent_record_offset":18988,"ancestor_record_offset":11856,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":12448,"parent_record_offset":12420,"ancestor_record_offset":11856,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":12420,"parent_record_offset":12392,"ancestor_record_offset":11856,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":12364,"parent_record_offset":12336,"ancestor_record_offset":11856,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":12392,"parent_record_offset":12364,"ancestor_record_offset":11856,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":12336,"parent_record_offset":12308,"ancestor_record_offset":11856,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":486,"record_offset":12308,"parent_record_offset":12280,"ancestor_record_offset":11856,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":486,"record_offset":12280,"parent_record_offset":12236,"ancestor_record_offset":11856,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":486,"record_offset":12064,"parent_record_offset":12040,"ancestor_record_offset":11856,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":486,"record_offset":12484,"parent_record_offset":12448,"ancestor_record_offset":11856,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":486,"record_offset":17944,"parent_record_offset":17880,"ancestor_record_offset":11856,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":486,"record_offset":12748,"parent_record_offset":12724,"ancestor_record_offset":11856,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":486,"record_offset":17916,"parent_record_offset":17880,"ancestor_record_offset":11856,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":486,"record_offset":17880,"parent_record_offset":17856,"ancestor_record_offset":11856,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":486,"record_offset":18152,"parent_record_offset":18128,"ancestor_record_offset":11856,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":486,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":11856,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":486,"record_offset":12724,"parent_record_offset":12448,"ancestor_record_offset":11856,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":486,"record_offset":18056,"parent_record_offset":17968,"ancestor_record_offset":11856,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":486,"record_offset":12140,"parent_record_offset":12096,"ancestor_record_offset":11856,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":486,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":11856,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":486,"record_offset":12644,"parent_record_offset":12616,"ancestor_record_offset":11856,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":486,"record_offset":17968,"parent_record_offset":17944,"ancestor_record_offset":11856,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":486,"record_offset":18000,"parent_record_offset":17968,"ancestor_record_offset":11856,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":486,"record_offset":12236,"parent_record_offset":12172,"ancestor_record_offset":11856,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":486,"record_offset":12208,"parent_record_offset":12172,"ancestor_record_offset":11856,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00486.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::spawn_brush_previews::spawn_brush_previews
// 信息 ID: 9559d95918d249fefc7727fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cc1c15e21a217d5b987dc287: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > @ 0xe6a060 -> 0x697cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> @ 0xf15a41 -> 0x697cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
