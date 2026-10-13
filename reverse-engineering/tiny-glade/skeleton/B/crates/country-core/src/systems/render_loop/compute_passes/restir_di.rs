// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/restir_di.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 40787d0b2058b85b3d984e91395022573bb7be8596bf4dc7533a468130bbf3ee
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: restir_di.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 552
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":552,"record_offset":41304,"rva":10944192,"va":5379653312}
// 编译实例 RVAs: 0xa6fec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10037279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8e1d766ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379653386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1939141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e83b69e2ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379655280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1582618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8e6d9e7ff","size":5,"soft_fail":false,"target_va":5378071808,"va":5379654421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1580866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8bee0e7ff","size":5,"soft_fail":false,"target_va":5378073152,"va":5379654013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409056d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1485408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8a055e9ff","size":5,"soft_fail":false,"target_va":5378168528,"va":5379653931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1075274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8b697efff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379653477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1068526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e812b2efff","size":5,"soft_fail":false,"target_va":5378585968,"va":5379654489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1067867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8a5b4efff","size":5,"soft_fail":false,"target_va":5378586704,"va":5379654566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1059757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e853d4efff","size":5,"soft_fail":false,"target_va":5378593936,"va":5379653688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096f090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1053156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e81ceeefff","size":5,"soft_fail":false,"target_va":5378601104,"va":5379654255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-602156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8d4cff6ff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379654903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1468263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e867671600","size":5,"soft_fail":false,"target_va":5381123296,"va":5379655028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1515423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e89f1f1700","size":5,"soft_fail":false,"target_va":5381170352,"va":5379654924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12136876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8ac31b900","size":5,"soft_fail":false,"target_va":5391790752,"va":5379653871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160d680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12178387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8d3d3b900","size":5,"soft_fail":false,"target_va":5391832704,"va":5379654312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32193600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8403ceb01","size":5,"soft_fail":false,"target_va":5411847152,"va":5379653547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: restir_di.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32193197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8ad3aeb01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379653966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2999592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e828c52d00","size":5,"soft_fail":false,"target_va":5382655024,"va":5379655427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31886608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8108de601","size":5,"soft_fail":false,"target_va":5411541776,"va":5379655163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31827277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e84da5e501","size":5,"soft_fail":false,"target_va":5411482433,"va":5379655151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31887646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e81e91e601","size":5,"soft_fail":false,"target_va":5411542880,"va":5379655229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1058821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8fbd7efff","size":5,"soft_fail":false,"target_va":5378595520,"va":5379654336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1464241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8b1571600","size":5,"soft_fail":false,"target_va":5381117840,"va":5379653594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1462949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8a5521600","size":5,"soft_fail":false,"target_va":5381116576,"va":5379653622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1463139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e863531600","size":5,"soft_fail":false,"target_va":5381116784,"va":5379653640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1463153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e871531600","size":5,"soft_fail":false,"target_va":5381116816,"va":5379653658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1543992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379653312,"raw_hex":"e8388f1700","size":5,"soft_fail":false,"target_va":5381199056,"va":5379655059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379653312,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379653567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":552,"record_offset":42596,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":552,"record_offset":42744,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":552,"record_offset":42356,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":552,"record_offset":43260,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":552,"record_offset":43060,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":552,"record_offset":42820,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":552,"record_offset":43108,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":552,"record_offset":42632,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":30361,"source_line":392,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":552,"record_offset":41520,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":552,"record_offset":43596,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":552,"record_offset":42716,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":552,"record_offset":41972,"parent_record_offset":41948,"ancestor_record_offset":41304,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":552,"record_offset":42020,"parent_record_offset":41996,"ancestor_record_offset":41304,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":552,"record_offset":41996,"parent_record_offset":41972,"ancestor_record_offset":41304,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":41948,"parent_record_offset":41924,"ancestor_record_offset":41304,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":41924,"parent_record_offset":41884,"ancestor_record_offset":41304,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":41884,"parent_record_offset":41860,"ancestor_record_offset":41304,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":552,"record_offset":42272,"parent_record_offset":42248,"ancestor_record_offset":41304,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":552,"record_offset":42248,"parent_record_offset":42224,"ancestor_record_offset":41304,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":552,"record_offset":42196,"parent_record_offset":42164,"ancestor_record_offset":41304,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":552,"record_offset":42140,"parent_record_offset":41612,"ancestor_record_offset":41304,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":552,"record_offset":42164,"parent_record_offset":42140,"ancestor_record_offset":41304,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":552,"record_offset":41640,"parent_record_offset":41612,"ancestor_record_offset":41304,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":552,"record_offset":42224,"parent_record_offset":42164,"ancestor_record_offset":41304,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":552,"record_offset":41688,"parent_record_offset":41664,"ancestor_record_offset":41304,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":552,"record_offset":41664,"parent_record_offset":41640,"ancestor_record_offset":41304,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":552,"record_offset":41860,"parent_record_offset":41836,"ancestor_record_offset":41304,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":552,"record_offset":41576,"parent_record_offset":41548,"ancestor_record_offset":41304,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":552,"record_offset":41548,"parent_record_offset":41520,"ancestor_record_offset":41304,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":552,"record_offset":41612,"parent_record_offset":41548,"ancestor_record_offset":41304,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":552,"record_offset":42424,"parent_record_offset":42384,"ancestor_record_offset":41304,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":552,"record_offset":43664,"parent_record_offset":43596,"ancestor_record_offset":41304,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":552,"record_offset":41776,"parent_record_offset":41716,"ancestor_record_offset":41304,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":552,"record_offset":43152,"parent_record_offset":43108,"ancestor_record_offset":41304,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":552,"record_offset":41796,"parent_record_offset":41776,"ancestor_record_offset":41304,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":552,"record_offset":41836,"parent_record_offset":41664,"ancestor_record_offset":41304,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":552,"record_offset":42324,"parent_record_offset":41520,"ancestor_record_offset":41304,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":552,"record_offset":42788,"parent_record_offset":42744,"ancestor_record_offset":41304,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":552,"record_offset":42508,"parent_record_offset":42480,"ancestor_record_offset":41304,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":552,"record_offset":42536,"parent_record_offset":42508,"ancestor_record_offset":41304,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":552,"record_offset":41716,"parent_record_offset":41688,"ancestor_record_offset":41304,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":552,"record_offset":42088,"parent_record_offset":42064,"ancestor_record_offset":41304,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":552,"record_offset":41748,"parent_record_offset":41716,"ancestor_record_offset":41304,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":552,"record_offset":42064,"parent_record_offset":41884,"ancestor_record_offset":41304,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":552,"record_offset":42452,"parent_record_offset":42424,"ancestor_record_offset":41304,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":552,"record_offset":42656,"parent_record_offset":42632,"ancestor_record_offset":41304,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":552,"record_offset":42680,"parent_record_offset":42656,"ancestor_record_offset":41304,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":552,"record_offset":42384,"parent_record_offset":42356,"ancestor_record_offset":41304,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":552,"record_offset":43292,"parent_record_offset":43260,"ancestor_record_offset":41304,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":552,"record_offset":43636,"parent_record_offset":43596,"ancestor_record_offset":41304,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":552,"record_offset":43692,"parent_record_offset":43596,"ancestor_record_offset":41304,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00552.jsonl.gz"}
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
// 完整结构及原始类型记录: restir_di.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: restir_di.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/restir_di.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: restir_di.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":552,"record_offset":42820,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00552.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":552,"record_offset":43060,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00552.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":552,"record_offset":43108,"parent_record_offset":41304,"ancestor_record_offset":41304,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00552.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5163493,"position_among_mapped_render_targets":46,"mapped_render_target_count":59,"previous_callsite":5163457,"next_callsite":5163523,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/bin_lights.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: restir_di.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights
// 信息 ID: a6acb2ec23e357de17813940
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec9e5 -> 0xa6fec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
