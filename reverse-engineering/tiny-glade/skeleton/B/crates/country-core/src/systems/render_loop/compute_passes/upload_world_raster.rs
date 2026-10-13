// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 847c602e4ede5c103eb9ba626ae21924d75d9c472062fbdf1012f392d562ac0e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: upload_world_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 173
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":173,"record_offset":8712,"rva":4038736,"va":5372747856}
// 编译实例 RVAs: 0x3da050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3131820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e85436d0ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5372747927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140363db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-484628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e8ec9af8ff","size":5,"soft_fail":false,"target_va":5372263856,"va":5372748479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t741989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e865520b00","size":5,"soft_fail":false,"target_va":5373490544,"va":5372748550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405876a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1758367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e89fd41a00","size":5,"soft_fail":false,"target_va":5374506656,"va":5372748284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140588820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1762825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e809e61a00","size":5,"soft_fail":false,"target_va":5374511136,"va":5372748306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2289947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e81bf12200","size":5,"soft_fail":false,"target_va":5375038752,"va":5372748800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2332876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e8cc982300","size":5,"soft_fail":false,"target_va":5375081504,"va":5372748623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2768006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e8863c2a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5372748021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6813563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e87bf76700","size":5,"soft_fail":false,"target_va":5379561616,"va":5372748048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39098662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e826995402","size":5,"soft_fail":false,"target_va":5411847168,"va":5372748501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10091960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e8b8fd9900","size":5,"soft_fail":false,"target_va":5382840208,"va":5372748243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38793115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e89bef4f02","size":5,"soft_fail":false,"target_va":5411541776,"va":5372748656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38793144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e8b8ef4f02","size":5,"soft_fail":false,"target_va":5411541904,"va":5372748755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5851466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e84a495900","size":5,"soft_fail":false,"target_va":5378600064,"va":5372748593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9098183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372747856,"raw_hex":"e8c7d38a00","size":5,"soft_fail":false,"target_va":5381846624,"va":5372748436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":10036,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":9952,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":173,"record_offset":10576,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":173,"record_offset":10204,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":10348,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":10232,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":10276,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":173,"record_offset":10008,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":5784,"source_line":547,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":173,"record_offset":9000,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":8712,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":10500,"parent_record_offset":10444,"ancestor_record_offset":8712,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":10084,"parent_record_offset":10060,"ancestor_record_offset":8712,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":8712,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":10472,"parent_record_offset":10444,"ancestor_record_offset":8712,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":8712,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":8712,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":8712,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":8712,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":8712,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":9408,"parent_record_offset":9368,"ancestor_record_offset":8712,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":8712,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":8712,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":9756,"parent_record_offset":9732,"ancestor_record_offset":8712,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":8712,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":9680,"parent_record_offset":9648,"ancestor_record_offset":8712,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":9624,"parent_record_offset":9092,"ancestor_record_offset":8712,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":8712,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":9124,"parent_record_offset":9092,"ancestor_record_offset":8712,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":9708,"parent_record_offset":9648,"ancestor_record_offset":8712,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":8712,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":9148,"parent_record_offset":9124,"ancestor_record_offset":8712,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":9344,"parent_record_offset":9320,"ancestor_record_offset":8712,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":173,"record_offset":9824,"parent_record_offset":9800,"ancestor_record_offset":8712,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":9800,"parent_record_offset":9092,"ancestor_record_offset":8712,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":9056,"parent_record_offset":9028,"ancestor_record_offset":8712,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":9028,"parent_record_offset":9000,"ancestor_record_offset":8712,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":9092,"parent_record_offset":9028,"ancestor_record_offset":8712,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":173,"record_offset":9260,"parent_record_offset":9200,"ancestor_record_offset":8712,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":10316,"parent_record_offset":10276,"ancestor_record_offset":8712,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":9280,"parent_record_offset":9260,"ancestor_record_offset":8712,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":9320,"parent_record_offset":9148,"ancestor_record_offset":8712,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":8712,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":9920,"parent_record_offset":9000,"ancestor_record_offset":8712,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":8712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":173,"record_offset":9200,"parent_record_offset":9172,"ancestor_record_offset":8712,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":9572,"parent_record_offset":9548,"ancestor_record_offset":8712,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":9232,"parent_record_offset":9200,"ancestor_record_offset":8712,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":9548,"parent_record_offset":9368,"ancestor_record_offset":8712,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":8712,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":9872,"parent_record_offset":9848,"ancestor_record_offset":8712,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":173,"record_offset":10836,"rva":4040048,"va":5372749168}
// 编译实例 RVAs: 0x3da570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3133132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e83431d0ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5372749239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140363db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-485940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e8cc95f8ff","size":5,"soft_fail":false,"target_va":5372263856,"va":5372749791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t740677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e8454d0b00","size":5,"soft_fail":false,"target_va":5373490544,"va":5372749862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405876a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1757055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e87fcf1a00","size":5,"soft_fail":false,"target_va":5374506656,"va":5372749596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140588650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1761049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e819df1a00","size":5,"soft_fail":false,"target_va":5374510672,"va":5372749618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2288635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e8fbeb2200","size":5,"soft_fail":false,"target_va":5375038752,"va":5372750112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2331564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e8ac932300","size":5,"soft_fail":false,"target_va":5375081504,"va":5372749935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2766694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e866372a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5372749333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6812251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e85bf26700","size":5,"soft_fail":false,"target_va":5379561616,"va":5372749360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39097350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e806945402","size":5,"soft_fail":false,"target_va":5411847168,"va":5372749813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10090648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e898f89900","size":5,"soft_fail":false,"target_va":5382840208,"va":5372749555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38791803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e87bea4f02","size":5,"soft_fail":false,"target_va":5411541776,"va":5372749968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38791832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e898ea4f02","size":5,"soft_fail":false,"target_va":5411541904,"va":5372750067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5850154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e82a445900","size":5,"soft_fail":false,"target_va":5378600064,"va":5372749905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9096871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749168,"raw_hex":"e8a7ce8a00","size":5,"soft_fail":false,"target_va":5381846624,"va":5372749748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":12160,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":12076,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":173,"record_offset":12700,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":173,"record_offset":12328,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":12472,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":12356,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":12400,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":173,"record_offset":12132,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":5784,"source_line":547,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":173,"record_offset":11124,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":10836,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":12624,"parent_record_offset":12568,"ancestor_record_offset":10836,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":12208,"parent_record_offset":12184,"ancestor_record_offset":10836,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":10836,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":12596,"parent_record_offset":12568,"ancestor_record_offset":10836,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":10836,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":11628,"parent_record_offset":11604,"ancestor_record_offset":10836,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":10836,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":12544,"parent_record_offset":12520,"ancestor_record_offset":10836,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":10836,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":11532,"parent_record_offset":11492,"ancestor_record_offset":10836,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":10836,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":10836,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":10836,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":10836,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":11804,"parent_record_offset":11772,"ancestor_record_offset":10836,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":11748,"parent_record_offset":11216,"ancestor_record_offset":10836,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":10836,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":11248,"parent_record_offset":11216,"ancestor_record_offset":10836,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":11832,"parent_record_offset":11772,"ancestor_record_offset":10836,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":10836,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":11272,"parent_record_offset":11248,"ancestor_record_offset":10836,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10836,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":173,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":10836,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":11924,"parent_record_offset":11216,"ancestor_record_offset":10836,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":11180,"parent_record_offset":11152,"ancestor_record_offset":10836,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":11152,"parent_record_offset":11124,"ancestor_record_offset":10836,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":11216,"parent_record_offset":11152,"ancestor_record_offset":10836,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":173,"record_offset":11384,"parent_record_offset":11324,"ancestor_record_offset":10836,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":12440,"parent_record_offset":12400,"ancestor_record_offset":10836,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":11404,"parent_record_offset":11384,"ancestor_record_offset":10836,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":11444,"parent_record_offset":11272,"ancestor_record_offset":10836,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":12184,"parent_record_offset":12160,"ancestor_record_offset":10836,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":12044,"parent_record_offset":11124,"ancestor_record_offset":10836,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":10836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":173,"record_offset":11324,"parent_record_offset":11296,"ancestor_record_offset":10836,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":11696,"parent_record_offset":11672,"ancestor_record_offset":10836,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":11356,"parent_record_offset":11324,"ancestor_record_offset":10836,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":11672,"parent_record_offset":11492,"ancestor_record_offset":10836,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":10836,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":10836,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":173,"record_offset":2356,"rva":4034800,"va":5372743920}
// 编译实例 RVAs: 0x3d90f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3127884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e8b445d0ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5372743991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140363db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-480692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e84caaf8ff","size":5,"soft_fail":false,"target_va":5372263856,"va":5372744543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t745925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e8c5610b00","size":5,"soft_fail":false,"target_va":5373490544,"va":5372744614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405876a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1762303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e8ffe31a00","size":5,"soft_fail":false,"target_va":5374506656,"va":5372744348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405889f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1767225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e839f71a00","size":5,"soft_fail":false,"target_va":5374511600,"va":5372744370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2293883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e87b002300","size":5,"soft_fail":false,"target_va":5375038752,"va":5372744864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2336812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e82ca82300","size":5,"soft_fail":false,"target_va":5375081504,"va":5372744687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2771942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e8e64b2a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5372744085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6817499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e8db066800","size":5,"soft_fail":false,"target_va":5379561616,"va":5372744112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39102598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e886a85402","size":5,"soft_fail":false,"target_va":5411847168,"va":5372744565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10095896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e8180d9a00","size":5,"soft_fail":false,"target_va":5382840208,"va":5372744307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38797051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e8fbfe4f02","size":5,"soft_fail":false,"target_va":5411541776,"va":5372744720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38797080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e818ff4f02","size":5,"soft_fail":false,"target_va":5411541904,"va":5372744819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5855402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e8aa585900","size":5,"soft_fail":false,"target_va":5378600064,"va":5372744657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9102119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372743920,"raw_hex":"e827e38a00","size":5,"soft_fail":false,"target_va":5381846624,"va":5372744500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":3680,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":3596,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":173,"record_offset":4220,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":173,"record_offset":3848,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":3992,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":3876,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":3920,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":173,"record_offset":3652,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":5784,"source_line":547,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":173,"record_offset":2644,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":2356,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":4144,"parent_record_offset":4088,"ancestor_record_offset":2356,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":3728,"parent_record_offset":3704,"ancestor_record_offset":2356,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2356,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":4116,"parent_record_offset":4088,"ancestor_record_offset":2356,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":2356,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2356,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2356,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":2356,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2356,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":3052,"parent_record_offset":3012,"ancestor_record_offset":2356,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":2356,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":2356,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":2356,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":2356,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":3324,"parent_record_offset":3292,"ancestor_record_offset":2356,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":3268,"parent_record_offset":2736,"ancestor_record_offset":2356,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":3292,"parent_record_offset":3268,"ancestor_record_offset":2356,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":2356,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":3352,"parent_record_offset":3292,"ancestor_record_offset":2356,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":2356,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":2356,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":2356,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":173,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2356,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":3444,"parent_record_offset":2736,"ancestor_record_offset":2356,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":2700,"parent_record_offset":2672,"ancestor_record_offset":2356,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":2672,"parent_record_offset":2644,"ancestor_record_offset":2356,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":2736,"parent_record_offset":2672,"ancestor_record_offset":2356,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":173,"record_offset":2904,"parent_record_offset":2844,"ancestor_record_offset":2356,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":3960,"parent_record_offset":3920,"ancestor_record_offset":2356,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":2924,"parent_record_offset":2904,"ancestor_record_offset":2356,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":2964,"parent_record_offset":2792,"ancestor_record_offset":2356,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":3704,"parent_record_offset":3680,"ancestor_record_offset":2356,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":3564,"parent_record_offset":2644,"ancestor_record_offset":2356,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":2356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":173,"record_offset":2844,"parent_record_offset":2816,"ancestor_record_offset":2356,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":2356,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":2876,"parent_record_offset":2844,"ancestor_record_offset":2356,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":3192,"parent_record_offset":3012,"ancestor_record_offset":2356,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":2356,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2356,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":173,"record_offset":6596,"rva":4037424,"va":5372746544}
// 编译实例 RVAs: 0x3d9b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3130508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8743bd0ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5372746615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140363db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-483316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e80ca0f8ff","size":5,"soft_fail":false,"target_va":5372263856,"va":5372747167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t743301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e885570b00","size":5,"soft_fail":false,"target_va":5373490544,"va":5372747238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405876a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1759679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8bfd91a00","size":5,"soft_fail":false,"target_va":5374506656,"va":5372746972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140588f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1765993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e869f21a00","size":5,"soft_fail":false,"target_va":5374512992,"va":5372746994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2291259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e83bf62200","size":5,"soft_fail":false,"target_va":5375038752,"va":5372747488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2334188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8ec9d2300","size":5,"soft_fail":false,"target_va":5375081504,"va":5372747311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2769318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8a6412a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5372746709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6814875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e89bfc6700","size":5,"soft_fail":false,"target_va":5379561616,"va":5372746736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39099974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8469e5402","size":5,"soft_fail":false,"target_va":5411847168,"va":5372747189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10093272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8d8029a00","size":5,"soft_fail":false,"target_va":5382840208,"va":5372746931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38794427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8bbf44f02","size":5,"soft_fail":false,"target_va":5411541776,"va":5372747344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38794456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8d8f44f02","size":5,"soft_fail":false,"target_va":5411541904,"va":5372747443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5852778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e86a4e5900","size":5,"soft_fail":false,"target_va":5378600064,"va":5372747281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9099495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372746544,"raw_hex":"e8e7d88a00","size":5,"soft_fail":false,"target_va":5381846624,"va":5372747124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":7912,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":7828,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":173,"record_offset":8452,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":173,"record_offset":8080,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":8224,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":8108,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":8152,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":173,"record_offset":7884,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":5784,"source_line":547,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":173,"record_offset":6876,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":6596,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":8376,"parent_record_offset":8320,"ancestor_record_offset":6596,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":6596,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":7332,"parent_record_offset":7308,"ancestor_record_offset":6596,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":8348,"parent_record_offset":8320,"ancestor_record_offset":6596,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":8320,"parent_record_offset":8296,"ancestor_record_offset":6596,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":6596,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":7356,"parent_record_offset":7332,"ancestor_record_offset":6596,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":8296,"parent_record_offset":8272,"ancestor_record_offset":6596,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":6596,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":7284,"parent_record_offset":7244,"ancestor_record_offset":6596,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":6596,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":6596,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":6596,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":6596,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":7556,"parent_record_offset":7524,"ancestor_record_offset":6596,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":7500,"parent_record_offset":6968,"ancestor_record_offset":6596,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":6596,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":7000,"parent_record_offset":6968,"ancestor_record_offset":6596,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":7584,"parent_record_offset":7524,"ancestor_record_offset":6596,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":6596,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":6596,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":6596,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":173,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":6596,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":7676,"parent_record_offset":6968,"ancestor_record_offset":6596,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":6932,"parent_record_offset":6904,"ancestor_record_offset":6596,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":6596,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":6968,"parent_record_offset":6904,"ancestor_record_offset":6596,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":173,"record_offset":7136,"parent_record_offset":7076,"ancestor_record_offset":6596,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":8192,"parent_record_offset":8152,"ancestor_record_offset":6596,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":7156,"parent_record_offset":7136,"ancestor_record_offset":6596,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":7196,"parent_record_offset":7024,"ancestor_record_offset":6596,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":6596,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":7796,"parent_record_offset":6876,"ancestor_record_offset":6596,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":6596,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":173,"record_offset":7076,"parent_record_offset":7048,"ancestor_record_offset":6596,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":6596,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":7108,"parent_record_offset":7076,"ancestor_record_offset":6596,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":7424,"parent_record_offset":7244,"ancestor_record_offset":6596,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":6596,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":7748,"parent_record_offset":7724,"ancestor_record_offset":6596,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":173,"record_offset":240,"rva":4033488,"va":5372742608}
// 编译实例 RVAs: 0x3d8bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3126572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e8d44ad0ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5372742679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140363db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-479380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e86caff8ff","size":5,"soft_fail":false,"target_va":5372263856,"va":5372743231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t747237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e8e5660b00","size":5,"soft_fail":false,"target_va":5373490544,"va":5372743302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405876a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1763615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e81fe91a00","size":5,"soft_fail":false,"target_va":5374506656,"va":5372743036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140588bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1769001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e829fe1a00","size":5,"soft_fail":false,"target_va":5374512064,"va":5372743058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2295195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e89b052300","size":5,"soft_fail":false,"target_va":5375038752,"va":5372743552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2338124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e84cad2300","size":5,"soft_fail":false,"target_va":5375081504,"va":5372743375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2773254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e806512a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5372742773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6818811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e8fb0b6800","size":5,"soft_fail":false,"target_va":5379561616,"va":5372742800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39103910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e8a6ad5402","size":5,"soft_fail":false,"target_va":5411847168,"va":5372743253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10097208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e838129a00","size":5,"soft_fail":false,"target_va":5382840208,"va":5372742995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38798363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e81b045002","size":5,"soft_fail":false,"target_va":5411541776,"va":5372743408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38798392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e838045002","size":5,"soft_fail":false,"target_va":5411541904,"va":5372743507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5856714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e8ca5d5900","size":5,"soft_fail":false,"target_va":5378600064,"va":5372743345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9103431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372742608,"raw_hex":"e847e88a00","size":5,"soft_fail":false,"target_va":5381846624,"va":5372743188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":1556,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":1472,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":173,"record_offset":2096,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":173,"record_offset":1724,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":1868,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":1752,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":1796,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":173,"record_offset":1528,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5784,"source_line":547,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":173,"record_offset":520,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":240,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":2020,"parent_record_offset":1964,"ancestor_record_offset":240,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":240,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":240,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":240,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":240,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":240,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":1000,"parent_record_offset":976,"ancestor_record_offset":240,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":240,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":240,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":928,"parent_record_offset":888,"ancestor_record_offset":240,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":240,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":240,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":240,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":240,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":1200,"parent_record_offset":1168,"ancestor_record_offset":240,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":1144,"parent_record_offset":612,"ancestor_record_offset":240,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":1168,"parent_record_offset":1144,"ancestor_record_offset":240,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":644,"parent_record_offset":612,"ancestor_record_offset":240,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":1228,"parent_record_offset":1168,"ancestor_record_offset":240,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":240,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":240,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":240,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":173,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":240,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":1320,"parent_record_offset":612,"ancestor_record_offset":240,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":240,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":548,"parent_record_offset":520,"ancestor_record_offset":240,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":612,"parent_record_offset":548,"ancestor_record_offset":240,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":173,"record_offset":780,"parent_record_offset":720,"ancestor_record_offset":240,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":1836,"parent_record_offset":1796,"ancestor_record_offset":240,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":800,"parent_record_offset":780,"ancestor_record_offset":240,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":840,"parent_record_offset":668,"ancestor_record_offset":240,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":240,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":1440,"parent_record_offset":520,"ancestor_record_offset":240,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":173,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":240,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":240,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":752,"parent_record_offset":720,"ancestor_record_offset":240,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":1068,"parent_record_offset":888,"ancestor_record_offset":240,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":240,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":240,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":173,"record_offset":4480,"rva":4036112,"va":5372745232}
// 编译实例 RVAs: 0x3d9610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3129196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e89440d0ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5372745303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140363db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-482004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e82ca5f8ff","size":5,"soft_fail":false,"target_va":5372263856,"va":5372745855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t744613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e8a55c0b00","size":5,"soft_fail":false,"target_va":5373490544,"va":5372745926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405876a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1760991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e8dfde1a00","size":5,"soft_fail":false,"target_va":5374506656,"va":5372745660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140588d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1766841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e8b9f51a00","size":5,"soft_fail":false,"target_va":5374512528,"va":5372745682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2292571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e85bfb2200","size":5,"soft_fail":false,"target_va":5375038752,"va":5372746176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2335500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e80ca32300","size":5,"soft_fail":false,"target_va":5375081504,"va":5372745999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2770630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e8c6462a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5372745397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6816187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e8bb016800","size":5,"soft_fail":false,"target_va":5379561616,"va":5372745424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: upload_world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39101286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e866a35402","size":5,"soft_fail":false,"target_va":5411847168,"va":5372745877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10094584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e8f8079a00","size":5,"soft_fail":false,"target_va":5382840208,"va":5372745619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38795739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e8dbf94f02","size":5,"soft_fail":false,"target_va":5411541776,"va":5372746032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38795768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e8f8f94f02","size":5,"soft_fail":false,"target_va":5411541904,"va":5372746131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5854090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e88a535900","size":5,"soft_fail":false,"target_va":5378600064,"va":5372745969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection::BindingNameHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9100807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372745232,"raw_hex":"e807de8a00","size":5,"soft_fail":false,"target_va":5381846624,"va":5372745812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":5796,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":5712,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":173,"record_offset":6336,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":173,"record_offset":5964,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":6108,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":5992,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":173,"record_offset":6036,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::on_uploaded_to_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":173,"record_offset":5768,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":5784,"source_line":547,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":173,"record_offset":4760,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":6284,"parent_record_offset":6260,"ancestor_record_offset":4480,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":6260,"parent_record_offset":6204,"ancestor_record_offset":4480,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":173,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":4480,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":4480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":6232,"parent_record_offset":6204,"ancestor_record_offset":4480,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":6204,"parent_record_offset":6180,"ancestor_record_offset":4480,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":5264,"parent_record_offset":5240,"ancestor_record_offset":4480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":5240,"parent_record_offset":5216,"ancestor_record_offset":4480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":4480,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":4480,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":5168,"parent_record_offset":5128,"ancestor_record_offset":4480,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":173,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":4480,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":173,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":4480,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":4480,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":4480,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":173,"record_offset":5440,"parent_record_offset":5408,"ancestor_record_offset":4480,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":5384,"parent_record_offset":4852,"ancestor_record_offset":4480,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":4480,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":4884,"parent_record_offset":4852,"ancestor_record_offset":4480,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":173,"record_offset":5468,"parent_record_offset":5408,"ancestor_record_offset":4480,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":4480,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":4480,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":173,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4480,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":173,"record_offset":5584,"parent_record_offset":5560,"ancestor_record_offset":4480,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":5560,"parent_record_offset":4852,"ancestor_record_offset":4480,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":4816,"parent_record_offset":4788,"ancestor_record_offset":4480,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":4788,"parent_record_offset":4760,"ancestor_record_offset":4480,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":173,"record_offset":4852,"parent_record_offset":4788,"ancestor_record_offset":4480,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":173,"record_offset":5020,"parent_record_offset":4960,"ancestor_record_offset":4480,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":6076,"parent_record_offset":6036,"ancestor_record_offset":4480,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":5040,"parent_record_offset":5020,"ancestor_record_offset":4480,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":5080,"parent_record_offset":4908,"ancestor_record_offset":4480,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":4480,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":173,"record_offset":5680,"parent_record_offset":4760,"ancestor_record_offset":4480,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":4480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":173,"record_offset":4960,"parent_record_offset":4932,"ancestor_record_offset":4480,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":4480,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":4992,"parent_record_offset":4960,"ancestor_record_offset":4480,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":173,"record_offset":5308,"parent_record_offset":5128,"ancestor_record_offset":4480,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":5608,"parent_record_offset":5584,"ancestor_record_offset":4480,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":4480,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00173.jsonl.gz"}
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
// 完整结构及原始类型记录: upload_world_raster.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: upload_world_raster.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: upload_world_raster.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":10232,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":10276,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":12356,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":12400,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":3876,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":3920,"parent_record_offset":2356,"ancestor_record_offset":2356,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":8108,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":8152,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":1752,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":1796,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":5992,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":173,"record_offset":6036,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00173.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161637,"position_among_mapped_render_targets":1,"mapped_render_target_count":59,"previous_callsite":null,"next_callsite":5161651,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161651,"position_among_mapped_render_targets":2,"mapped_render_target_count":59,"previous_callsite":5161637,"next_callsite":5161665,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161665,"position_among_mapped_render_targets":3,"mapped_render_target_count":59,"previous_callsite":5161651,"next_callsite":5161679,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161679,"position_among_mapped_render_targets":4,"mapped_render_target_count":59,"previous_callsite":5161665,"next_callsite":5161693,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161693,"position_among_mapped_render_targets":5,"mapped_render_target_count":59,"previous_callsite":5161679,"next_callsite":5161707,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161707,"position_among_mapped_render_targets":6,"mapped_render_target_count":59,"previous_callsite":5161693,"next_callsite":5161718,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: upload_world_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>
// 信息 ID: e605298b76567046f1088c60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::Cobblestone1Raster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec2b3 -> 0x3da050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>
// 信息 ID: 73f7e3ee408ba42e704de255
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::Cobblestone2Raster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec2c1 -> 0x3da570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>
// 信息 ID: d25ff36d006ddb5ef31ddc23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::Cobblestone3Raster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec2cf -> 0x3d90f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>
// 信息 ID: be12c7b2c99e7d7c276d8ce3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::GardenRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec2eb -> 0x3d9b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>
// 信息 ID: 66f0652845a4e559e684ee7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::PathRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec2a5 -> 0x3d8bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>
// 信息 ID: 57e8cc196012938ffa4b76a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture -> ["country_core::resources::world_raster_defs::WaterRaster"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec2dd -> 0x3d9610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
