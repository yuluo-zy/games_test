// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/dof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 471d9d4691374c5c01c9405d3dd6e270eb0662a82179db98790dad868ca9cc47
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 400, 1308
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::dof_prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":48792,"rva":5550368,"va":5374259488}
// 编译实例 RVAs: 0x54b120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t256867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e863eb0300","size":5,"soft_fail":false,"target_va":5374516480,"va":5374259608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1014381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e86d7a0f00","size":5,"soft_fail":false,"target_va":5375274368,"va":5374259982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1015721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e8a97f0f00","size":5,"soft_fail":false,"target_va":5375275808,"va":5374260082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1054383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e8af161000","size":5,"soft_fail":false,"target_va":5375314864,"va":5374260476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1717912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e898361a00","size":5,"soft_fail":false,"target_va":5375978624,"va":5374260707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1979418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e81a341e00","size":5,"soft_fail":false,"target_va":5376239920,"va":5374260497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8924604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e8bc2d8800","size":5,"soft_fail":false,"target_va":5383184544,"va":5374259935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dda5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8974975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e87ff28800","size":5,"soft_fail":false,"target_va":5383235008,"va":5374260028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29333348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e86497bf01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374260007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::div_up_extent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2081574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e826c31f00","size":5,"soft_fail":false,"target_va":5376341328,"va":5374259749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2082213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e8a5c51f00","size":5,"soft_fail":false,"target_va":5376341760,"va":5374259542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::format
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2081105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e851c11f00","size":5,"soft_fail":false,"target_va":5376340880,"va":5374259770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2080941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e8adc01f00","size":5,"soft_fail":false,"target_va":5376340512,"va":5374259566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1959899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374259488,"raw_hex":"e8dbe71d00","size":5,"soft_fail":false,"target_va":5376219552,"va":5374259648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":49320,"parent_record_offset":48792,"ancestor_record_offset":48792,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":49224,"parent_record_offset":48792,"ancestor_record_offset":48792,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":49176,"parent_record_offset":48792,"ancestor_record_offset":48792,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":400,"record_offset":48988,"parent_record_offset":48792,"ancestor_record_offset":48792,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":400,"record_offset":48960,"parent_record_offset":48792,"ancestor_record_offset":48792,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":49048,"parent_record_offset":48988,"ancestor_record_offset":48792,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":49420,"parent_record_offset":49348,"ancestor_record_offset":48792,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":49348,"parent_record_offset":49320,"ancestor_record_offset":48792,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":400,"record_offset":49024,"parent_record_offset":48988,"ancestor_record_offset":48792,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::calc_dof
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":49656,"rva":5551744,"va":5374260864}
// 编译实例 RVAs: 0x54b680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1513202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e80ee9e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374262893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1515006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e802e2e8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374264713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540110
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e89e45ffff","size":5,"soft_fail":false,"target_va":5374214416,"va":5374262125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t255340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e86ce50300","size":5,"soft_fail":false,"target_va":5374516480,"va":5374261135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=19
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1012491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e80b730f00","size":5,"soft_fail":false,"target_va":5375274368,"va":5374261872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1013593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e859770f00","size":5,"soft_fail":false,"target_va":5375275808,"va":5374262210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1052242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8520e1000","size":5,"soft_fail":false,"target_va":5375314864,"va":5374262617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=31
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1708674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e882121a00","size":5,"soft_fail":false,"target_va":5375978624,"va":5374269945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140729090
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1954176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e880d11d00","size":5,"soft_fail":false,"target_va":5376217232,"va":5374263051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1977277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8bd2b1e00","size":5,"soft_fail":false,"target_va":5376239920,"va":5374262638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7c500
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8586961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8d1068300","size":5,"soft_fail":false,"target_va":5382849792,"va":5374262826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcd7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8914274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e862058800","size":5,"soft_fail":false,"target_va":5383182240,"va":5374267961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8922724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e864268800","size":5,"soft_fail":false,"target_va":5383184544,"va":5374261815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8919877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8451b8800","size":5,"soft_fail":false,"target_va":5383186880,"va":5374266998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd0e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8929918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e87e428800","size":5,"soft_fail":false,"target_va":5383196224,"va":5374266301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8ee0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8966880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8e0d28800","size":5,"soft_fail":false,"target_va":5383229152,"va":5374262267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dda050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8965198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e84ecc8800","size":5,"soft_fail":false,"target_va":5383233616,"va":5374268413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=34
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29331459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e80390bf01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374261896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28708842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8ea0fb601","size":5,"soft_fail":false,"target_va":5402978144,"va":5374269297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2955417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e867e7d2ff","size":5,"soft_fail":false,"target_va":5371308432,"va":5374263844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3257833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8e9b53100","size":5,"soft_fail":false,"target_va":5377527120,"va":5374269282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28716188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e89c2cb601","size":5,"soft_fail":false,"target_va":5402978880,"va":5374262687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-62548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8ac0bffff","size":5,"soft_fail":false,"target_va":5374207728,"va":5374270271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28711184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e81019b601","size":5,"soft_fail":false,"target_va":5402980464,"va":5374269275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-52493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8f332ffff","size":5,"soft_fail":false,"target_va":5374211808,"va":5374264296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-53353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8972fffff","size":5,"soft_fail":false,"target_va":5374212640,"va":5374265988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t-55846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8da25ffff","size":5,"soft_fail":false,"target_va":5374209136,"va":5374264977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-54352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8b02bffff","size":5,"soft_fail":false,"target_va":5374209808,"va":5374264155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1977838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8ee2d1e00","size":5,"soft_fail":false,"target_va":5376240848,"va":5374263005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1998479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e88f7e1e00","size":5,"soft_fail":false,"target_va":5376262400,"va":5374263916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1997155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e863791e00","size":5,"soft_fail":false,"target_va":5376261200,"va":5374264040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2080787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e813c01f00","size":5,"soft_fail":false,"target_va":5376341760,"va":5374260968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::format
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2079791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e82fbc1f00","size":5,"soft_fail":false,"target_va":5376340880,"va":5374261084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::half_res
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2080640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e880bf1f00","size":5,"soft_fail":false,"target_va":5376341664,"va":5374261019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2079154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8b2b91f00","size":5,"soft_fail":false,"target_va":5376340512,"va":5374261353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t1958372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374260864,"raw_hex":"e8e4e11d00","size":5,"soft_fail":false,"target_va":5376219552,"va":5374261175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::dof::calc_dof_fullres
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/dof.rs
//     evidence={"module":400,"record_offset":59420,"parent_record_offset":49656,"ancestor_record_offset":49656,"inlinee":23005,"source_line":77,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::dof::calc_dof_multires
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/dof.rs
//     evidence={"module":400,"record_offset":49812,"parent_record_offset":49656,"ancestor_record_offset":49656,"inlinee":23001,"source_line":158,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":51012,"parent_record_offset":50988,"ancestor_record_offset":49656,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":51036,"parent_record_offset":51012,"ancestor_record_offset":49656,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":54944,"parent_record_offset":54920,"ancestor_record_offset":49656,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":50988,"parent_record_offset":50876,"ancestor_record_offset":49656,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":54920,"parent_record_offset":54820,"ancestor_record_offset":49656,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":54852,"parent_record_offset":54820,"ancestor_record_offset":49656,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":53940,"parent_record_offset":53916,"ancestor_record_offset":49656,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":50876,"parent_record_offset":50828,"ancestor_record_offset":49656,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":50828,"parent_record_offset":50780,"ancestor_record_offset":49656,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":51108,"parent_record_offset":50828,"ancestor_record_offset":49656,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":54820,"parent_record_offset":54792,"ancestor_record_offset":49656,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":50728,"parent_record_offset":50680,"ancestor_record_offset":49656,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":50680,"parent_record_offset":50632,"ancestor_record_offset":49656,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":50632,"parent_record_offset":50588,"ancestor_record_offset":49656,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":53916,"parent_record_offset":53876,"ancestor_record_offset":49656,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":53876,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":50780,"parent_record_offset":50728,"ancestor_record_offset":49656,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":50588,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":50528,"parent_record_offset":50504,"ancestor_record_offset":49656,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":400,"record_offset":54028,"parent_record_offset":54004,"ancestor_record_offset":49656,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":400,"record_offset":54004,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":400,"record_offset":51652,"parent_record_offset":51620,"ancestor_record_offset":49656,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":400,"record_offset":53176,"parent_record_offset":53152,"ancestor_record_offset":49656,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":400,"record_offset":53152,"parent_record_offset":53128,"ancestor_record_offset":49656,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":400,"record_offset":53100,"parent_record_offset":53068,"ancestor_record_offset":49656,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":57088,"parent_record_offset":56700,"ancestor_record_offset":49656,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":53044,"parent_record_offset":52652,"ancestor_record_offset":49656,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":53068,"parent_record_offset":53044,"ancestor_record_offset":49656,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":52684,"parent_record_offset":52652,"ancestor_record_offset":49656,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":53128,"parent_record_offset":53068,"ancestor_record_offset":49656,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":52732,"parent_record_offset":52708,"ancestor_record_offset":49656,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":52708,"parent_record_offset":52684,"ancestor_record_offset":49656,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":52824,"parent_record_offset":52760,"ancestor_record_offset":49656,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":52912,"parent_record_offset":52888,"ancestor_record_offset":49656,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":400,"record_offset":53244,"parent_record_offset":53220,"ancestor_record_offset":49656,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":53220,"parent_record_offset":52652,"ancestor_record_offset":49656,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":55684,"parent_record_offset":55652,"ancestor_record_offset":49656,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":56700,"parent_record_offset":55684,"ancestor_record_offset":49656,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":51620,"parent_record_offset":51584,"ancestor_record_offset":49656,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":52652,"parent_record_offset":51620,"ancestor_record_offset":49656,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":400,"record_offset":50932,"parent_record_offset":50876,"ancestor_record_offset":49656,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":400,"record_offset":50956,"parent_record_offset":50932,"ancestor_record_offset":49656,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":400,"record_offset":52384,"parent_record_offset":52360,"ancestor_record_offset":49656,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":400,"record_offset":52432,"parent_record_offset":52408,"ancestor_record_offset":49656,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":400,"record_offset":52360,"parent_record_offset":52336,"ancestor_record_offset":49656,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":50280,"parent_record_offset":50212,"ancestor_record_offset":49656,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":400,"record_offset":51852,"parent_record_offset":51828,"ancestor_record_offset":49656,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":400,"record_offset":51936,"parent_record_offset":51912,"ancestor_record_offset":49656,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":51172,"parent_record_offset":51148,"ancestor_record_offset":49656,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":400,"record_offset":50552,"parent_record_offset":50528,"ancestor_record_offset":49656,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":400,"record_offset":50504,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":400,"record_offset":50376,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":7215,"source_line":229,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":400,"record_offset":50404,"parent_record_offset":50376,"ancestor_record_offset":49656,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":400,"record_offset":54280,"parent_record_offset":54252,"ancestor_record_offset":49656,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":400,"record_offset":54884,"parent_record_offset":54852,"ancestor_record_offset":49656,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":400,"record_offset":50312,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":23002,"source_line":2605,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":400,"record_offset":52048,"parent_record_offset":52024,"ancestor_record_offset":49656,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":52848,"parent_record_offset":52824,"ancestor_record_offset":49656,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":52620,"parent_record_offset":51652,"ancestor_record_offset":49656,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":52888,"parent_record_offset":52708,"ancestor_record_offset":49656,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":53340,"parent_record_offset":51584,"ancestor_record_offset":49656,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":50460,"parent_record_offset":50432,"ancestor_record_offset":49656,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":52992,"parent_record_offset":52968,"ancestor_record_offset":49656,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":51148,"parent_record_offset":50728,"ancestor_record_offset":49656,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":54204,"parent_record_offset":54060,"ancestor_record_offset":49656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":400,"record_offset":52272,"parent_record_offset":52248,"ancestor_record_offset":49656,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":400,"record_offset":52192,"parent_record_offset":52168,"ancestor_record_offset":49656,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":400,"record_offset":51252,"parent_record_offset":51220,"ancestor_record_offset":49656,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":54304,"parent_record_offset":54280,"ancestor_record_offset":49656,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":51064,"parent_record_offset":51036,"ancestor_record_offset":49656,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":53972,"parent_record_offset":53876,"ancestor_record_offset":49656,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":400,"record_offset":54136,"parent_record_offset":54060,"ancestor_record_offset":49656,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":400,"record_offset":52760,"parent_record_offset":52732,"ancestor_record_offset":49656,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":400,"record_offset":52796,"parent_record_offset":52760,"ancestor_record_offset":49656,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":400,"record_offset":52968,"parent_record_offset":52936,"ancestor_record_offset":49656,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":400,"record_offset":57776,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":400,"record_offset":51220,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":400,"record_offset":53268,"parent_record_offset":53244,"ancestor_record_offset":49656,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":400,"record_offset":53292,"parent_record_offset":53268,"ancestor_record_offset":49656,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":400,"record_offset":52408,"parent_record_offset":52384,"ancestor_record_offset":49656,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":61448,"parent_record_offset":59420,"ancestor_record_offset":49656,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":54436,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":22896,"source_line":440,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":55076,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":54468,"parent_record_offset":54436,"ancestor_record_offset":49656,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":55032,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=24
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":54988,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":400,"record_offset":57220,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":22349,"source_line":76,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":400,"record_offset":55652,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":400,"record_offset":51584,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":400,"record_offset":50432,"parent_record_offset":50404,"ancestor_record_offset":49656,"inlinee":23004,"source_line":221,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":400,"record_offset":52336,"parent_record_offset":52312,"ancestor_record_offset":49656,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":51680,"parent_record_offset":51652,"ancestor_record_offset":49656,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":51708,"parent_record_offset":51680,"ancestor_record_offset":49656,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":52312,"parent_record_offset":52116,"ancestor_record_offset":49656,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52224,"parent_record_offset":52140,"ancestor_record_offset":49656,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52168,"parent_record_offset":52140,"ancestor_record_offset":49656,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52568,"parent_record_offset":51788,"ancestor_record_offset":49656,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52140,"parent_record_offset":52116,"ancestor_record_offset":49656,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":51764,"parent_record_offset":51740,"ancestor_record_offset":49656,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":51740,"parent_record_offset":51708,"ancestor_record_offset":49656,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":51788,"parent_record_offset":51764,"ancestor_record_offset":49656,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":51996,"parent_record_offset":51968,"ancestor_record_offset":49656,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":52024,"parent_record_offset":51996,"ancestor_record_offset":49656,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":52084,"parent_record_offset":51968,"ancestor_record_offset":49656,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":51968,"parent_record_offset":51788,"ancestor_record_offset":49656,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52116,"parent_record_offset":51788,"ancestor_record_offset":49656,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":51828,"parent_record_offset":51788,"ancestor_record_offset":49656,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":51912,"parent_record_offset":51788,"ancestor_record_offset":49656,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":52484,"parent_record_offset":51788,"ancestor_record_offset":49656,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":400,"record_offset":52936,"parent_record_offset":52912,"ancestor_record_offset":49656,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$4::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":400,"record_offset":51324,"parent_record_offset":51284,"ancestor_record_offset":49656,"inlinee":23007,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":400,"record_offset":54108,"parent_record_offset":54060,"ancestor_record_offset":49656,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":400,"record_offset":54060,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":23008,"source_line":150,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":400,"record_offset":54252,"parent_record_offset":54228,"ancestor_record_offset":49656,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":400,"record_offset":50252,"parent_record_offset":50212,"ancestor_record_offset":49656,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":400,"record_offset":50212,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":400,"record_offset":50156,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::calc_dof_fullres
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x59DD
// 首条位置: {"module":400,"inlinee":23005}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":61448,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":61400,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":61208,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":400,"record_offset":61180,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":22349,"source_line":76,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":400,"record_offset":59612,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":400,"record_offset":59512,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":400,"record_offset":59484,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":400,"record_offset":59672,"parent_record_offset":59644,"ancestor_record_offset":59420,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":400,"record_offset":61104,"parent_record_offset":61072,"ancestor_record_offset":59420,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":61048,"parent_record_offset":60660,"ancestor_record_offset":59420,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":61072,"parent_record_offset":61048,"ancestor_record_offset":59420,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":60688,"parent_record_offset":60660,"ancestor_record_offset":59420,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":60736,"parent_record_offset":60712,"ancestor_record_offset":59420,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":60712,"parent_record_offset":60688,"ancestor_record_offset":59420,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":60828,"parent_record_offset":60764,"ancestor_record_offset":59420,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":60916,"parent_record_offset":60892,"ancestor_record_offset":59420,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":59644,"parent_record_offset":59612,"ancestor_record_offset":59420,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":60660,"parent_record_offset":59644,"ancestor_record_offset":59420,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":400,"record_offset":60392,"parent_record_offset":60368,"ancestor_record_offset":59420,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":400,"record_offset":60440,"parent_record_offset":60416,"ancestor_record_offset":59420,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":400,"record_offset":60368,"parent_record_offset":60344,"ancestor_record_offset":59420,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":59580,"parent_record_offset":59512,"ancestor_record_offset":59420,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":400,"record_offset":59860,"parent_record_offset":59836,"ancestor_record_offset":59420,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":400,"record_offset":59944,"parent_record_offset":59920,"ancestor_record_offset":59420,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":59884,"parent_record_offset":59860,"ancestor_record_offset":59420,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":400,"record_offset":60056,"parent_record_offset":60032,"ancestor_record_offset":59420,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":60852,"parent_record_offset":60828,"ancestor_record_offset":59420,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":60628,"parent_record_offset":59672,"ancestor_record_offset":59420,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":60892,"parent_record_offset":60712,"ancestor_record_offset":59420,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":61148,"parent_record_offset":59612,"ancestor_record_offset":59420,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":60996,"parent_record_offset":60972,"ancestor_record_offset":59420,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":61552,"parent_record_offset":61480,"ancestor_record_offset":59420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":400,"record_offset":60280,"parent_record_offset":60256,"ancestor_record_offset":59420,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":400,"record_offset":60200,"parent_record_offset":60176,"ancestor_record_offset":59420,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":400,"record_offset":60764,"parent_record_offset":60736,"ancestor_record_offset":59420,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":400,"record_offset":60800,"parent_record_offset":60764,"ancestor_record_offset":59420,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":400,"record_offset":60972,"parent_record_offset":60940,"ancestor_record_offset":59420,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":400,"record_offset":60416,"parent_record_offset":60392,"ancestor_record_offset":59420,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":61480,"parent_record_offset":61448,"ancestor_record_offset":59420,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":400,"record_offset":60344,"parent_record_offset":60320,"ancestor_record_offset":59420,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":59696,"parent_record_offset":59672,"ancestor_record_offset":59420,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":59720,"parent_record_offset":59696,"ancestor_record_offset":59420,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":60320,"parent_record_offset":60124,"ancestor_record_offset":59420,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":60232,"parent_record_offset":60148,"ancestor_record_offset":59420,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":60176,"parent_record_offset":60148,"ancestor_record_offset":59420,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":60576,"parent_record_offset":59796,"ancestor_record_offset":59420,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":60148,"parent_record_offset":60124,"ancestor_record_offset":59420,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":59772,"parent_record_offset":59748,"ancestor_record_offset":59420,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":59748,"parent_record_offset":59720,"ancestor_record_offset":59420,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":59796,"parent_record_offset":59772,"ancestor_record_offset":59420,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":60004,"parent_record_offset":59976,"ancestor_record_offset":59420,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":60032,"parent_record_offset":60004,"ancestor_record_offset":59420,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":60092,"parent_record_offset":59976,"ancestor_record_offset":59420,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":59976,"parent_record_offset":59796,"ancestor_record_offset":59420,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":60124,"parent_record_offset":59796,"ancestor_record_offset":59420,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":59836,"parent_record_offset":59796,"ancestor_record_offset":59420,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":59920,"parent_record_offset":59796,"ancestor_record_offset":59420,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":60492,"parent_record_offset":59796,"ancestor_record_offset":59420,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":400,"record_offset":60940,"parent_record_offset":60916,"ancestor_record_offset":59420,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":400,"record_offset":59552,"parent_record_offset":59512,"ancestor_record_offset":59420,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":62496,"rva":5563600,"va":5374272720}
// 编译实例 RVAs: 0x54e4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2712031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e8219ed6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374273802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1862575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e85194e3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374273610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1523084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e874c2e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374272775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1523288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e8a8c1e8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374272995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28704731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e8dbffb501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374273408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2965629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e883bfd2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374273288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3337097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e889eb3200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374273218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28706617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e83907b601","size":5,"soft_fail":false,"target_va":5402980128,"va":5374273506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":64796,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":65388,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":65740,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":65880,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":62684,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":63956,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63076,"parent_record_offset":63052,"ancestor_record_offset":62496,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63100,"parent_record_offset":63076,"ancestor_record_offset":62496,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63904,"parent_record_offset":63880,"ancestor_record_offset":62496,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63052,"parent_record_offset":63024,"ancestor_record_offset":62496,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63880,"parent_record_offset":63856,"ancestor_record_offset":62496,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":65636,"parent_record_offset":65608,"ancestor_record_offset":62496,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":65816,"parent_record_offset":65784,"ancestor_record_offset":62496,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":63024,"parent_record_offset":62996,"ancestor_record_offset":62496,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":62996,"parent_record_offset":62968,"ancestor_record_offset":62496,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":63168,"parent_record_offset":62996,"ancestor_record_offset":62496,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":63856,"parent_record_offset":63832,"ancestor_record_offset":62496,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62932,"parent_record_offset":62904,"ancestor_record_offset":62496,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62904,"parent_record_offset":62876,"ancestor_record_offset":62496,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62848,"parent_record_offset":62820,"ancestor_record_offset":62496,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62876,"parent_record_offset":62848,"ancestor_record_offset":62496,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62820,"parent_record_offset":62792,"ancestor_record_offset":62496,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":62792,"parent_record_offset":62764,"ancestor_record_offset":62496,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":65440,"parent_record_offset":65388,"ancestor_record_offset":62496,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":62764,"parent_record_offset":62736,"ancestor_record_offset":62496,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":65784,"parent_record_offset":65740,"ancestor_record_offset":62496,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":62968,"parent_record_offset":62932,"ancestor_record_offset":62496,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":62736,"parent_record_offset":62684,"ancestor_record_offset":62496,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":65468,"parent_record_offset":65388,"ancestor_record_offset":62496,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":63228,"parent_record_offset":63204,"ancestor_record_offset":62496,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":64028,"parent_record_offset":64000,"ancestor_record_offset":62496,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":64000,"parent_record_offset":63956,"ancestor_record_offset":62496,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":63204,"parent_record_offset":62932,"ancestor_record_offset":62496,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":63784,"parent_record_offset":62684,"ancestor_record_offset":62496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":63124,"parent_record_offset":63100,"ancestor_record_offset":62496,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":65848,"parent_record_offset":65740,"ancestor_record_offset":62496,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":65496,"parent_record_offset":65388,"ancestor_record_offset":62496,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":66244,"rva":5564752,"va":5374273872}
// 编译实例 RVAs: 0x54e950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2713183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e8a199d6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374274954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1863727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e8d18fe3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374274762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1524236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e8f4bde8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374273927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1524440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e828bde8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374274147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28703579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e85bfbb501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374274560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2966781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e803bbd2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374274440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3335945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e809e73200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374274370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28705465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e8b902b601","size":5,"soft_fail":false,"target_va":5402980128,"va":5374274658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":68544,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":69136,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":69488,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":69628,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":66432,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":67704,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":66824,"parent_record_offset":66800,"ancestor_record_offset":66244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":66848,"parent_record_offset":66824,"ancestor_record_offset":66244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":67652,"parent_record_offset":67628,"ancestor_record_offset":66244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":66800,"parent_record_offset":66772,"ancestor_record_offset":66244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":67628,"parent_record_offset":67604,"ancestor_record_offset":66244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":69384,"parent_record_offset":69356,"ancestor_record_offset":66244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":69564,"parent_record_offset":69532,"ancestor_record_offset":66244,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":66772,"parent_record_offset":66744,"ancestor_record_offset":66244,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":66744,"parent_record_offset":66716,"ancestor_record_offset":66244,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":66916,"parent_record_offset":66744,"ancestor_record_offset":66244,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":67604,"parent_record_offset":67580,"ancestor_record_offset":66244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66680,"parent_record_offset":66652,"ancestor_record_offset":66244,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66652,"parent_record_offset":66624,"ancestor_record_offset":66244,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66596,"parent_record_offset":66568,"ancestor_record_offset":66244,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66624,"parent_record_offset":66596,"ancestor_record_offset":66244,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66568,"parent_record_offset":66540,"ancestor_record_offset":66244,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":66540,"parent_record_offset":66512,"ancestor_record_offset":66244,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":69188,"parent_record_offset":69136,"ancestor_record_offset":66244,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":66512,"parent_record_offset":66484,"ancestor_record_offset":66244,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":69532,"parent_record_offset":69488,"ancestor_record_offset":66244,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":66716,"parent_record_offset":66680,"ancestor_record_offset":66244,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":66484,"parent_record_offset":66432,"ancestor_record_offset":66244,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":69216,"parent_record_offset":69136,"ancestor_record_offset":66244,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":66976,"parent_record_offset":66952,"ancestor_record_offset":66244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":67776,"parent_record_offset":67748,"ancestor_record_offset":66244,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":67748,"parent_record_offset":67704,"ancestor_record_offset":66244,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":66952,"parent_record_offset":66680,"ancestor_record_offset":66244,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":67532,"parent_record_offset":66432,"ancestor_record_offset":66244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":66872,"parent_record_offset":66848,"ancestor_record_offset":66244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":69596,"parent_record_offset":69488,"ancestor_record_offset":66244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":69244,"parent_record_offset":69136,"ancestor_record_offset":66244,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":69996,"rva":5565904,"va":5374275024}
// 编译实例 RVAs: 0x54edd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2714335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e82195d6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374276106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1864879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e8518be3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374275914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1525388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e874b9e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374275079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1525588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e8acb8e8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374275295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28702431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e8dff6b501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374275708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2967929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e887b6d2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374275588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3334797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e88de23200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374275518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28704317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e83dfeb501","size":5,"soft_fail":false,"target_va":5402980128,"va":5374275806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":72300,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":72892,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":73244,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":73384,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":70188,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":71460,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":70580,"parent_record_offset":70556,"ancestor_record_offset":69996,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":70604,"parent_record_offset":70580,"ancestor_record_offset":69996,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":71408,"parent_record_offset":71384,"ancestor_record_offset":69996,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":70556,"parent_record_offset":70528,"ancestor_record_offset":69996,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":71384,"parent_record_offset":71360,"ancestor_record_offset":69996,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":73140,"parent_record_offset":73112,"ancestor_record_offset":69996,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":73320,"parent_record_offset":73288,"ancestor_record_offset":69996,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":70528,"parent_record_offset":70500,"ancestor_record_offset":69996,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":70500,"parent_record_offset":70472,"ancestor_record_offset":69996,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":70672,"parent_record_offset":70500,"ancestor_record_offset":69996,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":71360,"parent_record_offset":71336,"ancestor_record_offset":69996,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70436,"parent_record_offset":70408,"ancestor_record_offset":69996,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70408,"parent_record_offset":70380,"ancestor_record_offset":69996,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70352,"parent_record_offset":70324,"ancestor_record_offset":69996,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70380,"parent_record_offset":70352,"ancestor_record_offset":69996,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70324,"parent_record_offset":70296,"ancestor_record_offset":69996,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":70296,"parent_record_offset":70268,"ancestor_record_offset":69996,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":72944,"parent_record_offset":72892,"ancestor_record_offset":69996,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":70268,"parent_record_offset":70240,"ancestor_record_offset":69996,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":73288,"parent_record_offset":73244,"ancestor_record_offset":69996,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":70472,"parent_record_offset":70436,"ancestor_record_offset":69996,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":70240,"parent_record_offset":70188,"ancestor_record_offset":69996,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":72972,"parent_record_offset":72892,"ancestor_record_offset":69996,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":70732,"parent_record_offset":70708,"ancestor_record_offset":69996,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":71532,"parent_record_offset":71504,"ancestor_record_offset":69996,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":71504,"parent_record_offset":71460,"ancestor_record_offset":69996,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":70708,"parent_record_offset":70436,"ancestor_record_offset":69996,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":71288,"parent_record_offset":70188,"ancestor_record_offset":69996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":70628,"parent_record_offset":70604,"ancestor_record_offset":69996,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":73352,"parent_record_offset":73244,"ancestor_record_offset":69996,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":73000,"parent_record_offset":72892,"ancestor_record_offset":69996,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":73556,"rva":5567008,"va":5374276128}
// 编译实例 RVAs: 0x54f220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2715439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e8d190d6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374277210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1865983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e80187e3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374277018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1526492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e824b5e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374276183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1526699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e855b4e8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374276406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28701320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e888f2b501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374276819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2969040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e830b2d2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374276699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3333686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e836de3200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374276629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28703206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e8e6f9b501","size":5,"soft_fail":false,"target_va":5402980128,"va":5374276917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":75860,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":76452,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":76804,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":76944,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":73748,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":75020,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74140,"parent_record_offset":74116,"ancestor_record_offset":73556,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74164,"parent_record_offset":74140,"ancestor_record_offset":73556,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74968,"parent_record_offset":74944,"ancestor_record_offset":73556,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74116,"parent_record_offset":74088,"ancestor_record_offset":73556,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74944,"parent_record_offset":74920,"ancestor_record_offset":73556,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":76700,"parent_record_offset":76672,"ancestor_record_offset":73556,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":76880,"parent_record_offset":76848,"ancestor_record_offset":73556,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":74088,"parent_record_offset":74060,"ancestor_record_offset":73556,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":74060,"parent_record_offset":74032,"ancestor_record_offset":73556,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":74232,"parent_record_offset":74060,"ancestor_record_offset":73556,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":74920,"parent_record_offset":74896,"ancestor_record_offset":73556,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73996,"parent_record_offset":73968,"ancestor_record_offset":73556,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73968,"parent_record_offset":73940,"ancestor_record_offset":73556,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73912,"parent_record_offset":73884,"ancestor_record_offset":73556,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73940,"parent_record_offset":73912,"ancestor_record_offset":73556,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73884,"parent_record_offset":73856,"ancestor_record_offset":73556,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":73856,"parent_record_offset":73828,"ancestor_record_offset":73556,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":76504,"parent_record_offset":76452,"ancestor_record_offset":73556,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":73828,"parent_record_offset":73800,"ancestor_record_offset":73556,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":76848,"parent_record_offset":76804,"ancestor_record_offset":73556,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":74032,"parent_record_offset":73996,"ancestor_record_offset":73556,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":73800,"parent_record_offset":73748,"ancestor_record_offset":73556,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":76532,"parent_record_offset":76452,"ancestor_record_offset":73556,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":74292,"parent_record_offset":74268,"ancestor_record_offset":73556,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":75092,"parent_record_offset":75064,"ancestor_record_offset":73556,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":75064,"parent_record_offset":75020,"ancestor_record_offset":73556,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":74268,"parent_record_offset":73996,"ancestor_record_offset":73556,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":74848,"parent_record_offset":73748,"ancestor_record_offset":73556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":74188,"parent_record_offset":74164,"ancestor_record_offset":73556,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":76912,"parent_record_offset":76804,"ancestor_record_offset":73556,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":76560,"parent_record_offset":76452,"ancestor_record_offset":73556,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":77116,"rva":5568112,"va":5374277232}
// 编译实例 RVAs: 0x54f670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2716559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8718cd6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374278330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1867103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8a182e3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374278138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1527596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8d4b0e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374277287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1527810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8feafe8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374277517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28700209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e831eeb501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374277930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2970151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8d9add2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374277810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3332575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8dfd93200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374277740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28702095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e88ff5b501","size":5,"soft_fail":false,"target_va":5402980128,"va":5374278028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":79420,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":80012,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":80364,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":80504,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":77308,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":78580,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":77700,"parent_record_offset":77676,"ancestor_record_offset":77116,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":77724,"parent_record_offset":77700,"ancestor_record_offset":77116,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":78528,"parent_record_offset":78504,"ancestor_record_offset":77116,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":77676,"parent_record_offset":77648,"ancestor_record_offset":77116,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":78504,"parent_record_offset":78480,"ancestor_record_offset":77116,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":80260,"parent_record_offset":80232,"ancestor_record_offset":77116,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":80440,"parent_record_offset":80408,"ancestor_record_offset":77116,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":77648,"parent_record_offset":77620,"ancestor_record_offset":77116,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":77620,"parent_record_offset":77592,"ancestor_record_offset":77116,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":77792,"parent_record_offset":77620,"ancestor_record_offset":77116,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":78480,"parent_record_offset":78456,"ancestor_record_offset":77116,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77556,"parent_record_offset":77528,"ancestor_record_offset":77116,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77528,"parent_record_offset":77500,"ancestor_record_offset":77116,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77472,"parent_record_offset":77444,"ancestor_record_offset":77116,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77500,"parent_record_offset":77472,"ancestor_record_offset":77116,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77444,"parent_record_offset":77416,"ancestor_record_offset":77116,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":77416,"parent_record_offset":77388,"ancestor_record_offset":77116,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":80064,"parent_record_offset":80012,"ancestor_record_offset":77116,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":77388,"parent_record_offset":77360,"ancestor_record_offset":77116,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":80408,"parent_record_offset":80364,"ancestor_record_offset":77116,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":77592,"parent_record_offset":77556,"ancestor_record_offset":77116,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":77360,"parent_record_offset":77308,"ancestor_record_offset":77116,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":80092,"parent_record_offset":80012,"ancestor_record_offset":77116,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":77852,"parent_record_offset":77828,"ancestor_record_offset":77116,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":78652,"parent_record_offset":78624,"ancestor_record_offset":77116,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":78624,"parent_record_offset":78580,"ancestor_record_offset":77116,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":77828,"parent_record_offset":77556,"ancestor_record_offset":77116,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":78408,"parent_record_offset":77308,"ancestor_record_offset":77116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":77748,"parent_record_offset":77724,"ancestor_record_offset":77116,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":80472,"parent_record_offset":80364,"ancestor_record_offset":77116,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":80120,"parent_record_offset":80012,"ancestor_record_offset":77116,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::calc_dof_multires
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x59D9
// 首条位置: {"module":400,"inlinee":23001}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":53876,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":50588,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":400,"record_offset":54004,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":400,"record_offset":50504,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":400,"record_offset":50376,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":7215,"source_line":229,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":400,"record_offset":50312,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":23002,"source_line":2605,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":54768,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":400,"record_offset":57776,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":400,"record_offset":51220,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":54436,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":22896,"source_line":440,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":55076,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":55032,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=20
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":54988,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":400,"record_offset":57220,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":22349,"source_line":76,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":400,"record_offset":55652,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":400,"record_offset":51584,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":400,"record_offset":54060,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":23008,"source_line":150,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":400,"record_offset":50212,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":400,"record_offset":50156,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":51012,"parent_record_offset":50988,"ancestor_record_offset":49812,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":51036,"parent_record_offset":51012,"ancestor_record_offset":49812,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":54944,"parent_record_offset":54920,"ancestor_record_offset":49812,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":50988,"parent_record_offset":50876,"ancestor_record_offset":49812,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":54920,"parent_record_offset":54820,"ancestor_record_offset":49812,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":54852,"parent_record_offset":54820,"ancestor_record_offset":49812,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":53940,"parent_record_offset":53916,"ancestor_record_offset":49812,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":50876,"parent_record_offset":50828,"ancestor_record_offset":49812,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":50828,"parent_record_offset":50780,"ancestor_record_offset":49812,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":51108,"parent_record_offset":50828,"ancestor_record_offset":49812,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":54820,"parent_record_offset":54792,"ancestor_record_offset":49812,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":50728,"parent_record_offset":50680,"ancestor_record_offset":49812,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":50680,"parent_record_offset":50632,"ancestor_record_offset":49812,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":50632,"parent_record_offset":50588,"ancestor_record_offset":49812,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":53916,"parent_record_offset":53876,"ancestor_record_offset":49812,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":50780,"parent_record_offset":50728,"ancestor_record_offset":49812,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":50528,"parent_record_offset":50504,"ancestor_record_offset":49812,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":400,"record_offset":54028,"parent_record_offset":54004,"ancestor_record_offset":49812,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":400,"record_offset":51652,"parent_record_offset":51620,"ancestor_record_offset":49812,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":400,"record_offset":53176,"parent_record_offset":53152,"ancestor_record_offset":49812,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":400,"record_offset":53152,"parent_record_offset":53128,"ancestor_record_offset":49812,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":400,"record_offset":53100,"parent_record_offset":53068,"ancestor_record_offset":49812,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":57088,"parent_record_offset":56700,"ancestor_record_offset":49812,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":53044,"parent_record_offset":52652,"ancestor_record_offset":49812,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":53068,"parent_record_offset":53044,"ancestor_record_offset":49812,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":52684,"parent_record_offset":52652,"ancestor_record_offset":49812,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":400,"record_offset":53128,"parent_record_offset":53068,"ancestor_record_offset":49812,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":52732,"parent_record_offset":52708,"ancestor_record_offset":49812,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":52708,"parent_record_offset":52684,"ancestor_record_offset":49812,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":52824,"parent_record_offset":52760,"ancestor_record_offset":49812,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":400,"record_offset":52912,"parent_record_offset":52888,"ancestor_record_offset":49812,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":400,"record_offset":53244,"parent_record_offset":53220,"ancestor_record_offset":49812,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":53220,"parent_record_offset":52652,"ancestor_record_offset":49812,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":55684,"parent_record_offset":55652,"ancestor_record_offset":49812,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":56700,"parent_record_offset":55684,"ancestor_record_offset":49812,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":51620,"parent_record_offset":51584,"ancestor_record_offset":49812,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":400,"record_offset":52652,"parent_record_offset":51620,"ancestor_record_offset":49812,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":400,"record_offset":50932,"parent_record_offset":50876,"ancestor_record_offset":49812,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":400,"record_offset":50956,"parent_record_offset":50932,"ancestor_record_offset":49812,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":400,"record_offset":52384,"parent_record_offset":52360,"ancestor_record_offset":49812,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":400,"record_offset":52432,"parent_record_offset":52408,"ancestor_record_offset":49812,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":400,"record_offset":52360,"parent_record_offset":52336,"ancestor_record_offset":49812,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":50280,"parent_record_offset":50212,"ancestor_record_offset":49812,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":400,"record_offset":51852,"parent_record_offset":51828,"ancestor_record_offset":49812,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":400,"record_offset":51936,"parent_record_offset":51912,"ancestor_record_offset":49812,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":51172,"parent_record_offset":51148,"ancestor_record_offset":49812,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":400,"record_offset":50552,"parent_record_offset":50528,"ancestor_record_offset":49812,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":400,"record_offset":50404,"parent_record_offset":50376,"ancestor_record_offset":49812,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":400,"record_offset":54280,"parent_record_offset":54252,"ancestor_record_offset":49812,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":400,"record_offset":54884,"parent_record_offset":54852,"ancestor_record_offset":49812,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":400,"record_offset":52048,"parent_record_offset":52024,"ancestor_record_offset":49812,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":52848,"parent_record_offset":52824,"ancestor_record_offset":49812,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":52620,"parent_record_offset":51652,"ancestor_record_offset":49812,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":52888,"parent_record_offset":52708,"ancestor_record_offset":49812,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":53340,"parent_record_offset":51584,"ancestor_record_offset":49812,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":50460,"parent_record_offset":50432,"ancestor_record_offset":49812,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":52992,"parent_record_offset":52968,"ancestor_record_offset":49812,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":51148,"parent_record_offset":50728,"ancestor_record_offset":49812,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":54204,"parent_record_offset":54060,"ancestor_record_offset":49812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":400,"record_offset":52272,"parent_record_offset":52248,"ancestor_record_offset":49812,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":400,"record_offset":52192,"parent_record_offset":52168,"ancestor_record_offset":49812,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":400,"record_offset":51252,"parent_record_offset":51220,"ancestor_record_offset":49812,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":54304,"parent_record_offset":54280,"ancestor_record_offset":49812,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":51064,"parent_record_offset":51036,"ancestor_record_offset":49812,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":53972,"parent_record_offset":53876,"ancestor_record_offset":49812,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":400,"record_offset":54136,"parent_record_offset":54060,"ancestor_record_offset":49812,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":400,"record_offset":52760,"parent_record_offset":52732,"ancestor_record_offset":49812,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":400,"record_offset":52796,"parent_record_offset":52760,"ancestor_record_offset":49812,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":400,"record_offset":52968,"parent_record_offset":52936,"ancestor_record_offset":49812,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":400,"record_offset":53268,"parent_record_offset":53244,"ancestor_record_offset":49812,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":400,"record_offset":53292,"parent_record_offset":53268,"ancestor_record_offset":49812,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":400,"record_offset":52408,"parent_record_offset":52384,"ancestor_record_offset":49812,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":400,"record_offset":54468,"parent_record_offset":54436,"ancestor_record_offset":49812,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":400,"record_offset":50432,"parent_record_offset":50404,"ancestor_record_offset":49812,"inlinee":23004,"source_line":221,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":400,"record_offset":52336,"parent_record_offset":52312,"ancestor_record_offset":49812,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":51680,"parent_record_offset":51652,"ancestor_record_offset":49812,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":51708,"parent_record_offset":51680,"ancestor_record_offset":49812,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":400,"record_offset":52312,"parent_record_offset":52116,"ancestor_record_offset":49812,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52224,"parent_record_offset":52140,"ancestor_record_offset":49812,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52168,"parent_record_offset":52140,"ancestor_record_offset":49812,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52568,"parent_record_offset":51788,"ancestor_record_offset":49812,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52140,"parent_record_offset":52116,"ancestor_record_offset":49812,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":51764,"parent_record_offset":51740,"ancestor_record_offset":49812,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":51740,"parent_record_offset":51708,"ancestor_record_offset":49812,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":51788,"parent_record_offset":51764,"ancestor_record_offset":49812,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":51996,"parent_record_offset":51968,"ancestor_record_offset":49812,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":52024,"parent_record_offset":51996,"ancestor_record_offset":49812,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":52084,"parent_record_offset":51968,"ancestor_record_offset":49812,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":400,"record_offset":51968,"parent_record_offset":51788,"ancestor_record_offset":49812,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":400,"record_offset":52116,"parent_record_offset":51788,"ancestor_record_offset":49812,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":51828,"parent_record_offset":51788,"ancestor_record_offset":49812,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":51912,"parent_record_offset":51788,"ancestor_record_offset":49812,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":400,"record_offset":52484,"parent_record_offset":51788,"ancestor_record_offset":49812,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":400,"record_offset":52936,"parent_record_offset":52912,"ancestor_record_offset":49812,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$4::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":400,"record_offset":51324,"parent_record_offset":51284,"ancestor_record_offset":49812,"inlinee":23007,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":400,"record_offset":54108,"parent_record_offset":54060,"ancestor_record_offset":49812,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":400,"record_offset":54252,"parent_record_offset":54228,"ancestor_record_offset":49812,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":400,"record_offset":50252,"parent_record_offset":50212,"ancestor_record_offset":49812,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF06E
// 首条位置: {"module":1308,"inlinee":61550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: dof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof::FullResDof
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::compute_passes::dof::FullResDof>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::compute_passes::dof::FullResDof"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof::HalfResAllDof
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new", "arguments": ["country_core::systems::render_loop::compute_passes::dof::HalfResAllDof"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof::HalfResDof
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new", "arguments": ["country_core::systems::render_loop::compute_passes::dof::HalfResDof"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::dof::IndirectArgsType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>", "constructor": "enum_iterator::All", "arguments": ["country_core::systems::render_loop::compute_passes::dof::IndirectArgsType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>", "country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["u32", "core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/dof.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: dof.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof::dof_prepare
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":400,"record_offset":49176,"parent_record_offset":48792,"ancestor_record_offset":48792,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00400.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":400,"record_offset":49224,"parent_record_offset":48792,"ancestor_record_offset":48792,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00400.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof::calc_dof
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":400,"record_offset":54988,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00400.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_transitive","evidence":{"module":400,"record_offset":55032,"parent_record_offset":49812,"ancestor_record_offset":49656,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00400.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof::calc_dof_fullres
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":400,"record_offset":61208,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00400.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":400,"record_offset":61400,"parent_record_offset":59420,"ancestor_record_offset":59420,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00400.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::dof::calc_dof_multires
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":400,"record_offset":54988,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00400.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":400,"record_offset":55032,"parent_record_offset":49812,"ancestor_record_offset":49812,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00400.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"76c71bb9ed69117382c9c6f7","relative_path":"_dof_analyze.cs.hlsl_cs_main_0.bin","sha256":"dbdd1ae325bbd43faf19e89f858a0c510e455cc688fa0cf3c0dfb566da309872","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"f14e9682a93c2937231ee6fa","relative_path":"_dof_autofocus.cs.hlsl_cs_main_0.bin","sha256":"8132f6aa57601509b830a927ebf7427c7c8401dc88a51ad5b645b34ca9d102f8","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"cc178ca93f91a0913caa6016","relative_path":"_dof_clear_halfres.cs.hlsl_cs_main_0.bin","sha256":"58aaee5c35ace08eaa21115b16d5a0dac5886b22a66a404cbf4a309d3184e807","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"c9727122d76cee8563547410","relative_path":"_dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin","sha256":"7472c2dc74fb32b16aac3ac46e4a47ab13d5fc74f666a55d6885a401c13f8112","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"2d238ea572e5dd674239761f","relative_path":"_dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin","sha256":"0f3e4bb614fd527da0d9141edb936d9da36ff53c84a4b1cf0cf9b08b2e2b65a3","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"7ee2838ece49cc9a0ba5ec04","relative_path":"_dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin","sha256":"b740d901a979ea337af70a1104a5d7a06ee7d5dc024749e57352e5e65984bc27","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"f255069001b2284e4fbd7624","relative_path":"_dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin","sha256":"624698a69dac9266798e966412c73a13cd8e06db2133c091d0315432727f6927","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"278f0f63c3b054b9ed92f08c","relative_path":"_dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin","sha256":"4cf45977c4886b2dd1b86e39750a6be362d54afe0813029e3d05ff464f3c9caf","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"cf768b2905dc1fae89f7fa7c","relative_path":"_dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin","sha256":"dd25e7dfafd48c3e7a55f4c8c5704870cba1b05f14fd8ebca41e3c1f08e77f19","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"0724a29d04f580f2c8a9bc07","relative_path":"_dof_fullres_upsample.cs.hlsl_cs_main_0.bin","sha256":"745320e76c99bd8152d2c6fb81d0f881ab78fca5ad7a880936f866f26dbf3691","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"02022370aaddcbbf904720b4","relative_path":"_dof_fused.cs.hlsl_cs_main_0.bin","sha256":"60590f9dd33fe731d651ff8e5c79e473914a81d1fb6446dfcb4f82aacc3a538a","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"0e538339f3eeb691c2f9aac8","relative_path":"_dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin","sha256":"7b7bfdb2cf909176d8dc1e65f173f958d2ae569fc28155f475a66add2a1e24ba","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"d495f19bd09db64b82611f78","relative_path":"_dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin","sha256":"0a76662f7ac381311d74669340aa3eb69babe6d991b293d8906604c86e6368af","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"18b10b55a81e0c92f356c370","relative_path":"_dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin","sha256":"26f5b6dc584cc9f904a2bb837e24332c1e56010e7584ef201f25881328ad630b","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"05dbb991bf6c4fd00a270709","relative_path":"_dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin","sha256":"264aa6c497ea0245449fb39c4986cdc48fcda0fc2f053b65b88eaba2110e2c01","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"3af4aa1f2db4ef80171ba537","relative_path":"_dof_init_buffers.cs.hlsl_cs_main_0.bin","sha256":"3758863601081c5009216122f9c42897049c47052e23ffaf13ecbe8c101877fd","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"193a42e3d391894862d1e291","relative_path":"_dof_prepare.cs.hlsl_cs_main_0.bin","sha256":"9836f009fd12c68e8811cc622a2e6aabb3ba787b82ecdff4155de02fc9c3df66","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"dof","prefix_candidates":["dof","dof_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fused.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":80,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":80,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"frame_constants","storage_class":"Uniform","type":{"id":83,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,8]},"decorations":[],"pointee":{"id":8,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[46,46,46,46,46,46,46,46,26,25,25,26,39,26,25,32,25,32,26,32,26,46,21]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":46,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":46,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":46,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":46,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":46,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":46,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":46,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":46,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":25,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":25,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":39,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":26,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":25,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":32,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":25,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":32,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":26,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":32,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":26,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":46,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":21,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"$Globals","storage_class":"Uniform","type":{"id":84,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[25,39,39,39,39,39]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":25,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":39,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":39,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"depth_tex_dims","type_id":39,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"coc_tex_dims","type_id":39,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"img_output_dims","type_id":39,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":85,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"color_tex","storage_class":"UniformConstant","type":{"id":85,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":85,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"img_output","storage_class":"UniformConstant","type":{"id":86,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,17]},"decorations":[],"pointee":{"id":17,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,91]},"decorations":[],"pointee":{"id":91,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[26,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,26]},"decorations":[],"pointee":{"id":26,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_halfres_all","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[55,55,55,55,55,55,55,55,40,39,39,40,50,40,39,98,39,98,40,98,40,55,29]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":55,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":55,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":55,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":39,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":39,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":50,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":39,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":98,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":39,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":98,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":98,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":55,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":29,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[39,50,50,50,50,50,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":110,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[32]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":32,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,112]},"decorations":[],"pointee":{"id":112,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,112]},"decorations":[],"pointee":{"id":112,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_upsample.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"halfres_tex","storage_class":"UniformConstant","type":{"id":40,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"spatial_resolve_offsets","storage_class":"Uniform","type":{"id":43,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,7]},"decorations":[],"pointee":{"id":7,"name":"type.spatial_resolve_offsets","record":{"opcode":"OpTypeStruct","operands":[16]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"spatial_resolve_offsets_array","type_id":16,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"img_output","storage_class":"UniformConstant","type":{"id":44,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[18]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":18,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[19]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":19,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[24,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[24,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_halfres","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[55,55,55,55,55,55,55,55,40,39,39,40,50,40,39,101,39,101,40,101,40,55,29]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":55,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":55,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":55,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":55,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":39,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":39,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":50,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":39,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":101,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":39,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":101,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":101,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":55,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":29,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[39,50,50,50,50,50,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":107,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[32]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":32,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,114]},"decorations":[],"pointee":{"id":114,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,114]},"decorations":[],"pointee":{"id":114,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":116,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":116,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_halfres_all","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[54,54,54,54,54,54,54,54,40,39,39,40,49,40,39,100,39,100,40,100,40,54,29]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":54,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":54,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":54,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":54,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":54,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":54,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":54,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":54,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":39,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":39,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":49,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":39,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":100,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":39,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":100,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":100,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":54,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":29,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[39,49,49,49,49,49,49]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[32]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":32,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,113]},"decorations":[],"pointee":{"id":113,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,113]},"decorations":[],"pointee":{"id":113,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_prepare.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"frame_constants","storage_class":"Uniform","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[48,48,48,48,48,48,48,48,26,25,25,26,34,26,25,53,25,53,26,53,26,48,21]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":48,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":48,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":48,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":48,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":48,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":48,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":48,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":48,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":25,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":25,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":34,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":26,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":25,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":25,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":53,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":26,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":53,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":26,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":48,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":21,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"$Globals","storage_class":"Uniform","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,9]},"decorations":[],"pointee":{"id":9,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[34,34,34]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":34,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"coc_out_tex_dims","type_id":34,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"tile_output_tex_dims","type_id":34,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,8]},"decorations":[],"pointee":{"id":8,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"autofocus_buf","storage_class":"Uniform","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.StructuredBuffer.DofFocus","record":{"opcode":"OpTypeStruct","operands":[22]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"coc_out_tex","storage_class":"UniformConstant","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,15]},"decorations":[],"pointee":{"id":15,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"tile_output_tex","storage_class":"UniformConstant","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,17]},"decorations":[],"pointee":{"id":17,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[25,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,63]},"decorations":[],"pointee":{"id":63,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[26,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,26]},"decorations":[],"pointee":{"id":26,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[66,66,66,66,66,66,66,66,39,38,38,39,48,39,38,100,38,100,39,100,39,66,29]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":66,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":66,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":66,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":39,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":38,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":38,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":39,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":48,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":39,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":38,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":100,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":38,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":100,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":39,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":100,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":39,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":66,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":29,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[38,48,48,48,48,48,48]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":38,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":109,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":110,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[32]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":32,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,111]},"decorations":[],"pointee":{"id":111,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,111]},"decorations":[],"pointee":{"id":111,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,39]},"decorations":[],"pointee":{"id":39,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,39]},"decorations":[],"pointee":{"id":39,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":109,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[64,64,64,64,64,64,64,64,43,42,42,43,54,43,42,106,42,106,43,106,43,64,32]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":64,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":64,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":64,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":43,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":42,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":42,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":43,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":54,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":43,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":42,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":106,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":42,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":106,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":43,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":106,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":43,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":64,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":32,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":110,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[42,54,54,54,54,54,54,54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":42,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":54,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":54,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":54,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":54,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":54,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":54,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"halfres_tex_dims","type_id":54,"decorations":[{"decoration":"Offset","operands":[56]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"halfres_tex","storage_class":"UniformConstant","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"spatial_resolve_offsets","storage_class":"Uniform","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,24]},"decorations":[],"pointee":{"id":24,"name":"type.spatial_resolve_offsets","record":{"opcode":"OpTypeStruct","operands":[33]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"spatial_resolve_offsets_array","type_id":33,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[35]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,29]},"decorations":[],"pointee":{"id":29,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[36]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":36,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,122]},"decorations":[],"pointee":{"id":122,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[43,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,122]},"decorations":[],"pointee":{"id":122,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[43,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,43]},"decorations":[],"pointee":{"id":43,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,43]},"decorations":[],"pointee":{"id":43,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_init_buffers.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[2]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"tile_offsets_initial_values","storage_class":"Uniform","type":{"id":21,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,3]},"decorations":[],"pointee":{"id":3,"name":"type.ConstantBuffer.DofTileOffsetsPacked","record":{"opcode":"OpTypeStruct","operands":[10]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"offsets","type_id":10,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"dispatch_args_buffer","storage_class":"Uniform","type":{"id":22,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"type.RWStructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[11]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":11,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,7]},"decorations":[],"pointee":{"id":7,"name":"type.RWStructuredBuffer.DofTileOffsetsPacked","record":{"opcode":"OpTypeStruct","operands":[12]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":12,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":25,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[15,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_analyze.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"$Globals","storage_class":"Uniform","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,6]},"decorations":[],"pointee":{"id":6,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[42,42]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"coc_tiles_tex_dims","type_id":42,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"coc_tiles_dilated_tex_dims","type_id":42,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"coc_tiles_tex","storage_class":"UniformConstant","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[30,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[30,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[19]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":19,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"dispatch_args_buffer","storage_class":"Uniform","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.RWStructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[20]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":68,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,16]},"decorations":[],"pointee":{"id":16,"name":"type.RWStructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,69]},"decorations":[],"pointee":{"id":69,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[31,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,31]},"decorations":[],"pointee":{"id":31,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[67,67,67,67,67,67,67,67,40,39,39,40,49,40,39,101,39,101,40,101,40,67,29]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":67,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":67,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":67,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":39,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":39,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":49,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":39,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":101,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":39,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":101,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":101,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":67,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":29,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[39,49,49,49,49,49,49]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":107,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[32]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":32,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,113]},"decorations":[],"pointee":{"id":113,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,113]},"decorations":[],"pointee":{"id":113,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":110,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[63,63,63,63,63,63,63,63,43,42,42,43,53,43,42,107,42,107,43,107,43,63,32]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":63,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":63,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":63,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":63,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":63,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":63,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":63,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":63,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":43,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":42,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":42,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":43,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":53,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":43,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":42,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":107,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":42,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":107,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":43,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":107,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":43,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":63,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":32,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[42,53,53,53,53,53,53,53]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":42,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":53,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":53,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":53,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":53,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":53,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":53,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"halfres_tex_dims","type_id":53,"decorations":[{"decoration":"Offset","operands":[56]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"halfres_tex","storage_class":"UniformConstant","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"spatial_resolve_offsets","storage_class":"Uniform","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,24]},"decorations":[],"pointee":{"id":24,"name":"type.spatial_resolve_offsets","record":{"opcode":"OpTypeStruct","operands":[33]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"spatial_resolve_offsets_array","type_id":33,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[35]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,29]},"decorations":[],"pointee":{"id":29,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[36]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":36,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,123]},"decorations":[],"pointee":{"id":123,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[43,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":124,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,123]},"decorations":[],"pointee":{"id":123,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[43,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,43]},"decorations":[],"pointee":{"id":43,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,43]},"decorations":[],"pointee":{"id":43,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_clear_halfres.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[2,3]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"img_output","storage_class":"UniformConstant","type":{"id":28,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[17,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":29,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,6]},"decorations":[],"pointee":{"id":6,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[12]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":12,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":30,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,9]},"decorations":[],"pointee":{"id":9,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[13]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":13,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":32,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,31]},"decorations":[],"pointee":{"id":31,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[18,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":32,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,31]},"decorations":[],"pointee":{"id":31,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[18,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[66,66,66,66,66,66,66,66,39,38,38,39,48,39,38,95,38,95,39,95,39,66,29]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":66,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":66,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":66,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":39,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":38,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":38,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":39,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":48,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":39,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":38,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":38,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":95,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":39,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":95,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":39,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":66,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":29,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[38,48,48,48,48,48,48]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":38,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[32]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":32,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":108,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,107]},"decorations":[],"pointee":{"id":107,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":108,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,107]},"decorations":[],"pointee":{"id":107,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":109,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,39]},"decorations":[],"pointee":{"id":39,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":109,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,39]},"decorations":[],"pointee":{"id":39,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_halfres","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[56,56,56,56,56,56,56,56,40,39,39,40,51,40,39,99,39,99,40,99,40,56,29]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":56,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":56,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":56,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":56,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":56,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":56,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":56,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":56,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":39,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":39,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":51,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":39,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":99,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":39,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":99,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":99,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":56,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":29,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[39,51,51,51,51,51,51]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":51,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":112,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[32]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":32,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,113]},"decorations":[],"pointee":{"id":113,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,113]},"decorations":[],"pointee":{"id":113,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":115,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_autofocus.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"frame_constants","storage_class":"Uniform","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[53,53,53,53,53,53,53,53,27,26,26,27,60,27,26,32,26,32,27,32,27,53,20]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":53,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":53,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":53,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":27,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":26,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":26,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":27,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":60,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":27,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":26,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":32,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":26,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":32,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":27,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":32,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":27,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":53,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":20,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"$Globals","storage_class":"Uniform","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,9]},"decorations":[],"pointee":{"id":9,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,31,27,27,27,26,27,26]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"point_uv","type_id":31,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"use_dual_focus","type_id":27,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"focus_lock_request","type_id":27,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":4,"name":"focus_is_locked","type_id":27,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":5,"name":"focus_box_height","type_id":26,"decorations":[{"decoration":"Offset","operands":[28]}]},{"index":6,"name":"use_tilt_shift","type_id":27,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":7,"name":"aperture_size","type_id":26,"decorations":[{"decoration":"Offset","operands":[36]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,8]},"decorations":[],"pointee":{"id":8,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[26,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"autofocus_buf","storage_class":"Uniform","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.RWStructuredBuffer.DofFocus","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"focus_readback_buf","storage_class":"Uniform","type":{"id":67,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"type.RWStructuredBuffer.DofFocusReadback","record":{"opcode":"OpTypeStruct","operands":[22]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,71]},"decorations":[],"pointee":{"id":71,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[27,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,27]},"decorations":[],"pointee":{"id":27,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"frame_constants","storage_class":"Uniform","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[67,67,67,67,67,67,67,67,40,39,39,40,49,40,39,96,39,96,40,96,40,67,29]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":67,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":67,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":67,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":39,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":39,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":49,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":39,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":96,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":39,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":96,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":96,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":67,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":29,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"$Globals","storage_class":"Uniform","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[39,49,49,49,49,49,49]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"bokeh_anisotropy","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"color_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"coc_tiles_dilated_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"depth_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"coc_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"img_output_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"color_tex","storage_class":"UniformConstant","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"coc_tiles_dilated_tex","storage_class":"UniformConstant","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"coc_tex","storage_class":"UniformConstant","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,13]},"decorations":[],"pointee":{"id":13,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"img_output","storage_class":"UniformConstant","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[39,1,2,0,0,2,2]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"tile_offsets_buffer","storage_class":"Uniform","type":{"id":107,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.StructuredBuffer.DofTileOffsets","record":{"opcode":"OpTypeStruct","operands":[31]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"tile_coords_buffer","storage_class":"Uniform","type":{"id":108,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.StructuredBuffer.v2uint","record":{"opcode":"OpTypeStruct","operands":[32]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":32,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":110,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,109]},"decorations":[],"pointee":{"id":109,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":110,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,109]},"decorations":[],"pointee":{"id":109,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[40,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":111,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,40]},"decorations":[],"pointee":{"id":40,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":41,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[41]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3140794,"position_among_mapped_render_targets":49,"mapped_render_target_count":70,"previous_callsite":3140752,"next_callsite":3141139,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3141193,"position_among_mapped_render_targets":51,"mapped_render_target_count":70,"previous_callsite":3141139,"next_callsite":3141597,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/generate_mips.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/taa.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::calc_dof
// 信息 ID: 1d70b804c10aac570dc27ecf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fee49 -> 0x54b680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::calc_dof_fullres
// 信息 ID: 5fd3891e3afe8072a01b2be5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=400, inline_record=59420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::calc_dof_multires
// 信息 ID: 08f7b33a680e5f19a9653765
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=400, inline_record=49812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure$0
// 信息 ID: df5a85b4e5f7148d4b7b3a7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::dof::calc_dof_multires", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >; module=1308, inline_record=220824; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<enum_iterator::All<country_core::systems::render_loop::compute_passes::dof::IndirectArgsType>,country_core::systems::render_loop::compute_passes::dof::calc_dof_multires::closure_env$0> >; module=1308, inline_record=221408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::dof_prepare
// 信息 ID: 1e644ffdb2bf82b107537a67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fecba -> 0x54b120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline
// 信息 ID: 2e93ee231871fe867e5e9206
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline
// 信息 ID: af24464cec24550f5628c271
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0xdd1027 -> 0x54f220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline
// 信息 ID: 46d870ffbc4742157b76fb45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0xdceba7 -> 0x54f670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline
// 信息 ID: a6ccabfe513126b3312d9608
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline
// 信息 ID: 9060cd99681963f6a190de7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
