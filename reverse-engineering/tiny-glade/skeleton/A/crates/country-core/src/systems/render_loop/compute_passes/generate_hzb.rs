// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 973103af08c803540062a9a16e290546d8e48b1164cbaf1ed600506a91f8c451
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: generate_hzb.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 469
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"PDB_registry_path_only","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":469,"record_offset":26468,"rva":6643968,"va":5375353088}
// 编译实例 RVAs: 0x656100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2604757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e82b41d8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375354448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2603736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e82845d8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375353443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-837226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e89639f3ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375353701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405aafa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-701031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8994df5ff","size":5,"soft_fail":false,"target_va":5374652320,"va":5375353346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-79520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e860c9feff","size":5,"soft_fail":false,"target_va":5375274368,"va":5375353883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-78322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e80ecefeff","size":5,"soft_fail":false,"target_va":5375275808,"va":5375354125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-40034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e89e63ffff","size":5,"soft_fail":false,"target_va":5375314864,"va":5375354893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t623262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e89e820900","size":5,"soft_fail":false,"target_va":5375978624,"va":5375355357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t884994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e802810d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375354921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7830703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8af7c7700","size":5,"soft_fail":false,"target_va":5383184544,"va":5375353836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: generate_hzb.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28239450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e85ae6ae01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375353905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27623074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8a27ea501","size":5,"soft_fail":false,"target_va":5402978144,"va":5375355065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2176637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e87d362100","size":5,"soft_fail":false,"target_va":5377529888,"va":5375353246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2009147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e83ba81e00","size":5,"soft_fail":false,"target_va":5377364336,"va":5375355184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27625075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e87386a501","size":5,"soft_fail":false,"target_va":5402980128,"va":5375355048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1144623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8d188eeff","size":5,"soft_fail":false,"target_va":5374209808,"va":5375354426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t887523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8e38a0d00","size":5,"soft_fail":false,"target_va":5376240848,"va":5375353320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::all_mip_levels
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t987395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e803110f00","size":5,"soft_fail":false,"target_va":5376340992,"va":5375353592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t988303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e88f140f00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375353452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t986930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8320f0f00","size":5,"soft_fail":false,"target_va":5376340512,"va":5375353577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t988102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8c6130f00","size":5,"soft_fail":false,"target_va":5376342096,"va":5375353989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::bind_individual_mips
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t988403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8f3140f00","size":5,"soft_fail":false,"target_va":5376342368,"va":5375353960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t987886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8ee120f00","size":5,"soft_fail":false,"target_va":5376342160,"va":5375354269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t987847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8c7120f00","size":5,"soft_fail":false,"target_va":5376341776,"va":5375353924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::view_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t987814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8a6120f00","size":5,"soft_fail":false,"target_va":5376341840,"va":5375354021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t865806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e80e360d00","size":5,"soft_fail":false,"target_va":5376219552,"va":5375353741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::as_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t994912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8602e0f00","size":5,"soft_fail":false,"target_va":5376348208,"va":5375353291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t994371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375353088,"raw_hex":"e8432c0f00","size":5,"soft_fail":false,"target_va":5376347648,"va":5375353272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":469,"record_offset":26672,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":469,"record_offset":27080,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":26695,"source_line":2736,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":26852,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":469,"record_offset":26780,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_groups
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":28100,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":22898,"source_line":456,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":27420,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":8420,"source_line":407,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":27340,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":27292,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":469,"record_offset":27192,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":469,"record_offset":27052,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ustr-1.0.0/src/lib.rs
//     evidence={"module":469,"record_offset":26752,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":8417,"source_line":469,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":27716,"parent_record_offset":27692,"ancestor_record_offset":26468,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":26468,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":27004,"parent_record_offset":26980,"ancestor_record_offset":26468,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":27692,"parent_record_offset":27664,"ancestor_record_offset":26468,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":26980,"parent_record_offset":26924,"ancestor_record_offset":26468,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":469,"record_offset":26720,"parent_record_offset":26696,"ancestor_record_offset":26468,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":26952,"parent_record_offset":26924,"ancestor_record_offset":26468,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":27664,"parent_record_offset":27636,"ancestor_record_offset":26468,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":27636,"parent_record_offset":27608,"ancestor_record_offset":26468,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":27812,"parent_record_offset":27636,"ancestor_record_offset":26468,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":26924,"parent_record_offset":26900,"ancestor_record_offset":26468,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":469,"record_offset":27572,"parent_record_offset":27540,"ancestor_record_offset":26468,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":469,"record_offset":27540,"parent_record_offset":27508,"ancestor_record_offset":26468,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":469,"record_offset":27476,"parent_record_offset":27420,"ancestor_record_offset":26468,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":469,"record_offset":27508,"parent_record_offset":27476,"ancestor_record_offset":26468,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":27608,"parent_record_offset":27572,"ancestor_record_offset":26468,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":469,"record_offset":27260,"parent_record_offset":27192,"ancestor_record_offset":26468,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":469,"record_offset":27872,"parent_record_offset":27848,"ancestor_record_offset":26468,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":469,"record_offset":27104,"parent_record_offset":27080,"ancestor_record_offset":26468,"inlinee":26696,"source_line":2704,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":469,"record_offset":26696,"parent_record_offset":26672,"ancestor_record_offset":26468,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":469,"record_offset":27848,"parent_record_offset":27572,"ancestor_record_offset":26468,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":26820,"parent_record_offset":26780,"ancestor_record_offset":26468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":27768,"parent_record_offset":27740,"ancestor_record_offset":26468,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":28132,"parent_record_offset":28100,"ancestor_record_offset":26468,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":469,"record_offset":27232,"parent_record_offset":27192,"ancestor_record_offset":26468,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00469.jsonl.gz"}
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
// 完整结构及原始类型记录: generate_hzb.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: generate_hzb.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: generate_hzb.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":469,"record_offset":27292,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00469.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":469,"record_offset":27340,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00469.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"8c4f8af8277e04e45d1a37a2","relative_path":"_generate_hzb.cs.hlsl_cs_main_0.bin","sha256":"9a2eea67293759684aeaeb8e98175bbe1cf87be252696c0ab7d54bc73ba5fdcf","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"generate_hzb","prefix_candidates":["generate_hzb","hzb_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _generate_hzb.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[256,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"spdConstants","storage_class":"PushConstant","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,6]},"decorations":[],"pointee":{"id":6,"name":"type.ConstantBuffer.SpdConstants","record":{"opcode":"OpTypeStruct","operands":[25,25,32,32]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"mips","type_id":25,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"numWorkGroups","type_id":25,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"workGroupOffset","type_id":32,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":3,"name":"input_dims","type_id":32,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"imgSrc","storage_class":"UniformConstant","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,8]},"decorations":[],"pointee":{"id":8,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"imgDst","storage_class":"UniformConstant","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeArray","operands":[10,57]},"decorations":[],"element":{"id":10,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[50,1,2,1,0,2,9]},"decorations":[]}}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"imgDst6","storage_class":"UniformConstant","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[50,1,2,1,0,2,3]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]},{"decoration":"Coherent","operands":[]}],"access_qualifiers":["Binding","Coherent","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"spdGlobalAtomic","storage_class":"Uniform","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.RWStructuredBuffer.SpdGlobalAtomicBuffer","record":{"opcode":"OpTypeStruct","operands":[20]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]},{"decoration":"Coherent","operands":[]}],"access_qualifiers":["Binding","BufferBlock","Coherent","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,66]},"decorations":[],"pointee":{"id":66,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[25,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3139015,"position_among_mapped_render_targets":29,"mapped_render_target_count":70,"previous_callsite":3138958,"next_callsite":3139056,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: generate_hzb.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
// 信息 ID: d9f3c717c8de36f017a9a31b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe5c7 -> 0x656100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
