// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/shader.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ab3f20cbb4b2cb70419e2e44546d60bd719559cb7edde96040d3ba5f2377b32c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shader.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 33, 475, 482, 487, 489, 495, 501, 509, 528, 529, 530, 531, 532, 572, 577, 693, 703, 1582, 1610, 1613, 1615, 1623, 1655, 1657, 1917, 1935, 1965, 1966
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use leekspin; // namespace candidate
//   package identities: [{"name":"leekspin","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":10540,"rva":23738144,"va":5392447264}
// 编译实例 RVAs: 0x16a3720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17366173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e86303f7fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5392447656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17368870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8daf8f6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392450369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087add0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14849709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e853691dff","size":5,"soft_fail":false,"target_va":5377600976,"va":5392450680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140881ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14823208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8d8d01dff","size":5,"soft_fail":false,"target_va":5377628896,"va":5392452099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c530
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11830638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8927a4bff","size":5,"soft_fail":false,"target_va":5380621616,"va":5392452249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130a7d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3772760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8a86ec6ff","size":5,"soft_fail":false,"target_va":5388675024,"va":5392447779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19089825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8a1492301","size":5,"soft_fail":false,"target_va":5411540992,"va":5392451162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16869393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8ef97fefe","size":5,"soft_fail":false,"target_va":5375579088,"va":5392448476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19089958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8264a2301","size":5,"soft_fail":false,"target_va":5411540963,"va":5392451000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19090946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8024e2301","size":5,"soft_fail":false,"target_va":5411541808,"va":5392450857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19090585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8994c2301","size":5,"soft_fail":false,"target_va":5411541776,"va":5392451186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19030859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e84b632201","size":5,"soft_fail":false,"target_va":5411481622,"va":5392450758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19090929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8f14d2301","size":5,"soft_fail":false,"target_va":5411541904,"va":5392450970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19091747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e823512301","size":5,"soft_fail":false,"target_va":5411542880,"va":5392451128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::leek::create_raster_pipeline
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11275465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e837f353ff","size":5,"soft_fail":false,"target_va":5381171872,"va":5392447332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::create_descriptor_set_layouts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11261687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8092954ff","size":5,"soft_fail":false,"target_va":5381185696,"va":5392447378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::create_descriptor_set_states
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11263205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e81b2354ff","size":5,"soft_fail":false,"target_va":5381187264,"va":5392450464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::get_vk_blend_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11261852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8642854ff","size":5,"soft_fail":false,"target_va":5381187568,"va":5392449415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::map_vertex_attribute_format
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11260688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8f02c54ff","size":5,"soft_fail":false,"target_va":5381187664,"va":5392448347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10590267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e83b98a100","size":5,"soft_fail":false,"target_va":5403038592,"va":5392448320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection_ty::Type::nbyte
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10591833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447264,"raw_hex":"e8599ea100","size":5,"soft_fail":false,"target_va":5403040256,"va":5392448418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*528(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392447264,"raw_hex":"ff9710020000","size":6,"soft_fail":false,"va":5392447618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":12748,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":11564,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":14728,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_graphics_pipelines
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":15720,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75193,"source_line":2150,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_pipeline_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":10968,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75163,"source_line":2219,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::GraphicsPipelineCreateInfo::stages
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":15692,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75192,"source_line":5033,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineColorBlendStateCreateInfo::attachments
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":15576,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75184,"source_line":4746,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineDynamicStateCreateInfo::dynamic_states
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":15632,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75188,"source_line":4812,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineLayoutCreateInfo::set_layouts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":10940,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75162,"source_line":5320,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineVertexInputStateCreateInfo::vertex_attribute_descriptions
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":14060,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75167,"source_line":4194,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineVertexInputStateCreateInfo::vertex_binding_descriptions
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":14088,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75168,"source_line":4185,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineViewportStateCreateInfo::scissor_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":14324,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75174,"source_line":4381,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineViewportStateCreateInfo::viewport_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":14296,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75173,"source_line":4370,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$216::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":14032,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75165,"source_line":4161,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$231::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":14268,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75171,"source_line":4346,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$247::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":15432,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75182,"source_line":4711,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$252::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":15604,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75186,"source_line":4791,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$263::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":15664,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75190,"source_line":4997,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$276::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":10912,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75160,"source_line":5291,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1657,"record_offset":14616,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1657,"record_offset":17176,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":11480,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":13636,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":13996,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":16016,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1657,"record_offset":17232,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1657,"record_offset":17204,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1657,"record_offset":11384,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1657,"record_offset":13864,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::cull_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14352,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75175,"source_line":624,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14464,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75177,"source_line":626,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14116,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":75119,"source_line":623,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":1657,"record_offset":11084,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":28564,"source_line":26,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1657,"record_offset":12372,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1657,"record_offset":12940,"parent_record_offset":10540,"ancestor_record_offset":10540,"inlinee":11891,"source_line":2551,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":11248,"parent_record_offset":11224,"ancestor_record_offset":10540,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":11272,"parent_record_offset":11248,"ancestor_record_offset":10540,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":16136,"parent_record_offset":16112,"ancestor_record_offset":10540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":11224,"parent_record_offset":11200,"ancestor_record_offset":10540,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":10540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1657,"record_offset":11140,"parent_record_offset":11112,"ancestor_record_offset":10540,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1657,"record_offset":11168,"parent_record_offset":11140,"ancestor_record_offset":10540,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1657,"record_offset":11200,"parent_record_offset":11168,"ancestor_record_offset":10540,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1657,"record_offset":11320,"parent_record_offset":11112,"ancestor_record_offset":10540,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1657,"record_offset":11112,"parent_record_offset":11084,"ancestor_record_offset":10540,"inlinee":21073,"source_line":1695,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":12820,"parent_record_offset":12788,"ancestor_record_offset":10540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":11628,"parent_record_offset":11596,"ancestor_record_offset":10540,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":16284,"parent_record_offset":16256,"ancestor_record_offset":10540,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":16088,"parent_record_offset":16064,"ancestor_record_offset":10540,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":10540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":10540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":11688,"parent_record_offset":11656,"ancestor_record_offset":10540,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":11656,"parent_record_offset":11628,"ancestor_record_offset":10540,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":10540,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":11724,"parent_record_offset":11688,"ancestor_record_offset":10540,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1657,"record_offset":18968,"parent_record_offset":18932,"ancestor_record_offset":10540,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1657,"record_offset":18780,"parent_record_offset":18756,"ancestor_record_offset":10540,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":12788,"parent_record_offset":12748,"ancestor_record_offset":10540,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":15160,"parent_record_offset":14760,"ancestor_record_offset":10540,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":15764,"parent_record_offset":15720,"ancestor_record_offset":10540,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":11596,"parent_record_offset":11564,"ancestor_record_offset":10540,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":11504,"parent_record_offset":11480,"ancestor_record_offset":10540,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1657,"record_offset":14760,"parent_record_offset":14728,"ancestor_record_offset":10540,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1657,"record_offset":10996,"parent_record_offset":10968,"ancestor_record_offset":10540,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1657,"record_offset":11020,"parent_record_offset":10996,"ancestor_record_offset":10540,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1657,"record_offset":11044,"parent_record_offset":11020,"ancestor_record_offset":10540,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1657,"record_offset":11748,"parent_record_offset":11724,"ancestor_record_offset":10540,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1657,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":10540,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1657,"record_offset":15288,"parent_record_offset":15252,"ancestor_record_offset":10540,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$54::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":1657,"record_offset":14640,"parent_record_offset":14616,"ancestor_record_offset":10540,"inlinee":75180,"source_line":77,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1657,"record_offset":12600,"parent_record_offset":12572,"ancestor_record_offset":10540,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1657,"record_offset":12660,"parent_record_offset":12628,"ancestor_record_offset":10540,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":15532,"parent_record_offset":15508,"ancestor_record_offset":10540,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1657,"record_offset":15252,"parent_record_offset":15216,"ancestor_record_offset":10540,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1657,"record_offset":15216,"parent_record_offset":15160,"ancestor_record_offset":10540,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1657,"record_offset":11528,"parent_record_offset":11504,"ancestor_record_offset":10540,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1657,"record_offset":11344,"parent_record_offset":11320,"ancestor_record_offset":10540,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1657,"record_offset":15484,"parent_record_offset":15460,"ancestor_record_offset":10540,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":15460,"parent_record_offset":15432,"ancestor_record_offset":10540,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":11796,"parent_record_offset":11772,"ancestor_record_offset":10540,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":11820,"parent_record_offset":11796,"ancestor_record_offset":10540,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":16312,"parent_record_offset":16284,"ancestor_record_offset":10540,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1657,"record_offset":13240,"parent_record_offset":13208,"ancestor_record_offset":10540,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1657,"record_offset":13208,"parent_record_offset":13176,"ancestor_record_offset":10540,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1657,"record_offset":12696,"parent_record_offset":12544,"ancestor_record_offset":10540,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":16040,"parent_record_offset":16016,"ancestor_record_offset":10540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":10540,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":10540,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":15508,"parent_record_offset":15484,"ancestor_record_offset":10540,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":19000,"parent_record_offset":18968,"ancestor_record_offset":10540,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":12908,"parent_record_offset":12748,"ancestor_record_offset":10540,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1657,"record_offset":14228,"parent_record_offset":14116,"ancestor_record_offset":10540,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1657,"record_offset":11412,"parent_record_offset":11384,"ancestor_record_offset":10540,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":18816,"parent_record_offset":18780,"ancestor_record_offset":10540,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":18840,"parent_record_offset":18816,"ancestor_record_offset":10540,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":18872,"parent_record_offset":18780,"ancestor_record_offset":10540,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":13136,"parent_record_offset":13096,"ancestor_record_offset":10540,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":13176,"parent_record_offset":13136,"ancestor_record_offset":10540,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":13284,"parent_record_offset":13096,"ancestor_record_offset":10540,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":13096,"parent_record_offset":13044,"ancestor_record_offset":10540,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":12572,"parent_record_offset":12544,"ancestor_record_offset":10540,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":12628,"parent_record_offset":12572,"ancestor_record_offset":10540,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":13376,"parent_record_offset":13044,"ancestor_record_offset":10540,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":12428,"parent_record_offset":12400,"ancestor_record_offset":10540,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":12988,"parent_record_offset":12964,"ancestor_record_offset":10540,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":13476,"parent_record_offset":13044,"ancestor_record_offset":10540,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":12544,"parent_record_offset":12484,"ancestor_record_offset":10540,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":13044,"parent_record_offset":13016,"ancestor_record_offset":10540,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":12456,"parent_record_offset":12428,"ancestor_record_offset":10540,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":12516,"parent_record_offset":12484,"ancestor_record_offset":10540,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":12484,"parent_record_offset":12456,"ancestor_record_offset":10540,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":13016,"parent_record_offset":12988,"ancestor_record_offset":10540,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1657,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":10540,"inlinee":29877,"source_line":23,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1657,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":10540,"inlinee":29886,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1657,"record_offset":13908,"parent_record_offset":13864,"ancestor_record_offset":10540,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1657,"record_offset":13964,"parent_record_offset":13864,"ancestor_record_offset":10540,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1657,"record_offset":13936,"parent_record_offset":13864,"ancestor_record_offset":10540,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::cull_mode_or_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":10540,"inlinee":75176,"source_line":624,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode_or_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14492,"parent_record_offset":14464,"ancestor_record_offset":10540,"inlinee":75178,"source_line":626,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::topology_or_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14144,"parent_record_offset":14116,"ancestor_record_offset":10540,"inlinee":75169,"source_line":623,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1657,"record_offset":12400,"parent_record_offset":12372,"ancestor_record_offset":10540,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1657,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":10540,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1965,"record_offset":6108,"rva":25960784,"va":5394669904}
// 编译实例 RVAs: 0x18c2150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19588573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e8231ad5fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394670056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19589682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e8ce15d5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394671181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfc5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13393678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e8f2a033ff","size":5,"soft_fail":false,"target_va":5381277104,"va":5394670777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17177044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e8d4190601","size":5,"soft_fail":false,"target_va":5411847200,"va":5394670151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16869643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e80b690101","size":5,"soft_fail":false,"target_va":5411540992,"va":5394671344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-20313659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e8c509cafe","size":5,"soft_fail":false,"target_va":5374356912,"va":5394670566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16869577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e8c9680101","size":5,"soft_fail":false,"target_va":5411540963,"va":5394671381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16870988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e84c6e0101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394671303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16870617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e8d96c0101","size":5,"soft_fail":false,"target_va":5411541904,"va":5394671282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16871451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394669904,"raw_hex":"e81b700101","size":5,"soft_fail":false,"target_va":5411542880,"va":5394671424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*560(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394669904,"raw_hex":"ff9030020000","size":6,"soft_fail":false,"va":5394671116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":7768,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":6708,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":7144,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":8220,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_set_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1965,"record_offset":10212,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":39396,"source_line":1561,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$1034::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1965,"record_offset":8784,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":86886,"source_line":45,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetLayoutBindingFlagsCreateInfo::binding_flags
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1965,"record_offset":9980,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":86890,"source_line":21154,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetLayoutCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1965,"record_offset":10008,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":86854,"source_line":3592,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1965,"record_offset":10560,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1965,"record_offset":8016,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1965,"record_offset":6532,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":30977,"source_line":174,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":10328,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1965,"record_offset":10588,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1965,"record_offset":8316,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":8791,"source_line":1391,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":1965,"record_offset":8132,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":39772,"source_line":690,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":1965,"record_offset":9208,"parent_record_offset":6108,"ancestor_record_offset":6108,"inlinee":28563,"source_line":30,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":6108,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":6108,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":6108,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":9596,"parent_record_offset":9568,"ancestor_record_offset":6108,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":7040,"parent_record_offset":6832,"ancestor_record_offset":6108,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":10488,"parent_record_offset":10400,"ancestor_record_offset":6108,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1965,"record_offset":9568,"parent_record_offset":9208,"ancestor_record_offset":6108,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":7836,"parent_record_offset":7804,"ancestor_record_offset":6108,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":6772,"parent_record_offset":6740,"ancestor_record_offset":6108,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":10428,"parent_record_offset":10400,"ancestor_record_offset":6108,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":10400,"parent_record_offset":10376,"ancestor_record_offset":6108,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":6108,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":6108,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":6832,"parent_record_offset":6800,"ancestor_record_offset":6108,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":6800,"parent_record_offset":6772,"ancestor_record_offset":6108,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":10376,"parent_record_offset":10352,"ancestor_record_offset":6108,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":6868,"parent_record_offset":6832,"ancestor_record_offset":6108,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":7804,"parent_record_offset":7768,"ancestor_record_offset":6108,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":7568,"parent_record_offset":7172,"ancestor_record_offset":6108,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":6740,"parent_record_offset":6708,"ancestor_record_offset":6108,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1965,"record_offset":7172,"parent_record_offset":7144,"ancestor_record_offset":6108,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":9436,"parent_record_offset":9408,"ancestor_record_offset":6108,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":9520,"parent_record_offset":9492,"ancestor_record_offset":6108,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":9260,"parent_record_offset":9208,"ancestor_record_offset":6108,"inlinee":28486,"source_line":204,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":9492,"parent_record_offset":9464,"ancestor_record_offset":6108,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":9368,"parent_record_offset":9312,"ancestor_record_offset":6108,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":9408,"parent_record_offset":9208,"ancestor_record_offset":6108,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":9464,"parent_record_offset":9436,"ancestor_record_offset":6108,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":9312,"parent_record_offset":9284,"ancestor_record_offset":6108,"inlinee":28488,"source_line":54,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":6108,"inlinee":28487,"source_line":43,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$1033::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1965,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":6108,"inlinee":86888,"source_line":39,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1965,"record_offset":10240,"parent_record_offset":10212,"ancestor_record_offset":6108,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1965,"record_offset":10264,"parent_record_offset":10240,"ancestor_record_offset":6108,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1965,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":6108,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1965,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":6108,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1965,"record_offset":6892,"parent_record_offset":6868,"ancestor_record_offset":6108,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1965,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":6108,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1965,"record_offset":8188,"parent_record_offset":8132,"ancestor_record_offset":6108,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1965,"record_offset":7664,"parent_record_offset":7636,"ancestor_record_offset":6108,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1965,"record_offset":8400,"parent_record_offset":8372,"ancestor_record_offset":6108,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1965,"record_offset":6996,"parent_record_offset":6940,"ancestor_record_offset":6108,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1965,"record_offset":8040,"parent_record_offset":8016,"ancestor_record_offset":6108,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1965,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":6108,"inlinee":35970,"source_line":221,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":1965,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":6108,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1965,"record_offset":10084,"parent_record_offset":10060,"ancestor_record_offset":6108,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1965,"record_offset":10132,"parent_record_offset":10108,"ancestor_record_offset":6108,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1965,"record_offset":7636,"parent_record_offset":7608,"ancestor_record_offset":6108,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1965,"record_offset":7608,"parent_record_offset":7568,"ancestor_record_offset":6108,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1965,"record_offset":10036,"parent_record_offset":10008,"ancestor_record_offset":6108,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1965,"record_offset":10108,"parent_record_offset":10084,"ancestor_record_offset":6108,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1965,"record_offset":9752,"parent_record_offset":9208,"ancestor_record_offset":6108,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1965,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":6108,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1965,"record_offset":6968,"parent_record_offset":6940,"ancestor_record_offset":6108,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1965,"record_offset":8160,"parent_record_offset":8132,"ancestor_record_offset":6108,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1965,"record_offset":10456,"parent_record_offset":10428,"ancestor_record_offset":6108,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1965,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":6108,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":10352,"parent_record_offset":10328,"ancestor_record_offset":6108,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1965,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":6108,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1965,"record_offset":6612,"parent_record_offset":6580,"ancestor_record_offset":6108,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1965,"record_offset":8092,"parent_record_offset":8064,"ancestor_record_offset":6108,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":9704,"parent_record_offset":9676,"ancestor_record_offset":6108,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":7700,"parent_record_offset":7568,"ancestor_record_offset":6108,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1965,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":6108,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1965,"record_offset":8448,"parent_record_offset":8424,"ancestor_record_offset":6108,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1965,"record_offset":8372,"parent_record_offset":8344,"ancestor_record_offset":6108,"inlinee":5362,"source_line":2612,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1965,"record_offset":8248,"parent_record_offset":8220,"ancestor_record_offset":6108,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1965,"record_offset":8276,"parent_record_offset":8248,"ancestor_record_offset":6108,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1965,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":6108,"inlinee":9857,"source_line":141,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1965,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":6108,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1965,"record_offset":8344,"parent_record_offset":8316,"ancestor_record_offset":6108,"inlinee":8777,"source_line":1024,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1965,"record_offset":652,"rva":25958928,"va":5394668048}
// 编译实例 RVAs: 0x18c1a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19586686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e88221d5fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394668169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19587969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e87f1cd5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394669468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfc5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13391969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e89fa733ff","size":5,"soft_fail":false,"target_va":5381277104,"va":5394669068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17178907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e81b210601","size":5,"soft_fail":false,"target_va":5411847200,"va":5394668288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16871338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e8aa6f0101","size":5,"soft_fail":false,"target_va":5411540992,"va":5394669649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-20311947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e87510cafe","size":5,"soft_fail":false,"target_va":5374356912,"va":5394668854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16871324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e89c6f0101","size":5,"soft_fail":false,"target_va":5411540963,"va":5394669634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16872784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e850750101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394669507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16872309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e875730101","size":5,"soft_fail":false,"target_va":5411541904,"va":5394669590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16873167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394668048,"raw_hex":"e8cf760101","size":5,"soft_fail":false,"target_va":5411542880,"va":5394669708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*560(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394668048,"raw_hex":"ff9030020000","size":6,"soft_fail":false,"va":5394669404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":2140,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":1092,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":1488,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":3184,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_set_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1965,"record_offset":5176,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39396,"source_line":1561,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$1034::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1965,"record_offset":3748,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":86886,"source_line":45,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetLayoutBindingFlagsCreateInfo::binding_flags
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1965,"record_offset":4944,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":86890,"source_line":21154,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetLayoutCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1965,"record_offset":4972,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":86854,"source_line":3592,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1965,"record_offset":5524,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1965,"record_offset":2392,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1965,"record_offset":1064,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":30977,"source_line":174,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":5292,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1965,"record_offset":5552,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1965,"record_offset":3280,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":8791,"source_line":1391,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":1965,"record_offset":3096,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39772,"source_line":690,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":1965,"record_offset":4172,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":28563,"source_line":30,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":652,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":652,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":652,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":4560,"parent_record_offset":4532,"ancestor_record_offset":652,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":1384,"parent_record_offset":1208,"ancestor_record_offset":652,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":5452,"parent_record_offset":5364,"ancestor_record_offset":652,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1965,"record_offset":4532,"parent_record_offset":4172,"ancestor_record_offset":652,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":2212,"parent_record_offset":2180,"ancestor_record_offset":652,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":652,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":5392,"parent_record_offset":5364,"ancestor_record_offset":652,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":652,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":652,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":652,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":1208,"parent_record_offset":1176,"ancestor_record_offset":652,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":652,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":652,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1965,"record_offset":1244,"parent_record_offset":1208,"ancestor_record_offset":652,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":2180,"parent_record_offset":2140,"ancestor_record_offset":652,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":1920,"parent_record_offset":1520,"ancestor_record_offset":652,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1965,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":652,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1965,"record_offset":1520,"parent_record_offset":1488,"ancestor_record_offset":652,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4400,"parent_record_offset":4372,"ancestor_record_offset":652,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4484,"parent_record_offset":4456,"ancestor_record_offset":652,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4224,"parent_record_offset":4172,"ancestor_record_offset":652,"inlinee":28486,"source_line":204,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":652,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4332,"parent_record_offset":4276,"ancestor_record_offset":652,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4372,"parent_record_offset":4172,"ancestor_record_offset":652,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":652,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":4276,"parent_record_offset":4248,"ancestor_record_offset":652,"inlinee":28488,"source_line":54,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":652,"inlinee":28487,"source_line":43,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$1033::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1965,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":652,"inlinee":86888,"source_line":39,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1965,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":652,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1965,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":652,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1965,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":652,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1965,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":652,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1965,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":652,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1965,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":652,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1965,"record_offset":3152,"parent_record_offset":3096,"ancestor_record_offset":652,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1965,"record_offset":2028,"parent_record_offset":1996,"ancestor_record_offset":652,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1965,"record_offset":3364,"parent_record_offset":3336,"ancestor_record_offset":652,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1965,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":652,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1965,"record_offset":2872,"parent_record_offset":2836,"ancestor_record_offset":652,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":1965,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":652,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1965,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":652,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1965,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":652,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1965,"record_offset":1996,"parent_record_offset":1964,"ancestor_record_offset":652,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1965,"record_offset":1964,"parent_record_offset":1920,"ancestor_record_offset":652,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1965,"record_offset":5000,"parent_record_offset":4972,"ancestor_record_offset":652,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1965,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":652,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1965,"record_offset":4716,"parent_record_offset":4172,"ancestor_record_offset":652,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1965,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":652,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1965,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":652,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1965,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":652,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1965,"record_offset":5420,"parent_record_offset":5392,"ancestor_record_offset":652,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1965,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":652,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1965,"record_offset":2652,"parent_record_offset":2624,"ancestor_record_offset":652,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1965,"record_offset":3064,"parent_record_offset":2392,"ancestor_record_offset":652,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1965,"record_offset":2964,"parent_record_offset":2500,"ancestor_record_offset":652,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":652,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1965,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":652,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":4668,"parent_record_offset":4640,"ancestor_record_offset":652,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":2068,"parent_record_offset":1920,"ancestor_record_offset":652,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1965,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":652,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1965,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":652,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1965,"record_offset":3336,"parent_record_offset":3308,"ancestor_record_offset":652,"inlinee":5362,"source_line":2612,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1965,"record_offset":3212,"parent_record_offset":3184,"ancestor_record_offset":652,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1965,"record_offset":3240,"parent_record_offset":3212,"ancestor_record_offset":652,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1965,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":652,"inlinee":8777,"source_line":1024,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1965,"record_offset":2588,"parent_record_offset":2552,"ancestor_record_offset":652,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1965,"record_offset":2624,"parent_record_offset":2588,"ancestor_record_offset":652,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1965,"record_offset":2720,"parent_record_offset":2552,"ancestor_record_offset":652,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1965,"record_offset":2552,"parent_record_offset":2500,"ancestor_record_offset":652,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1965,"record_offset":2752,"parent_record_offset":2500,"ancestor_record_offset":652,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1965,"record_offset":2836,"parent_record_offset":2808,"ancestor_record_offset":652,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1965,"record_offset":2808,"parent_record_offset":2500,"ancestor_record_offset":652,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1965,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":652,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1965,"record_offset":2908,"parent_record_offset":2500,"ancestor_record_offset":652,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1965,"record_offset":2500,"parent_record_offset":2468,"ancestor_record_offset":652,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1965,"record_offset":2468,"parent_record_offset":2440,"ancestor_record_offset":652,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1965,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":652,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::DescriptorPoolDiscardPile::reclaim_descriptor_pools
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AC9
// 首条位置: {"module":703,"inlinee":39625}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38500,"parent_record_offset":38312,"ancestor_record_offset":38312,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":39188,"parent_record_offset":38312,"ancestor_record_offset":38312,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":38344,"parent_record_offset":38312,"ancestor_record_offset":38312,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":38772,"parent_record_offset":38748,"ancestor_record_offset":38312,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":38640,"parent_record_offset":38608,"ancestor_record_offset":38312,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":38708,"parent_record_offset":38672,"ancestor_record_offset":38312,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":38820,"parent_record_offset":38796,"ancestor_record_offset":38312,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":38796,"parent_record_offset":38772,"ancestor_record_offset":38312,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":38672,"parent_record_offset":38640,"ancestor_record_offset":38312,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38748,"parent_record_offset":38564,"ancestor_record_offset":38312,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38564,"parent_record_offset":38532,"ancestor_record_offset":38312,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38608,"parent_record_offset":38564,"ancestor_record_offset":38312,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38936,"parent_record_offset":38912,"ancestor_record_offset":38312,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":703,"record_offset":38992,"parent_record_offset":38936,"ancestor_record_offset":38312,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":703,"record_offset":39148,"parent_record_offset":38564,"ancestor_record_offset":38312,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":703,"record_offset":39100,"parent_record_offset":39076,"ancestor_record_offset":38312,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":703,"record_offset":38532,"parent_record_offset":38500,"ancestor_record_offset":38312,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":38884,"parent_record_offset":38860,"ancestor_record_offset":38312,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":38860,"parent_record_offset":38564,"ancestor_record_offset":38312,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":38912,"parent_record_offset":38884,"ancestor_record_offset":38312,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":39028,"parent_record_offset":38884,"ancestor_record_offset":38312,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":38964,"parent_record_offset":38936,"ancestor_record_offset":38312,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":39308,"parent_record_offset":39244,"ancestor_record_offset":38312,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":38408,"parent_record_offset":38372,"ancestor_record_offset":38312,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":39332,"parent_record_offset":39308,"ancestor_record_offset":38312,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":38432,"parent_record_offset":38408,"ancestor_record_offset":38312,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":39216,"parent_record_offset":39188,"ancestor_record_offset":38312,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":38372,"parent_record_offset":38344,"ancestor_record_offset":38312,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":39244,"parent_record_offset":39216,"ancestor_record_offset":38312,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":38464,"parent_record_offset":38372,"ancestor_record_offset":38312,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":39280,"parent_record_offset":39244,"ancestor_record_offset":38312,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":652,"rva":12474944,"va":5381184064}
// 编译实例 RVAs: 0xbe5a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1459986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e812471600","size":5,"soft_fail":false,"target_va":5382644528,"va":5381184537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11027541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8abbb57ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381185664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5668377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8e781a9ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381184404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-563599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e87166f7ff","size":5,"soft_fail":false,"target_va":5380622000,"va":5381185594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30662363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8dbded301","size":5,"soft_fail":false,"target_va":5411847168,"va":5381184800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6829773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e833c997ff","size":5,"soft_fail":false,"target_va":5374354992,"va":5381184760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30297323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8eb4cce01","size":5,"soft_fail":false,"target_va":5411481920,"va":5381184592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30356439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8d733cf01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381185332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30356829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e85d35cf01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381185230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30356484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e80434cf01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381185415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30357597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e85d38cf01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381185278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13481465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8f9b5cd00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381184898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30307265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8c173ce01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381185098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30309682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8327dce01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381185113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t639461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8e5c10900","size":5,"soft_fail":false,"target_va":5381823600,"va":5381184134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t639187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381184064,"raw_hex":"e8d3c00900","size":5,"soft_fail":false,"target_va":5381824016,"va":5381184824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*-88(%rbp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381184064,"raw_hex":"ff55a8","size":3,"soft_fail":false,"va":5381184655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":693,"record_offset":2716,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":693,"record_offset":1532,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::append
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":2036,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":8470,"source_line":2554,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":1716,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":2560,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":1120,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":3020,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":693,"record_offset":3668,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39393,"source_line":1584,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::reset_descriptor_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":693,"record_offset":3464,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39387,"source_line":1601,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorPoolCreateInfo::max_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":693,"record_offset":3612,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39391,"source_line":3667,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorPoolCreateInfo::pool_sizes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":693,"record_offset":3640,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39392,"source_line":3672,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetAllocateInfo::set_layouts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":693,"record_offset":2532,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39386,"source_line":3734,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$174::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":693,"record_offset":3584,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39389,"source_line":3644,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":693,"record_offset":1828,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":693,"record_offset":4248,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":693,"record_offset":4220,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":2804,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":693,"record_offset":3548,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":693,"record_offset":1500,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":693,"record_offset":3280,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":693,"record_offset":908,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":693,"record_offset":1064,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":693,"record_offset":1860,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":693,"record_offset":1744,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::allocate_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":693,"record_offset":4000,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39344,"source_line":32,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::create_descriptor_pool
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":693,"record_offset":3784,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39342,"source_line":32,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::reset_descriptor_pool
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":693,"record_offset":3432,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":39343,"source_line":32,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":1200,"parent_record_offset":1168,"ancestor_record_offset":652,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":652,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":652,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":2348,"parent_record_offset":2316,"ancestor_record_offset":652,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":652,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":652,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":652,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":693,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":652,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":2228,"parent_record_offset":2036,"ancestor_record_offset":652,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":1168,"parent_record_offset":1120,"ancestor_record_offset":652,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":2096,"parent_record_offset":2068,"ancestor_record_offset":652,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":2068,"parent_record_offset":2036,"ancestor_record_offset":652,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":2432,"parent_record_offset":2228,"ancestor_record_offset":652,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":2264,"parent_record_offset":2228,"ancestor_record_offset":652,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":2500,"parent_record_offset":2036,"ancestor_record_offset":652,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":652,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":693,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":652,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":693,"record_offset":3488,"parent_record_offset":3464,"ancestor_record_offset":652,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":693,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":652,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":693,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":652,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":693,"record_offset":1916,"parent_record_offset":1884,"ancestor_record_offset":652,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":693,"record_offset":1944,"parent_record_offset":1884,"ancestor_record_offset":652,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":1316,"parent_record_offset":1120,"ancestor_record_offset":652,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":693,"record_offset":2460,"parent_record_offset":2228,"ancestor_record_offset":652,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":693,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":652,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":693,"record_offset":3316,"parent_record_offset":3280,"ancestor_record_offset":652,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":1288,"parent_record_offset":1120,"ancestor_record_offset":652,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":693,"record_offset":3216,"parent_record_offset":3044,"ancestor_record_offset":652,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":693,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":652,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":693,"record_offset":3344,"parent_record_offset":3280,"ancestor_record_offset":652,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":2924,"parent_record_offset":2860,"ancestor_record_offset":652,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":972,"parent_record_offset":936,"ancestor_record_offset":652,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":652,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":1400,"parent_record_offset":1364,"ancestor_record_offset":652,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":652,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":652,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":652,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":652,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":652,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":652,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":693,"record_offset":1456,"parent_record_offset":1364,"ancestor_record_offset":652,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":693,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":652,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":693,"record_offset":1972,"parent_record_offset":1884,"ancestor_record_offset":652,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":693,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":652,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":693,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":652,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":693,"record_offset":936,"parent_record_offset":908,"ancestor_record_offset":652,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":693,"record_offset":2860,"parent_record_offset":2832,"ancestor_record_offset":652,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":693,"record_offset":1028,"parent_record_offset":936,"ancestor_record_offset":652,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":693,"record_offset":2896,"parent_record_offset":2860,"ancestor_record_offset":652,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$2::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":4496,"rva":6806912,"va":5375516032}
// 编译实例 RVAs: 0x67dd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$3::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":4608,"rva":6806912,"va":5375516032}
// 编译实例 RVAs: 0x67dd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_layouts
// 记录形式: compiled_procedure; instances=1
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":4720,"rva":12476576,"va":5381185696}
// 编译实例 RVAs: 0xbe60a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a360
//     owner=unresolved_namespace; count=2
//     target_aliases=27 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11125450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e8363d56ff","size":5,"soft_fail":false,"target_va":5370061664,"va":5381187109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6104282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e826dba2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381185765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6104880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e8d0d8a2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381186379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140881ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3558058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e856b5c9ff","size":5,"soft_fail":false,"target_va":5377628896,"va":5381186949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bde2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e82e7effff","size":5,"soft_fail":false,"target_va":5381153440,"va":5381186669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418c1a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13481863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e887b7cd00","size":5,"soft_fail":false,"target_va":5394668048,"va":5381186180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418c2800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13485345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e821c5cd00","size":5,"soft_fail":false,"target_va":5394671616,"va":5381186266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30354205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e81d2bcf01","size":5,"soft_fail":false,"target_va":5411540992,"va":5381186782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6830667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e8b5c597ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5381186230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30355034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e85a2ecf01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381185924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30355339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e88b2fcf01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381186720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30356110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e88e32cf01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381186765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1471352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e878731600","size":5,"soft_fail":false,"target_va":5382657856,"va":5381186499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1567258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381185696,"raw_hex":"e81aea1700","size":5,"soft_fail":false,"target_va":5382753792,"va":5381186529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*560(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381185696,"raw_hex":"ff9330020000","size":6,"soft_fail":false,"va":5381186440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":5964,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":5236,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_set_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":693,"record_offset":7620,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":39396,"source_line":1561,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$168::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":693,"record_offset":7592,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":39395,"source_line":3560,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":693,"record_offset":6164,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":693,"record_offset":7136,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":693,"record_offset":6808,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":7360,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":693,"record_offset":8700,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":693,"record_offset":5880,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":693,"record_offset":4908,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":28564,"source_line":26,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":693,"record_offset":6348,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":693,"record_offset":6404,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":693,"record_offset":7916,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":693,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":4720,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":693,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":4720,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":693,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":4720,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":693,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4720,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":693,"record_offset":7520,"parent_record_offset":7432,"ancestor_record_offset":4720,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":4720,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":4992,"parent_record_offset":4964,"ancestor_record_offset":4720,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":5024,"parent_record_offset":4992,"ancestor_record_offset":4720,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":5172,"parent_record_offset":4936,"ancestor_record_offset":4720,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":4936,"parent_record_offset":4908,"ancestor_record_offset":4720,"inlinee":21073,"source_line":1695,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":6044,"parent_record_offset":6012,"ancestor_record_offset":4720,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":5300,"parent_record_offset":5268,"ancestor_record_offset":4720,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":7460,"parent_record_offset":7432,"ancestor_record_offset":4720,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":4720,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":4720,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":4720,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":5360,"parent_record_offset":5328,"ancestor_record_offset":4720,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":5328,"parent_record_offset":5300,"ancestor_record_offset":4720,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":4720,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":693,"record_offset":5636,"parent_record_offset":5604,"ancestor_record_offset":4720,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":6012,"parent_record_offset":5964,"ancestor_record_offset":4720,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":5268,"parent_record_offset":5236,"ancestor_record_offset":4720,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":693,"record_offset":7648,"parent_record_offset":7620,"ancestor_record_offset":4720,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":693,"record_offset":7672,"parent_record_offset":7648,"ancestor_record_offset":4720,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":693,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":4720,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":693,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":4720,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":693,"record_offset":5684,"parent_record_offset":5660,"ancestor_record_offset":4720,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":693,"record_offset":8444,"parent_record_offset":8412,"ancestor_record_offset":4720,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":693,"record_offset":8504,"parent_record_offset":8472,"ancestor_record_offset":4720,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":693,"record_offset":6664,"parent_record_offset":6636,"ancestor_record_offset":4720,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":693,"record_offset":6724,"parent_record_offset":6692,"ancestor_record_offset":4720,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":693,"record_offset":6836,"parent_record_offset":6808,"ancestor_record_offset":4720,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":693,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":4720,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":693,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":4720,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":693,"record_offset":5708,"parent_record_offset":5684,"ancestor_record_offset":4720,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":693,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":4720,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":693,"record_offset":7488,"parent_record_offset":7460,"ancestor_record_offset":4720,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":693,"record_offset":8472,"parent_record_offset":8412,"ancestor_record_offset":4720,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":693,"record_offset":8048,"parent_record_offset":8012,"ancestor_record_offset":4720,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":693,"record_offset":8164,"parent_record_offset":8136,"ancestor_record_offset":4720,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":693,"record_offset":6608,"parent_record_offset":6576,"ancestor_record_offset":4720,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":4720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":693,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":4720,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":693,"record_offset":8252,"parent_record_offset":8136,"ancestor_record_offset":4720,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":693,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":4720,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":693,"record_offset":6228,"parent_record_offset":6192,"ancestor_record_offset":4720,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":8328,"parent_record_offset":8300,"ancestor_record_offset":4720,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":4720,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":8276,"parent_record_offset":8252,"ancestor_record_offset":4720,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":6132,"parent_record_offset":5964,"ancestor_record_offset":4720,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":693,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":4720,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":693,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":4720,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":693,"record_offset":6192,"parent_record_offset":6164,"ancestor_record_offset":4720,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":693,"record_offset":6256,"parent_record_offset":6192,"ancestor_record_offset":4720,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":693,"record_offset":6636,"parent_record_offset":6576,"ancestor_record_offset":4720,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":693,"record_offset":6692,"parent_record_offset":6636,"ancestor_record_offset":4720,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":693,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":4720,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":693,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":4720,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":693,"record_offset":6576,"parent_record_offset":6488,"ancestor_record_offset":4720,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":693,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":4720,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":693,"record_offset":6548,"parent_record_offset":6488,"ancestor_record_offset":4720,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":693,"record_offset":6520,"parent_record_offset":6488,"ancestor_record_offset":4720,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":693,"record_offset":6488,"parent_record_offset":6460,"ancestor_record_offset":4720,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":693,"record_offset":8612,"parent_record_offset":7916,"ancestor_record_offset":4720,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":693,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":4720,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":693,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":4720,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":693,"record_offset":8076,"parent_record_offset":8048,"ancestor_record_offset":4720,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":693,"record_offset":8412,"parent_record_offset":8012,"ancestor_record_offset":4720,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":693,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":4720,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":693,"record_offset":8104,"parent_record_offset":8076,"ancestor_record_offset":4720,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":693,"record_offset":8136,"parent_record_offset":8104,"ancestor_record_offset":4720,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":693,"record_offset":8012,"parent_record_offset":7988,"ancestor_record_offset":4720,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":693,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":4720,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_layouts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1510A
// 首条位置: {"module":1917,"inlinee":86282}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::ComputePipelineDesc::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":9256,"rva":244608,"va":5368953728}
// 编译实例 RVAs: 0x3bb80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 386
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":19156,"rva":23743264,"va":5392452384}
// 编译实例 RVAs: 0x16a4b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17371631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e811eef6fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5392453114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17371693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e8d3edf6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392453192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140881ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14825390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e852c81dff","size":5,"soft_fail":false,"target_va":5377628896,"va":5392454281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408945f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14748757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e8abf31eff","size":5,"soft_fail":false,"target_va":5377705456,"va":5392454208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c530
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11832802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e81e724bff","size":5,"soft_fail":false,"target_va":5380621616,"va":5392454413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d797d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9614537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e8374b6dff","size":5,"soft_fail":false,"target_va":5382838224,"va":5392452756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19087249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e8913f2301","size":5,"soft_fail":false,"target_va":5411540963,"va":5392453709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19087922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e832422301","size":5,"soft_fail":false,"target_va":5411541808,"va":5392453881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19088638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e8fe442301","size":5,"soft_fail":false,"target_va":5411542064,"va":5392453421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19088781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e88d452301","size":5,"soft_fail":false,"target_va":5411542296,"va":5392453510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19088044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e8ac422301","size":5,"soft_fail":false,"target_va":5411541904,"va":5392453855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19089393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e8f1472301","size":5,"soft_fail":false,"target_va":5411542880,"va":5392453482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::leek::create_compute_pipeline
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11280631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e809df53ff","size":5,"soft_fail":false,"target_va":5381171824,"va":5392452450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::create_descriptor_set_layouts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11266809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e8071554ff","size":5,"soft_fail":false,"target_va":5381185696,"va":5392452500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::create_descriptor_set_states
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11266017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392452384,"raw_hex":"e81f1854ff","size":5,"soft_fail":false,"target_va":5381187264,"va":5392453276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*528(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392452384,"raw_hex":"ff9710020000","size":6,"soft_fail":false,"va":5392452698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1657,"record_offset":19780,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":19684,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_compute_pipelines
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":20408,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":75198,"source_line":2178,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_pipeline_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":19568,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":75163,"source_line":2219,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_shader_module
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":20208,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":73861,"source_line":2668,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ComputePipelineCreateInfo::stage
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":20380,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":75197,"source_line":3955,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineLayoutCreateInfo::set_layouts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":19540,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":75162,"source_line":5320,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ShaderModuleCreateInfo::code
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":20176,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":73860,"source_line":3467,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$189::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":20324,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":73863,"source_line":3843,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$194::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":20352,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":75195,"source_line":3931,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$276::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1657,"record_offset":19512,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":75160,"source_line":5291,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":20872,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":20704,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1657,"record_offset":19808,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1657,"record_offset":21140,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1657,"record_offset":19456,"parent_record_offset":19156,"ancestor_record_offset":19156,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":20620,"parent_record_offset":20596,"ancestor_record_offset":19156,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":20644,"parent_record_offset":20620,"ancestor_record_offset":19156,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":20024,"parent_record_offset":20000,"ancestor_record_offset":19156,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":20596,"parent_record_offset":20568,"ancestor_record_offset":19156,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":20000,"parent_record_offset":19944,"ancestor_record_offset":19156,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1657,"record_offset":20988,"parent_record_offset":20964,"ancestor_record_offset":19156,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1657,"record_offset":20936,"parent_record_offset":20912,"ancestor_record_offset":19156,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":20508,"parent_record_offset":20480,"ancestor_record_offset":19156,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":19972,"parent_record_offset":19944,"ancestor_record_offset":19156,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":19944,"parent_record_offset":19920,"ancestor_record_offset":19156,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":20568,"parent_record_offset":20536,"ancestor_record_offset":19156,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":20536,"parent_record_offset":20508,"ancestor_record_offset":19156,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":19920,"parent_record_offset":19896,"ancestor_record_offset":19156,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1657,"record_offset":22116,"parent_record_offset":22080,"ancestor_record_offset":19156,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1657,"record_offset":21928,"parent_record_offset":21904,"ancestor_record_offset":19156,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":20452,"parent_record_offset":20408,"ancestor_record_offset":19156,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":20480,"parent_record_offset":20452,"ancestor_record_offset":19156,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1657,"record_offset":19596,"parent_record_offset":19568,"ancestor_record_offset":19156,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1657,"record_offset":19620,"parent_record_offset":19596,"ancestor_record_offset":19156,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1657,"record_offset":19644,"parent_record_offset":19620,"ancestor_record_offset":19156,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1657,"record_offset":21200,"parent_record_offset":21172,"ancestor_record_offset":19156,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":21384,"parent_record_offset":21352,"ancestor_record_offset":19156,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":19848,"parent_record_offset":19808,"ancestor_record_offset":19156,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":22148,"parent_record_offset":22116,"ancestor_record_offset":19156,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":19712,"parent_record_offset":19684,"ancestor_record_offset":19156,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":19740,"parent_record_offset":19712,"ancestor_record_offset":19156,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":21964,"parent_record_offset":21928,"ancestor_record_offset":19156,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":21988,"parent_record_offset":21964,"ancestor_record_offset":19156,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":22020,"parent_record_offset":21928,"ancestor_record_offset":19156,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":19480,"parent_record_offset":19456,"ancestor_record_offset":19156,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$5::create_compute_pipeline::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":21172,"parent_record_offset":21140,"ancestor_record_offset":19156,"inlinee":75200,"source_line":453,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$5::create_compute_pipeline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 453
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x125C0
// 首条位置: {"module":1657,"inlinee":75200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1657,"record_offset":21200,"parent_record_offset":21172,"ancestor_record_offset":21172,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":21228,"parent_record_offset":21172,"ancestor_record_offset":21172,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":21440,"parent_record_offset":21416,"ancestor_record_offset":21172,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":21416,"parent_record_offset":21324,"ancestor_record_offset":21172,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":21352,"parent_record_offset":21324,"ancestor_record_offset":21172,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":21324,"parent_record_offset":21300,"ancestor_record_offset":21172,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":21300,"parent_record_offset":21276,"ancestor_record_offset":21172,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":21384,"parent_record_offset":21352,"ancestor_record_offset":21172,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":21252,"parent_record_offset":21228,"ancestor_record_offset":21172,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_states
// 记录形式: compiled_procedure; instances=1
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":9380,"rva":12478144,"va":5381187264}
// 编译实例 RVAs: 0xbe66c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b6140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13433347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381187264,"raw_hex":"e803facc00","size":5,"soft_fail":false,"target_va":5394620736,"va":5381187384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":9524,"parent_record_offset":9380,"ancestor_record_offset":9380,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":693,"record_offset":9744,"parent_record_offset":9380,"ancestor_record_offset":9380,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":693,"record_offset":9604,"parent_record_offset":9380,"ancestor_record_offset":9380,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":9768,"parent_record_offset":9744,"ancestor_record_offset":9380,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":693,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":9380,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":693,"record_offset":9684,"parent_record_offset":9604,"ancestor_record_offset":9380,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":693,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":9380,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":693,"record_offset":9792,"parent_record_offset":9768,"ancestor_record_offset":9380,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":693,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":9380,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":693,"record_offset":9572,"parent_record_offset":9524,"ancestor_record_offset":9380,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_states::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 480
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152D3
// 首条位置: {"module":1935,"inlinee":86739}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1935,"record_offset":9780,"parent_record_offset":8540,"ancestor_record_offset":8540,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1935,"record_offset":8644,"parent_record_offset":8540,"ancestor_record_offset":8540,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1935,"record_offset":10088,"parent_record_offset":8540,"ancestor_record_offset":8540,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":10420,"parent_record_offset":8540,"ancestor_record_offset":8540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1935,"record_offset":10272,"parent_record_offset":8540,"ancestor_record_offset":8540,"inlinee":9926,"source_line":296,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1935,"record_offset":8672,"parent_record_offset":8540,"ancestor_record_offset":8540,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1935,"record_offset":9000,"parent_record_offset":8540,"ancestor_record_offset":8540,"inlinee":11891,"source_line":2551,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":9924,"parent_record_offset":9900,"ancestor_record_offset":8540,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":9948,"parent_record_offset":9924,"ancestor_record_offset":8540,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":10608,"parent_record_offset":10584,"ancestor_record_offset":8540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":8540,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":10584,"parent_record_offset":10492,"ancestor_record_offset":8540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1935,"record_offset":9816,"parent_record_offset":9780,"ancestor_record_offset":8540,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1935,"record_offset":9844,"parent_record_offset":9816,"ancestor_record_offset":8540,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1935,"record_offset":9876,"parent_record_offset":9844,"ancestor_record_offset":8540,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1935,"record_offset":10028,"parent_record_offset":9780,"ancestor_record_offset":8540,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":8540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":10520,"parent_record_offset":10492,"ancestor_record_offset":8540,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":8540,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":10228,"parent_record_offset":10204,"ancestor_record_offset":8540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":8540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":8540,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1935,"record_offset":10156,"parent_record_offset":10088,"ancestor_record_offset":8540,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$329::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1935,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":8540,"inlinee":39783,"source_line":174,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1935,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":8540,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1935,"record_offset":8912,"parent_record_offset":8880,"ancestor_record_offset":8540,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1935,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":8540,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1935,"record_offset":10552,"parent_record_offset":10520,"ancestor_record_offset":8540,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1935,"record_offset":9384,"parent_record_offset":9352,"ancestor_record_offset":8540,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1935,"record_offset":9352,"parent_record_offset":9320,"ancestor_record_offset":8540,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1935,"record_offset":8948,"parent_record_offset":8796,"ancestor_record_offset":8540,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":8540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1935,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":8540,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1935,"record_offset":10328,"parent_record_offset":10300,"ancestor_record_offset":8540,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":9976,"parent_record_offset":9948,"ancestor_record_offset":8540,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":10128,"parent_record_offset":10088,"ancestor_record_offset":8540,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1935,"record_offset":10300,"parent_record_offset":10272,"ancestor_record_offset":8540,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1935,"record_offset":9280,"parent_record_offset":9240,"ancestor_record_offset":8540,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1935,"record_offset":9320,"parent_record_offset":9280,"ancestor_record_offset":8540,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1935,"record_offset":9428,"parent_record_offset":9240,"ancestor_record_offset":8540,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1935,"record_offset":9240,"parent_record_offset":9188,"ancestor_record_offset":8540,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1935,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":8540,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1935,"record_offset":8880,"parent_record_offset":8824,"ancestor_record_offset":8540,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1935,"record_offset":9520,"parent_record_offset":9188,"ancestor_record_offset":8540,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1935,"record_offset":8720,"parent_record_offset":8696,"ancestor_record_offset":8540,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1935,"record_offset":9088,"parent_record_offset":9044,"ancestor_record_offset":8540,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1935,"record_offset":9620,"parent_record_offset":9188,"ancestor_record_offset":8540,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1935,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":8540,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1935,"record_offset":9188,"parent_record_offset":9132,"ancestor_record_offset":8540,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1935,"record_offset":8744,"parent_record_offset":8720,"ancestor_record_offset":8540,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1935,"record_offset":8768,"parent_record_offset":8744,"ancestor_record_offset":8540,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1935,"record_offset":9132,"parent_record_offset":9088,"ancestor_record_offset":8540,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::create_descriptor_set_states::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1935,"record_offset":10360,"parent_record_offset":10272,"ancestor_record_offset":8540,"inlinee":86741,"source_line":486,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1935,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":8540,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1935,"record_offset":9044,"parent_record_offset":9000,"ancestor_record_offset":8540,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_states::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 486
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152D5
// 首条位置: {"module":1935,"inlinee":86741}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::enums::impl$329::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1935,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":10360,"inlinee":39783,"source_line":174,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$118::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7150
// 首条位置: {"module":509,"inlinee":29008}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":509,"record_offset":68284,"parent_record_offset":68260,"ancestor_record_offset":68260,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":509,"record_offset":68308,"parent_record_offset":68284,"ancestor_record_offset":68260,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$120::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A21
// 首条位置: {"module":1582,"inlinee":72225}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":1582,"record_offset":26824,"parent_record_offset":26800,"ancestor_record_offset":26800,"inlinee":15752,"source_line":323,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26876,"parent_record_offset":26848,"ancestor_record_offset":26800,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26900,"parent_record_offset":26876,"ancestor_record_offset":26800,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26848,"parent_record_offset":26824,"ancestor_record_offset":26800,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26960,"parent_record_offset":26848,"ancestor_record_offset":26800,"inlinee":15760,"source_line":816,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":26924,"parent_record_offset":26900,"ancestor_record_offset":26800,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_topology
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 623
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75F5
// 首条位置: {"module":528,"inlinee":30197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":528,"record_offset":13384,"parent_record_offset":13364,"ancestor_record_offset":13364,"inlinee":30198,"source_line":623,"archive":"_pdb/B/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":528,"record_offset":13404,"parent_record_offset":13384,"ancestor_record_offset":13364,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 623
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75F6
// 首条位置: {"module":528,"inlinee":30198}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":528,"record_offset":13404,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::topology
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 623
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1256F
// 首条位置: {"module":1655,"inlinee":75119}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1655,"record_offset":31844,"parent_record_offset":31816,"ancestor_record_offset":31816,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::topology_or_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14144,"parent_record_offset":14116,"ancestor_record_offset":14116,"inlinee":75169,"source_line":623,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1657,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":14116,"inlinee":29877,"source_line":23,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1657,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":14116,"inlinee":29886,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::topology_or_err
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 623
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x125A1
// 首条位置: {"module":1657,"inlinee":75169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1657,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":14144,"inlinee":29877,"source_line":23,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1657,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":14144,"inlinee":29886,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::cull_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 624
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x125A7
// 首条位置: {"module":1657,"inlinee":75175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::cull_mode_or_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":14352,"inlinee":75176,"source_line":624,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1657,"record_offset":14400,"parent_record_offset":14376,"ancestor_record_offset":14352,"inlinee":29877,"source_line":23,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1657,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":14352,"inlinee":29886,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::cull_mode_or_err
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 624
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x125A8
// 首条位置: {"module":1657,"inlinee":75176}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1657,"record_offset":14400,"parent_record_offset":14376,"ancestor_record_offset":14376,"inlinee":29877,"source_line":23,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1657,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":14376,"inlinee":29886,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 624
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69AF
// 首条位置: {"module":487,"inlinee":27055}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":38184,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":27056,"source_line":624,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":38208,"parent_record_offset":38184,"ancestor_record_offset":38160,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 624
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69B0
// 首条位置: {"module":487,"inlinee":27056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":38208,"parent_record_offset":38184,"ancestor_record_offset":38184,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 626
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x125A9
// 首条位置: {"module":1657,"inlinee":75177}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1657,"record_offset":14576,"parent_record_offset":14464,"ancestor_record_offset":14464,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode_or_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1657,"record_offset":14492,"parent_record_offset":14464,"ancestor_record_offset":14464,"inlinee":75178,"source_line":626,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1657,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":14464,"inlinee":29877,"source_line":23,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1657,"record_offset":14540,"parent_record_offset":14516,"ancestor_record_offset":14464,"inlinee":29886,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode_or_err
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 626
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x125AA
// 首条位置: {"module":1657,"inlinee":75178}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1657,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":14492,"inlinee":29877,"source_line":23,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1657,"record_offset":14540,"parent_record_offset":14516,"ancestor_record_offset":14492,"inlinee":29886,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 626
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7677
// 首条位置: {"module":530,"inlinee":30327}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":94796,"parent_record_offset":94772,"ancestor_record_offset":94772,"inlinee":30328,"source_line":626,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":530,"record_offset":94820,"parent_record_offset":94796,"ancestor_record_offset":94772,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 626
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7678
// 首条位置: {"module":530,"inlinee":30328}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":530,"record_offset":94820,"parent_record_offset":94796,"ancestor_record_offset":94796,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 627
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7650
// 首条位置: {"module":529,"inlinee":30288}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":33508,"parent_record_offset":33484,"ancestor_record_offset":33484,"inlinee":30289,"source_line":627,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":33532,"parent_record_offset":33508,"ancestor_record_offset":33484,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 627
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7651
// 首条位置: {"module":529,"inlinee":30289}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":33532,"parent_record_offset":33508,"ancestor_record_offset":33508,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 628
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x205D
// 首条位置: {"module":33,"inlinee":8285}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=21
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":33,"record_offset":23040,"parent_record_offset":23016,"ancestor_record_offset":23016,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":33,"record_offset":23064,"parent_record_offset":23040,"ancestor_record_offset":23016,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 628
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x205E
// 首条位置: {"module":33,"inlinee":8286}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":33,"record_offset":23064,"parent_record_offset":23040,"ancestor_record_offset":23040,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 630
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6587
// 首条位置: {"module":475,"inlinee":25991}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":21736,"inlinee":25992,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":475,"record_offset":21784,"parent_record_offset":21760,"ancestor_record_offset":21736,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 630
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6588
// 首条位置: {"module":475,"inlinee":25992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":475,"record_offset":21784,"parent_record_offset":21760,"ancestor_record_offset":21760,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 632
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6589
// 首条位置: {"module":475,"inlinee":25993}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":21820,"inlinee":25994,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":475,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":21820,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 632
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x658A
// 首条位置: {"module":475,"inlinee":25994}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":475,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":21844,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 634
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x769A
// 首条位置: {"module":531,"inlinee":30362}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":78828,"parent_record_offset":78804,"ancestor_record_offset":78804,"inlinee":30363,"source_line":634,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":531,"record_offset":78852,"parent_record_offset":78828,"ancestor_record_offset":78804,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op_checked
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 634
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x769B
// 首条位置: {"module":531,"inlinee":30363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":531,"record_offset":78852,"parent_record_offset":78828,"ancestor_record_offset":78828,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 635
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x769C
// 首条位置: {"module":531,"inlinee":30364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":78912,"parent_record_offset":78888,"ancestor_record_offset":78888,"inlinee":30365,"source_line":635,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":531,"record_offset":78936,"parent_record_offset":78912,"ancestor_record_offset":78888,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask_checked
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 635
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x769D
// 首条位置: {"module":531,"inlinee":30365}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":531,"record_offset":78936,"parent_record_offset":78912,"ancestor_record_offset":78912,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 637
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2061
// 首条位置: {"module":33,"inlinee":8289}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=25
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":33,"record_offset":23124,"parent_record_offset":23100,"ancestor_record_offset":23100,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":33,"record_offset":23148,"parent_record_offset":23124,"ancestor_record_offset":23100,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 637
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2062
// 首条位置: {"module":33,"inlinee":8290}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":33,"record_offset":23148,"parent_record_offset":23124,"ancestor_record_offset":23124,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::polygon_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 638
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12570
// 首条位置: {"module":1655,"inlinee":75120}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1655,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":31884,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 638
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7679
// 首条位置: {"module":530,"inlinee":30329}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":98156,"parent_record_offset":98132,"ancestor_record_offset":98132,"inlinee":30330,"source_line":638,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":530,"record_offset":98180,"parent_record_offset":98156,"ancestor_record_offset":98132,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode_checked
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 638
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x767A
// 首条位置: {"module":530,"inlinee":30330}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":530,"record_offset":98180,"parent_record_offset":98156,"ancestor_record_offset":98156,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$7::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 657
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":9832,"rva":12478288,"va":5381187408}
// 编译实例 RVAs: 0xbe6750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$123::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 660
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x714C
// 首条位置: {"module":509,"inlinee":29004}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$12::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":509,"record_offset":68232,"parent_record_offset":68204,"ancestor_record_offset":68204,"inlinee":29006,"source_line":194,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::impl$118::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":509,"record_offset":68260,"parent_record_offset":68204,"ancestor_record_offset":68204,"inlinee":29008,"source_line":621,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":509,"record_offset":68284,"parent_record_offset":68260,"ancestor_record_offset":68204,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":509,"record_offset":68308,"parent_record_offset":68284,"ancestor_record_offset":68204,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$125::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 660
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A15
// 首条位置: {"module":1582,"inlinee":72213}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$13::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1582,"record_offset":26432,"parent_record_offset":26404,"ancestor_record_offset":26404,"inlinee":72215,"source_line":200,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::impl$120::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1582,"record_offset":26800,"parent_record_offset":26404,"ancestor_record_offset":26404,"inlinee":72225,"source_line":621,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":1582,"record_offset":26824,"parent_record_offset":26800,"ancestor_record_offset":26404,"inlinee":15752,"source_line":323,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26876,"parent_record_offset":26848,"ancestor_record_offset":26404,"inlinee":15755,"source_line":484,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u16
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26728,"parent_record_offset":26704,"ancestor_record_offset":26404,"inlinee":56116,"source_line":369,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26536,"parent_record_offset":26512,"ancestor_record_offset":26404,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26644,"parent_record_offset":26620,"ancestor_record_offset":26404,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26900,"parent_record_offset":26876,"ancestor_record_offset":26404,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$10::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26704,"parent_record_offset":26680,"ancestor_record_offset":26404,"inlinee":17376,"source_line":811,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26512,"parent_record_offset":26488,"ancestor_record_offset":26404,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26848,"parent_record_offset":26824,"ancestor_record_offset":26404,"inlinee":15754,"source_line":937,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26620,"parent_record_offset":26600,"ancestor_record_offset":26404,"inlinee":21871,"source_line":811,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":26960,"parent_record_offset":26848,"ancestor_record_offset":26404,"inlinee":15760,"source_line":816,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":26924,"parent_record_offset":26900,"ancestor_record_offset":26404,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":26752,"parent_record_offset":26728,"ancestor_record_offset":26404,"inlinee":56117,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":26560,"parent_record_offset":26536,"ancestor_record_offset":26404,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$37::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1582,"record_offset":26680,"parent_record_offset":26456,"ancestor_record_offset":26404,"inlinee":72223,"source_line":14,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$45::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1582,"record_offset":26488,"parent_record_offset":26456,"ancestor_record_offset":26404,"inlinee":72219,"source_line":23,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$52::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1582,"record_offset":26600,"parent_record_offset":26456,"ancestor_record_offset":26404,"inlinee":72221,"source_line":32,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$9::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1582,"record_offset":26456,"parent_record_offset":26432,"ancestor_record_offset":26404,"inlinee":72217,"source_line":171,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineDesc::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 667
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":9948,"rva":12478304,"va":5381187424}
// 编译实例 RVAs: 0xbe6760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$127::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 681
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6870
// 首条位置: {"module":482,"inlinee":26736}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$130::eq
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 681
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80E8
// 首条位置: {"module":577,"inlinee":33000}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::enums::impl$266::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1613,"record_offset":42156,"parent_record_offset":42096,"ancestor_record_offset":42096,"inlinee":33002,"source_line":36,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$275::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1613,"record_offset":42184,"parent_record_offset":42096,"ancestor_record_offset":42096,"inlinee":33004,"source_line":55,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1613,"record_offset":42128,"parent_record_offset":42096,"ancestor_record_offset":42096,"inlinee":8804,"source_line":649,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1613,"record_offset":42212,"parent_record_offset":42096,"ancestor_record_offset":42096,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$132::hash
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 681
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80DA
// 首条位置: {"module":577,"inlinee":32986}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::enums::impl$270::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1582,"record_offset":50924,"parent_record_offset":50692,"ancestor_record_offset":50692,"inlinee":32990,"source_line":36,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$279::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1582,"record_offset":51120,"parent_record_offset":50692,"ancestor_record_offset":50692,"inlinee":32992,"source_line":55,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$531::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1582,"record_offset":50728,"parent_record_offset":50692,"ancestor_record_offset":50692,"inlinee":32988,"source_line":649,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$22::hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1582,"record_offset":51316,"parent_record_offset":50692,"ancestor_record_offset":50692,"inlinee":32994,"source_line":49,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50776,"parent_record_offset":50752,"ancestor_record_offset":50692,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":51364,"parent_record_offset":51340,"ancestor_record_offset":50692,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50800,"parent_record_offset":50776,"ancestor_record_offset":50692,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":51388,"parent_record_offset":51364,"ancestor_record_offset":50692,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":50752,"parent_record_offset":50728,"ancestor_record_offset":50692,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":51340,"parent_record_offset":51316,"ancestor_record_offset":50692,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1582,"record_offset":50876,"parent_record_offset":50852,"ancestor_record_offset":50692,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":51412,"parent_record_offset":51388,"ancestor_record_offset":50692,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1582,"record_offset":50824,"parent_record_offset":50800,"ancestor_record_offset":50692,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1582,"record_offset":50852,"parent_record_offset":50800,"ancestor_record_offset":50692,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RenderPassAttachmentDesc::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 690
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":10072,"rva":12478336,"va":5381187456}
// 编译实例 RVAs: 0xbe6780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RenderPassAttachmentDesc::clear_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 704
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":10200,"rva":12478368,"va":5381187488}
// 编译实例 RVAs: 0xbe67a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":10336,"rva":12478400,"va":5381187520}
// 编译实例 RVAs: 0xbe67c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$12::create_raster_pipeline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 802
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12082
// 首条位置: {"module":1610,"inlinee":73858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1610,"record_offset":59628,"parent_record_offset":59384,"ancestor_record_offset":59384,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_shader_module
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1610,"record_offset":59480,"parent_record_offset":59384,"ancestor_record_offset":59384,"inlinee":73861,"source_line":2668,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineShaderStageCreateInfo::name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1610,"record_offset":59924,"parent_record_offset":59384,"ancestor_record_offset":59384,"inlinee":73865,"source_line":3877,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ShaderModuleCreateInfo::code
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1610,"record_offset":59448,"parent_record_offset":59384,"ancestor_record_offset":59384,"inlinee":73860,"source_line":3467,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$189::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1610,"record_offset":59600,"parent_record_offset":59384,"ancestor_record_offset":59384,"inlinee":73863,"source_line":3843,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":59952,"parent_record_offset":59384,"ancestor_record_offset":59384,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":59656,"parent_record_offset":59384,"ancestor_record_offset":59384,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":59868,"parent_record_offset":59844,"ancestor_record_offset":59384,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":59844,"parent_record_offset":59788,"ancestor_record_offset":59384,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":59816,"parent_record_offset":59788,"ancestor_record_offset":59384,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":59788,"parent_record_offset":59764,"ancestor_record_offset":59384,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":59764,"parent_record_offset":59740,"ancestor_record_offset":59384,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1610,"record_offset":59512,"parent_record_offset":59480,"ancestor_record_offset":59384,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1610,"record_offset":59536,"parent_record_offset":59512,"ancestor_record_offset":59384,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1610,"record_offset":59560,"parent_record_offset":59536,"ancestor_record_offset":59384,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":59692,"parent_record_offset":59656,"ancestor_record_offset":59384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::get_vk_blend_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 991
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":10476,"rva":12478448,"va":5381187568}
// 编译实例 RVAs: 0xbe67f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::map_vertex_attribute_format
// 记录形式: compiled_procedure; instances=1
// 源码行: 1049
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":693,"record_offset":10592,"rva":12478544,"va":5381187664}
// 编译实例 RVAs: 0xbe6850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30354061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381187664,"raw_hex":"e88d2acf01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381187838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":693,"record_offset":10728,"parent_record_offset":10592,"ancestor_record_offset":10592,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00693.jsonl.gz"}
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
// 完整结构及原始类型记录: shader.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::ComputePipelineDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::DescriptorPoolDiscardPile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["rhapsody::raw::shader::DescriptorPoolDiscardPile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["ash::vk::definitions::DescriptorPool", "alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::shader::DescriptorPoolDiscardPile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::DescriptorSetInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::shader::DescriptorSetInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::shader::DescriptorSetInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::DescriptorSetState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::shader::DescriptorSetState>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::shader::DescriptorSetState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >", "rhapsody::raw::shader::create_descriptor_set_states::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::RasterPipeline
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["rhapsody::raw::shader::RasterPipeline", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::RasterPipelineDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::shader::RasterPipelineDesc>", "constructor": "ref$", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::shader::RasterPipelineDesc> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<rhapsody::raw::shader::RasterPipelineDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::RasterPipelineOpts
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 31; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::RenderPassAttachmentDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "arrayvec::arrayvec::impl$27::hash<rhapsody::raw::shader::RenderPassAttachmentDesc,8,std::hash::random::DefaultHasher>", "constructor": "arrayvec::arrayvec::impl$27::hash", "arguments": ["rhapsody::raw::shader::RenderPassAttachmentDesc", "8", "std::hash::random::DefaultHasher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::shader::RenderPassAttachmentDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>", "rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>", "core::option::IntoIter<ash::vk::definitions::AttachmentDescription>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::AttachmentDescription,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::AttachmentDescription", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::ShaderPipelineCommon
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::shader::ShaderPipelineCommon>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::shader::ShaderPipelineCommon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shader.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shader.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 169; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >
// 信息 ID: 7fc373da8c168ef6c73625f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["shader_pipeline::compiled::CompiledShaderPipeline", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline -> ["alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::ComputeShaderProgram::new @ 0x938028 -> 0x16a4b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >
// 信息 ID: bf5e1bf0949617cd251f7382
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["shader_pipeline::compiled::CompiledShaderPipeline", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline -> ["alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::RasterShaderProgramVariant::get_or_generate @ 0x938e08 -> 0x16a3720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::ComputePipelineDesc::new
// 信息 ID: ffaba85f4dd8cd9ae2172ebf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0094895827cf75842dfd9081: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 01032eead5fe87cdf7af3db3: void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 0230a00f6501f069c8d8be52: void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// [泛型宿主/闭包/直接引用方参数候选] 06f136cf5a2d63be4470c1a9: void (*)(bevy_ecs::event::reader::EventReader<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 077651db72eb55a761f661a4: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::AutoClutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,enum2$<core::option::Option<ref$<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 08b4757565a23d44078797c4: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >,bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 0aa172b68cab4584f80e4193: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 0aea34a098357f624f5c6f72: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)
// 其余候选接口见完整附档，共 154 条。
// 引用方: 原程序直接转移 646 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> > @ 0x23cc -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> > @ 0x242c -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)> > @ 0x3365d -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)> > @ 0x33672 -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)> > @ 0x336bd -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)> > @ 0x336d2 -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> > @ 0x404eb -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> > @ 0x4055b -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)> > @ 0x47e28 -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)> > @ 0x47e98 -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<epaint::TextureId>,rhapsody::raw::texture::Texture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<epaint::TextureId>,rhapsody::raw::texture::Texture,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > > @ 0x4934a -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global> @ 0x4cf86 -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global> | hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global> | hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global> @ 0x4ef7a -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,alloc::alloc::Global> @ 0x4f75b -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<epaint::TextureId>,rhapsody::raw::texture::Texture>,alloc::alloc::Global> @ 0x51b8a -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> > @ 0xaf24c -> 0x3bb80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::DescriptorPoolDiscardPile::reclaim_descriptor_pools
// 信息 ID: 727bb3244f3eb3e2dfffc14c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::Queue::reclaim_command_buffer_resources; module=703, inline_record=38312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
// 信息 ID: ef8eba143daa1a322967fc9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > | rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> > @ 0x16a53f7 -> 0xbe5a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > @ 0x16a59ac -> 0xbe5a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > @ 0x16a60ac -> 0xbe5a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineDesc::new
// 信息 ID: edbbf3bf696652e5cbc2502d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1697f80 -> 0xbe6760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::cull_mode
// 信息 ID: cf0e53c6dd7bc8c11f0a14fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1657, inline_record=14352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::cull_mode_or_err
// 信息 ID: 779139f05929411d2c4f7c0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1657, inline_record=14376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode
// 信息 ID: 5500f86ca6e0c5e924acd0cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1657, inline_record=14464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode_or_err
// 信息 ID: 7060096f68d6f127d32aed73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1657, inline_record=14492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::polygon_mode
// 信息 ID: 455327b368420bdebdda0e77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=31884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
// 信息 ID: 6f049210b67e5a4bd422e3f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=33, inline_record=23100; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass; module=475, inline_record=54884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass; module=475, inline_record=57092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay; module=487, inline_record=48752; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::painterly::painterly_pass; module=487, inline_record=58568; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_render_from_target; module=487, inline_record=70140; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_render_from_target; module=487, inline_record=72180; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=489, inline_record=26816; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
// 信息 ID: 8a18cb4ea5781da45498e116
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=33, inline_record=23124; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass; module=475, inline_record=54908; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass; module=475, inline_record=57116; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay; module=487, inline_record=48776; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::painterly::painterly_pass; module=487, inline_record=58592; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_render_from_target; module=487, inline_record=70164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_render_from_target; module=487, inline_record=72204; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=489, inline_record=26840; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
// 信息 ID: 30e89eb0e410c947d8d9c00d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>; module=487, inline_record=38160; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>; module=487, inline_record=41144; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$0,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=19612; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=31532; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$17,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=37956; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$15,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=56596; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$34,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=57432; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$60,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=65812; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
// 信息 ID: d89d494de60028e82e14e707
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>; module=487, inline_record=38184; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>; module=487, inline_record=41168; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$0,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=19636; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=31552; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$17,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=37980; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$15,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=56620; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$34,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=57452; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$60,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=65836; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
// 信息 ID: 8bd5e75a6f27bf6037b5b649
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass; module=530, inline_record=94772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
// 信息 ID: 4aebea2fd72059b77d2105b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass; module=530, inline_record=94796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
// 信息 ID: f78f8a293de4c11999e94a74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=33, inline_record=23016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg; module=487, inline_record=46600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay; module=487, inline_record=48668; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::painterly::painterly_pass; module=487, inline_record=58484; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_render_from_target; module=487, inline_record=70056; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_render_from_target; module=487, inline_record=72096; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=489, inline_record=26732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::menu_background::render_menu_bg; module=495, inline_record=89036; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
// 信息 ID: e10153962ad4aee36782f687
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=33, inline_record=23040; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg; module=487, inline_record=46624; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay; module=487, inline_record=48692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::painterly::painterly_pass; module=487, inline_record=58508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_render_from_target; module=487, inline_record=70080; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::vermeer_render_from_target; module=487, inline_record=72120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=489, inline_record=26756; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::menu_background::render_menu_bg; module=495, inline_record=89060; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
// 信息 ID: 2b584bc8e9546d94117cbf89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass; module=529, inline_record=33484; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass; module=529, inline_record=47000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass; module=529, inline_record=49348; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=77372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
// 信息 ID: 4eea076f3a073282f970f2ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass; module=529, inline_record=33508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass; module=529, inline_record=47024; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass; module=529, inline_record=49372; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=77396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode
// 信息 ID: 851c4012f74dd63f7a95f427
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass; module=530, inline_record=98132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode_checked
// 信息 ID: d9c6c4670bd5548aeb87ab35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass; module=530, inline_record=98156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask
// 信息 ID: aa5e198cc9f917eebda19ea6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=78888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask_checked
// 信息 ID: a440d73e8dc4098c2b02be70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=78912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op
// 信息 ID: 726e602f4f567b91a5d4fc86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=78804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op_checked
// 信息 ID: fe6a7014c2840fd77e7b8d8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=78828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
// 信息 ID: b16e4ca934def2b2f9bb69e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=21820; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=23960; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=26100; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=29104; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=31244; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=39008; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=41148; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=43288; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
// 信息 ID: 7f7ed3055adc32a33e5f1909
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=21844; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=23984; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=26124; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=29128; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=31268; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=39032; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=41172; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=43312; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
// 信息 ID: 6af78eec5407b845af1fad51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=21736; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=23876; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=26016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=29020; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=31160; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=38924; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=41064; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=43204; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
// 信息 ID: 6018b55969a8785b597c5e1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=21760; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=23900; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=26040; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=29044; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=31184; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=38948; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=41088; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=43228; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_topology
// 信息 ID: f6b8c939512d261edc81255c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$57,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=13364; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$35,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=65276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
// 信息 ID: c2f5d5955ebe44d7d0747e55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$57,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=13384; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$35,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=65296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::topology
// 信息 ID: 7e6f212df0c05637471b6514
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=31816; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1657, inline_record=14116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::topology_or_err
// 信息 ID: 0d78784f84a1a2ad10519ed6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1657, inline_record=14144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RenderPassAttachmentDesc::clear_input
// 信息 ID: 35bc34abbfca486c52037507
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x8685ac -> 0xbe67a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RenderPassAttachmentDesc::new
// 信息 ID: bd169f88ed521f6fba75ef48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x868440 -> 0xbe6780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x868552 -> 0xbe6780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count
// 信息 ID: 018a32dbe58df2c741682a22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x86845b -> 0xbe67c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x868578 -> 0xbe67c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0> >
// 信息 ID: 4e7be4616e16da9d1903144b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["shader_pipeline::reflection::Binding"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::enumerate::Enumerate -> ["core::slice::iter::Iter<shader_pipeline::reflection::Binding>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >", "rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::shader::create_descriptor_set_layout -> ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set @ 0xbff006 -> 0x18c2150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0> >
// 信息 ID: c1bd5237a8614e1b7d783e83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::collections::hash::map::Iter -> ["shader_pipeline::reflection::BindingIndex", "shader_pipeline::reflection::Binding"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>", "rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::shader::create_descriptor_set_layout -> ["core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::shader::create_descriptor_set_layouts @ 0xbe6284 -> 0x18c1a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layouts
// 信息 ID: b24e8809b96d0b5ac54c7d8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a3792 -> 0xbe60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a4b94 -> 0xbe60a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layouts::closure$1
// 信息 ID: c177cf03aaac5838e4d3f9ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::shader::create_descriptor_set_layouts", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$1 <- std::collections::hash::map::impl$87::from_iter<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::DescriptorSetLayoutBinding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] std::collections::hash::map::impl$87::from_iter<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::DescriptorSetLayoutBinding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$1> >; module=1966, inline_record=1836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_states
// 信息 ID: 84770ae6984e0bfcaf5b331a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a43a0 -> 0xbe66c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a4e9c -> 0xbe66c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_states::closure$0
// 信息 ID: 4bab97705c97045d83179c23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::create_descriptor_set_states", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_states::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_states::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_states::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_states::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >; module=1935, inline_record=8540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_states::closure$0::closure$0
// 信息 ID: f37b54793bf20f40db628cce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::create_descriptor_set_states", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >; module=1935, inline_record=10360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::get_vk_blend_mode
// 信息 ID: 5661824eec44f45f32ee1f7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a3f87 -> 0xbe67f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$118::eq
// 信息 ID: b92230e3885eb8951fc836ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shader::RasterShaderProgramVariant::get_or_generate; module=509, inline_record=68260; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>; module=1613, inline_record=46076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$120::hash
// 信息 ID: 24164bf54bcf4c265d237d08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::shader::RasterPipelineDesc> >; module=1582, inline_record=26800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$123::eq
// 信息 ID: cf9bd0b14bbe74022a3a6989
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shader::RasterShaderProgramVariant::get_or_generate; module=509, inline_record=68204; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,alloc::boxed::Box<rhapsody::raw::shader::RasterPipeline,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>; module=1613, inline_record=46020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$125::hash
// 信息 ID: 21bc14ad503057233be73de1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::shader::RasterPipelineDesc> >; module=1582, inline_record=26404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$127::clone
// 信息 ID: bfd26b4919aa2a0eebdc0771
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=482, inline_record=14992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$12::create_raster_pipeline::closure$0
// 信息 ID: 97638ef13acb4536489cd73b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::impl$12::create_raster_pipeline", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::PipelineShaderStageCreateInfo,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::PipelineShaderStageCreateInfo,core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::PipelineShaderStageCreateInfo,core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::PipelineShaderStageCreateInfo,core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > > > >; module=1610, inline_record=59384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$130::eq
// 信息 ID: ddef6c09b6d159d91c58f4da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>; module=1613, inline_record=42096; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>; module=1613, inline_record=42316; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>; module=1613, inline_record=42780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$132::hash
// 信息 ID: 4873b65f89b04ea3a0515c0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::render_pass::RenderPassDesc> >; module=1582, inline_record=50692; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$27::hash<rhapsody::raw::shader::RenderPassAttachmentDesc,8,std::hash::random::DefaultHasher>; module=1623, inline_record=20744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$2::deref
// 信息 ID: 250691b8a462562814a3e89e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 1177 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43b79 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43cba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x44288 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45e63 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45c78 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x457de -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::InitData::on_create<tuple$<> > @ 0x7d1dc -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::init<tuple$<> > @ 0x7e616 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd797c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd7a85 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0xd8010 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdbaba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdba0a -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea3b -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea5c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdeef4 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$3::deref
// 信息 ID: ce241cddd1e2dcdbc6929639
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 1177 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43b79 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43cba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x44288 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45e63 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45c78 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x457de -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::InitData::on_create<tuple$<> > @ 0x7d1dc -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::init<tuple$<> > @ 0x7e616 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd797c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd7a85 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0xd8010 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdbaba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdba0a -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea3b -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea5c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdeef4 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$5::create_compute_pipeline::closure$0
// 信息 ID: f2377d38aae0edad1452668d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::impl$5::create_compute_pipeline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1657, inline_record=21172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$7::default
// 信息 ID: 9df52bd3965908aec9ca9df7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 48 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x83400f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x83436f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x8346df -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834a9f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834dff -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x83514f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x835e8f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x8361ef -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x83654f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x83688f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836bcf -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x83758f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x8378ef -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass @ 0x837ddf -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass @ 0x83818f -> 0xbe6750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::DrawPacket::new @ 0x867868 -> 0xbe6750 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::map_vertex_attribute_format
// 信息 ID: bd2636229855fc4e1b117dff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a3b5b -> 0xbe6850 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a3b91 -> 0xbe6850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
