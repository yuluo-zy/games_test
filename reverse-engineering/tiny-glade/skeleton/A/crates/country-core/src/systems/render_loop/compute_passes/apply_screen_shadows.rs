// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a16af3bda98307c05c6d56b0a0a1f13479adee5e5431f7160a050b6a8e7e2427
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: apply_screen_shadows.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 402
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":402,"record_offset":19512,"rva":5581088,"va":5374290208}
// 编译实例 RVAs: 0x552920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ac090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t366301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e8dd960500","size":5,"soft_fail":false,"target_va":5374656656,"va":5374290350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405acbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t368773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e885a00500","size":5,"soft_fail":false,"target_va":5374659536,"va":5374290758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t983751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e8c7020f00","size":5,"soft_fail":false,"target_va":5375274368,"va":5374290612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t984866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e822070f00","size":5,"soft_fail":false,"target_va":5375275808,"va":5374290937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1023559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e8479e0f00","size":5,"soft_fail":false,"target_va":5375314864,"va":5374291300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1687149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e86dbe1900","size":5,"soft_fail":false,"target_va":5375978624,"va":5374291470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1948587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e8abbb1d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5374291328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcab40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8879992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e8787f8700","size":5,"soft_fail":false,"target_va":5383170880,"va":5374290883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8894005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e835b68700","size":5,"soft_fail":false,"target_va":5383184544,"va":5374290534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd1ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8909428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e874f28700","size":5,"soft_fail":false,"target_va":5383200416,"va":5374290983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8937785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e839618800","size":5,"soft_fail":false,"target_va":5383228352,"va":5374290562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29302722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e8c21fbf01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374290633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3072653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e88de22e00","size":5,"soft_fail":false,"target_va":5377364336,"va":5374291678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28688693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e835c1b501","size":5,"soft_fail":false,"target_va":5402980128,"va":5374291430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2051495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e8a74d1f00","size":5,"soft_fail":false,"target_va":5376341760,"va":5374290260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2050223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e8af481f00","size":5,"soft_fail":false,"target_va":5376340512,"va":5374290284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2050416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e870491f00","size":5,"soft_fail":false,"target_va":5376340720,"va":5374290299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2050430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374290208,"raw_hex":"e87e491f00","size":5,"soft_fail":false,"target_va":5376340752,"va":5374290317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":402,"record_offset":19736,"parent_record_offset":19512,"ancestor_record_offset":19512,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":402,"record_offset":20080,"parent_record_offset":19512,"ancestor_record_offset":19512,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":402,"record_offset":20004,"parent_record_offset":19512,"ancestor_record_offset":19512,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":402,"record_offset":19812,"parent_record_offset":19512,"ancestor_record_offset":19512,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":402,"record_offset":19708,"parent_record_offset":19512,"ancestor_record_offset":19512,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":19780,"parent_record_offset":19736,"ancestor_record_offset":19512,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":402,"record_offset":20112,"parent_record_offset":20080,"ancestor_record_offset":19512,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":402,"record_offset":20416,"rva":5582848,"va":5374291968}
// 编译实例 RVAs: 0x553000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1022481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374291968,"raw_hex":"e8119a0f00","size":5,"soft_fail":false,"target_va":5375314864,"va":5374292378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1947516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374291968,"raw_hex":"e87cb71d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5374292399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8892493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374291968,"raw_hex":"e84db08700","size":5,"soft_fail":false,"target_va":5383184544,"va":5374292046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd1ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8908340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374291968,"raw_hex":"e834ee8700","size":5,"soft_fail":false,"target_va":5383200416,"va":5374292071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd3ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_screen_shadows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8915739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374291968,"raw_hex":"e81b0b8800","size":5,"soft_fail":false,"target_va":5383207840,"va":5374292096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":402,"record_offset":20600,"parent_record_offset":20416,"ancestor_record_offset":20416,"inlinee":22896,"source_line":440,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":20700,"parent_record_offset":20628,"ancestor_record_offset":20416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":402,"record_offset":20628,"parent_record_offset":20600,"ancestor_record_offset":20416,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00402.jsonl.gz"}
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
// 完整结构及原始类型记录: apply_screen_shadows.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: apply_screen_shadows.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: apply_screen_shadows.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":402,"record_offset":19812,"parent_record_offset":19512,"ancestor_record_offset":19512,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00402.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":402,"record_offset":20004,"parent_record_offset":19512,"ancestor_record_offset":19512,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00402.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"83866d3a9083ee3434e9635e","relative_path":"_apply_screen_shadows.cs.hlsl_cs_main_0.bin","sha256":"6ce595a88babb7f8922be72c561f08422d386b563304df0a32c1f0693eb7cdf2","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"apply_screen_shadows","prefix_candidates":["apply_screen_shadows"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _apply_screen_shadows.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[16,16,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"sampler_shadow_nearest","storage_class":"UniformConstant","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":7,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[7]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_shadow_linear","storage_class":"UniformConstant","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":8,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[8]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"frame_constants","storage_class":"Uniform","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,9]},"decorations":[],"pointee":{"id":9,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[59,59,59,59,59,59,59,59,36,34,34,36,89,36,34,44,34,44,36,44,36,59,30]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":59,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":59,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":59,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":59,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":59,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":59,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":59,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":59,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":36,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":34,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":34,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":36,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":89,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":36,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":34,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":44,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":34,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":44,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":36,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":44,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":36,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":59,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":30,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"$Globals","storage_class":"Uniform","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,13]},"decorations":[],"pointee":{"id":13,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[89,89,89,89,36,89]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"gbuffer_tex_dims","type_id":89,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"depth_tex_dims","type_id":89,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"history_tex_dims","type_id":89,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"reprojection_tex_dims","type_id":89,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"history_valid","type_id":36,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"output_tex_dims","type_id":89,"decorations":[{"decoration":"Offset","operands":[36]}]}]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"gbuffer_tex","storage_class":"UniformConstant","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,15]},"decorations":[],"pointee":{"id":15,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[36,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[34,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"history_tex","storage_class":"UniformConstant","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[34,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[34,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"shadow_map","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[34,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"output_tex","storage_class":"UniformConstant","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,22]},"decorations":[],"pointee":{"id":22,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[34,1,2,0,0,2,15]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"shadow_mask_histogram","storage_class":"Uniform","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,24]},"decorations":[],"pointee":{"id":24,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,99]},"decorations":[],"pointee":{"id":99,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[36,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,36]},"decorations":[],"pointee":{"id":36,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3140086,"position_among_mapped_render_targets":43,"mapped_render_target_count":70,"previous_callsite":3140035,"next_callsite":3140182,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/eval_gi_screen_radiance.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3140672,"position_among_mapped_render_targets":47,"mapped_render_target_count":70,"previous_callsite":3140650,"next_callsite":3140752,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: apply_screen_shadows.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>", "bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
// 信息 ID: fcc5ee97c54dfb268d96074a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe9f6 -> 0x552920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean
// 信息 ID: a05adc4c6a40f28966b7a2fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fec40 -> 0x553000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
