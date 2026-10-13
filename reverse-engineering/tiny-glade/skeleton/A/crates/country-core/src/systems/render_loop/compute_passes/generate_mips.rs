// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b9b43a6fdba736a2999ff2aba4a2b18974dda8b0dffaa3b43df76b0e46c71601
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: generate_mips.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 72, 424
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"PDB_registry_path_only","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":72,"record_offset":232,"rva":902256,"va":5369611376}
// 编译实例 RVAs: 0xdc470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140064910
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-490979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e81d82f8ff","size":5,"soft_fail":false,"target_va":5369121040,"va":5369612014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400da5a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e815e0ffff","size":5,"soft_fail":false,"target_va":5369603488,"va":5369611654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a6e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t829448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e808a80c00","size":5,"soft_fail":false,"target_va":5370441296,"va":5369611843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391980
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2838222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8ce4e2b00","size":5,"soft_fail":false,"target_va":5372451200,"va":5369612973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3137121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e861de2f00","size":5,"soft_fail":false,"target_va":5372749696,"va":5369612570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3137970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8b2e12f00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369611737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33981313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e881830602","size":5,"soft_fail":false,"target_va":5403593360,"va":5369612042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33365327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e84f1dfd01","size":5,"soft_fail":false,"target_va":5402978144,"va":5369612812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7918329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8f9d27800","size":5,"soft_fail":false,"target_va":5377529888,"va":5369611554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7751147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8eb457600","size":5,"soft_fail":false,"target_va":5377364336,"va":5369613184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33366403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e88321fd01","size":5,"soft_fail":false,"target_va":5402979152,"va":5369612744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33367328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e82025fd01","size":5,"soft_fail":false,"target_va":5402980128,"va":5369612795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_groups
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4601312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8e0354600","size":5,"soft_fail":false,"target_va":5374214016,"va":5369612699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4597255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e807264600","size":5,"soft_fail":false,"target_va":5374209808,"va":5369612548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6629215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e85f276500","size":5,"soft_fail":false,"target_va":5376240848,"va":5369611628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6730010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e81ab16600","size":5,"soft_fail":false,"target_va":5376341760,"va":5369611745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6730206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8deb16600","size":5,"soft_fail":false,"target_va":5376342096,"va":5369611885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::bind_individual_mips
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6730273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e821b26600","size":5,"soft_fail":false,"target_va":5376342368,"va":5369612090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6730232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8f8b16600","size":5,"soft_fail":false,"target_va":5376342160,"va":5369611923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6729912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8b8b06600","size":5,"soft_fail":false,"target_va":5376341776,"va":5369611859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::view_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6729681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8d1af6600","size":5,"soft_fail":false,"target_va":5376341840,"va":5369612154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::as_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6736604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8dcca6600","size":5,"soft_fail":false,"target_va":5376348208,"va":5369611599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6736063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369611376,"raw_hex":"e8bfc86600","size":5,"soft_fail":false,"target_va":5376347648,"va":5369611580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":72,"record_offset":500,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":72,"record_offset":680,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":72,"record_offset":608,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":72,"record_offset":1000,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8420,"source_line":407,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":72,"record_offset":880,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ustr-1.0.0/src/lib.rs
//     evidence={"module":72,"record_offset":580,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8417,"source_line":469,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":232,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":232,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":232,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":808,"parent_record_offset":752,"ancestor_record_offset":232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":72,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":232,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":780,"parent_record_offset":752,"ancestor_record_offset":232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":232,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":1212,"parent_record_offset":1184,"ancestor_record_offset":232,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":1388,"parent_record_offset":1212,"ancestor_record_offset":232,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":72,"record_offset":1148,"parent_record_offset":1116,"ancestor_record_offset":232,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":72,"record_offset":1116,"parent_record_offset":1084,"ancestor_record_offset":232,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":72,"record_offset":1052,"parent_record_offset":1000,"ancestor_record_offset":232,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":72,"record_offset":1084,"parent_record_offset":1052,"ancestor_record_offset":232,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":72,"record_offset":1184,"parent_record_offset":1148,"ancestor_record_offset":232,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":72,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":72,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":232,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":72,"record_offset":1424,"parent_record_offset":1148,"ancestor_record_offset":232,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":72,"record_offset":648,"parent_record_offset":608,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":72,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":232,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset>
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":424,"record_offset":28704,"rva":5953808,"va":5374662928}
// 编译实例 RVAs: 0x5ad910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1914386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8eec9e2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374664077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1913580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e814cde2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374663287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-447852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8942af9ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5374663527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405aafa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e877d5ffff","size":5,"soft_fail":false,"target_va":5374652320,"va":5374663204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t650345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e869ec0900","size":5,"soft_fail":false,"target_va":5375314864,"va":5374664514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1313742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8ce0b1400","size":5,"soft_fail":false,"target_va":5375978624,"va":5374664877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1575380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8d4091800","size":5,"soft_fail":false,"target_va":5376239920,"va":5374664535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8521190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8e6058200","size":5,"soft_fail":false,"target_va":5383184544,"va":5374663349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: generate_mips.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28929806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e80e6fb901","size":5,"soft_fail":false,"target_va":5403593360,"va":5374663549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28313491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e89307b001","size":5,"soft_fail":false,"target_va":5402978144,"va":5374664648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2866779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e85bbe2b00","size":5,"soft_fail":false,"target_va":5377529888,"va":5374663104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2699243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8eb2f2900","size":5,"soft_fail":false,"target_va":5377364336,"va":5374665088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28314567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8c70bb001","size":5,"soft_fail":false,"target_va":5402979152,"va":5374664580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28315492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8640fb001","size":5,"soft_fail":false,"target_va":5402980128,"va":5374664631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-454252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e89411f9ff","size":5,"soft_fail":false,"target_va":5374209808,"va":5374664055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1577665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8c1121800","size":5,"soft_fail":false,"target_va":5376240848,"va":5374663178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1678460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e87c9c1900","size":5,"soft_fail":false,"target_va":5376341760,"va":5374663295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1678700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e86c9d1900","size":5,"soft_fail":false,"target_va":5376342096,"va":5374663391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::bind_individual_mips
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1678766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8ae9d1900","size":5,"soft_fail":false,"target_va":5376342368,"va":5374663597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1678726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8869d1900","size":5,"soft_fail":false,"target_va":5376342160,"va":5374663429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1678406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e8469c1900","size":5,"soft_fail":false,"target_va":5376341776,"va":5374663365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::view_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1678174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e85e9b1900","size":5,"soft_fail":false,"target_va":5376341840,"va":5374663661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::as_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1685054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e83eb61900","size":5,"soft_fail":false,"target_va":5376348208,"va":5374663149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1684513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374662928,"raw_hex":"e821b41900","size":5,"soft_fail":false,"target_va":5376347648,"va":5374663130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":424,"record_offset":28952,"parent_record_offset":28704,"ancestor_record_offset":28704,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":424,"record_offset":29132,"parent_record_offset":28704,"ancestor_record_offset":28704,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":424,"record_offset":29060,"parent_record_offset":28704,"ancestor_record_offset":28704,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_groups
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":424,"record_offset":30128,"parent_record_offset":28704,"ancestor_record_offset":28704,"inlinee":22898,"source_line":456,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":424,"record_offset":29452,"parent_record_offset":28704,"ancestor_record_offset":28704,"inlinee":8420,"source_line":407,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":424,"record_offset":29332,"parent_record_offset":28704,"ancestor_record_offset":28704,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ustr-1.0.0/src/lib.rs
//     evidence={"module":424,"record_offset":29032,"parent_record_offset":28704,"ancestor_record_offset":28704,"inlinee":8417,"source_line":469,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":424,"record_offset":29744,"parent_record_offset":29720,"ancestor_record_offset":28704,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":424,"record_offset":29768,"parent_record_offset":29744,"ancestor_record_offset":28704,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":424,"record_offset":29284,"parent_record_offset":29260,"ancestor_record_offset":28704,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":424,"record_offset":29720,"parent_record_offset":29692,"ancestor_record_offset":28704,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":424,"record_offset":29260,"parent_record_offset":29204,"ancestor_record_offset":28704,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":424,"record_offset":29000,"parent_record_offset":28976,"ancestor_record_offset":28704,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":424,"record_offset":29232,"parent_record_offset":29204,"ancestor_record_offset":28704,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":424,"record_offset":29692,"parent_record_offset":29664,"ancestor_record_offset":28704,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":424,"record_offset":29664,"parent_record_offset":29636,"ancestor_record_offset":28704,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":424,"record_offset":29840,"parent_record_offset":29664,"ancestor_record_offset":28704,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":424,"record_offset":29204,"parent_record_offset":29180,"ancestor_record_offset":28704,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":424,"record_offset":29600,"parent_record_offset":29568,"ancestor_record_offset":28704,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":424,"record_offset":29568,"parent_record_offset":29536,"ancestor_record_offset":28704,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":424,"record_offset":29504,"parent_record_offset":29452,"ancestor_record_offset":28704,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":424,"record_offset":29536,"parent_record_offset":29504,"ancestor_record_offset":28704,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":424,"record_offset":29636,"parent_record_offset":29600,"ancestor_record_offset":28704,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":424,"record_offset":29900,"parent_record_offset":29876,"ancestor_record_offset":28704,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":424,"record_offset":28976,"parent_record_offset":28952,"ancestor_record_offset":28704,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":424,"record_offset":29876,"parent_record_offset":29600,"ancestor_record_offset":28704,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":424,"record_offset":29100,"parent_record_offset":29060,"ancestor_record_offset":28704,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":424,"record_offset":29796,"parent_record_offset":29768,"ancestor_record_offset":28704,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":424,"record_offset":30160,"parent_record_offset":30128,"ancestor_record_offset":28704,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00424.jsonl.gz"}
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
// 完整结构及原始类型记录: generate_mips.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: generate_mips.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: generate_mips.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":72,"record_offset":880,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00072.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":424,"record_offset":29332,"parent_record_offset":28704,"ancestor_record_offset":28704,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00424.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"c7ad43c79b8bcd4b8558e6bd","relative_path":"_generate_mips.cs.hlsl_cs_main_0.bin","sha256":"f903edb17549557cdd10b01c26caa328d76cd7a1c28ff315d29608d8c93e0921","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"generate_mips","prefix_candidates":["generate_mips"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _generate_mips.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[2,3,4]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[256,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"spdConstants","storage_class":"PushConstant","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,5]},"decorations":[],"pointee":{"id":5,"name":"type.ConstantBuffer.SpdConstants","record":{"opcode":"OpTypeStruct","operands":[27,27,34]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"mips","type_id":27,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"numWorkGroups","type_id":27,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"workGroupOffset","type_id":34,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"imgSrc","storage_class":"UniformConstant","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"imgDst","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,56]},"decorations":[],"pointee":{"id":56,"name":"unknown","record":{"opcode":"OpTypeArray","operands":[9,55]},"decorations":[],"element":{"id":9,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[50,1,2,1,0,2,8]},"decorations":[]}}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"imgDst6","storage_class":"UniformConstant","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[50,1,2,1,0,2,1]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]},{"decoration":"Coherent","operands":[]}],"access_qualifiers":["Binding","Coherent","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"spdGlobalAtomic","storage_class":"Uniform","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,13]},"decorations":[],"pointee":{"id":13,"name":"type.RWStructuredBuffer.SpdGlobalAtomicBuffer","record":{"opcode":"OpTypeStruct","operands":[22]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]},{"decoration":"Coherent","operands":[]}],"access_qualifiers":["Binding","BufferBlock","Coherent","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[27,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,27]},"decorations":[],"pointee":{"id":27,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,27]},"decorations":[],"pointee":{"id":27,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3141139,"position_among_mapped_render_targets":50,"mapped_render_target_count":70,"previous_callsite":3140794,"next_callsite":3141193,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5586384,"caller_aliases":{"alias_group_rva":5586384,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5587588,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: generate_mips.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>
// 信息 ID: e5778152979c50c1186a4df6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips -> ["country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fee13 -> 0xdc470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset>
// 信息 ID: d722058693cb449bab212e4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips -> ["country_core::resources::texture_presets::TexturePreset"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture @ 0x554284 -> 0x5ad910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
