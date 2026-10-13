// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/pass_builder/bind_resources.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 523a194a9fabd6d1942e05f60d7eb3971f7d935d485a5ead0b93345ee56ae097
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bind_resources.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 115, 1318, 1324
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use shader_pipeline; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1318,"record_offset":19624,"rva":14461760,"va":5383170880}
// 编译实例 RVAs: 0xdcab40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8955550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383170880,"raw_hex":"e8625977ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5383171225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7192712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383170880,"raw_hex":"e8783f92ff","size":5,"soft_fail":false,"target_va":5375978624,"va":5383171331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20422187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383170880,"raw_hex":"e82b9e3701","size":5,"soft_fail":false,"target_va":5403593360,"va":5383171168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19808998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383170880,"raw_hex":"e8e6422e01","size":5,"soft_fail":false,"target_va":5402980288,"va":5383171285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1318,"record_offset":21244,"parent_record_offset":19624,"ancestor_record_offset":19624,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1318,"record_offset":19860,"parent_record_offset":19624,"ancestor_record_offset":19624,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1318,"record_offset":20716,"parent_record_offset":20692,"ancestor_record_offset":19624,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1318,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":19624,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1318,"record_offset":20352,"parent_record_offset":20328,"ancestor_record_offset":19624,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1318,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":19624,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1318,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":19624,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":20028,"parent_record_offset":19964,"ancestor_record_offset":19624,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":19908,"parent_record_offset":19860,"ancestor_record_offset":19624,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":20000,"parent_record_offset":19964,"ancestor_record_offset":19624,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1318,"record_offset":21168,"parent_record_offset":21144,"ancestor_record_offset":19624,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1318,"record_offset":20628,"parent_record_offset":20604,"ancestor_record_offset":19624,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1318,"record_offset":20604,"parent_record_offset":20580,"ancestor_record_offset":19624,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1318,"record_offset":21052,"parent_record_offset":19964,"ancestor_record_offset":19624,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1318,"record_offset":21080,"parent_record_offset":21052,"ancestor_record_offset":19624,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1318,"record_offset":19964,"parent_record_offset":19936,"ancestor_record_offset":19624,"inlinee":7402,"source_line":53,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1318,"record_offset":19936,"parent_record_offset":19860,"ancestor_record_offset":19624,"inlinee":8170,"source_line":37,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1318,"record_offset":20056,"parent_record_offset":20028,"ancestor_record_offset":19624,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1318,"record_offset":21144,"parent_record_offset":21120,"ancestor_record_offset":19624,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$24::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1318,"record_offset":21120,"parent_record_offset":19964,"ancestor_record_offset":19624,"inlinee":61721,"source_line":55,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1318,"record_offset":20692,"parent_record_offset":20668,"ancestor_record_offset":19624,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":19624,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":19624,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20668,"parent_record_offset":20532,"ancestor_record_offset":19624,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":19624,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":19624,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20936,"parent_record_offset":20912,"ancestor_record_offset":19624,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20876,"parent_record_offset":20204,"ancestor_record_offset":19624,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20912,"parent_record_offset":20176,"ancestor_record_offset":19624,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20176,"parent_record_offset":20152,"ancestor_record_offset":19624,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20152,"parent_record_offset":20128,"ancestor_record_offset":19624,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20204,"parent_record_offset":20176,"ancestor_record_offset":19624,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1318,"record_offset":20412,"parent_record_offset":20384,"ancestor_record_offset":19624,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1318,"record_offset":20440,"parent_record_offset":20412,"ancestor_record_offset":19624,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1318,"record_offset":20500,"parent_record_offset":20384,"ancestor_record_offset":19624,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1318,"record_offset":20384,"parent_record_offset":20204,"ancestor_record_offset":19624,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":20532,"parent_record_offset":20204,"ancestor_record_offset":19624,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1318,"record_offset":20244,"parent_record_offset":20204,"ancestor_record_offset":19624,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1318,"record_offset":20328,"parent_record_offset":20204,"ancestor_record_offset":19624,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1318,"record_offset":20784,"parent_record_offset":20204,"ancestor_record_offset":19624,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1318,"record_offset":20740,"parent_record_offset":20716,"ancestor_record_offset":19624,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1318,"record_offset":20080,"parent_record_offset":20056,"ancestor_record_offset":19624,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1324,"record_offset":248,"rva":14634464,"va":5383343584}
// 编译实例 RVAs: 0xdf4de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9125518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383343584,"raw_hex":"e872c174ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5383343929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7376488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383343584,"raw_hex":"e898718fff","size":5,"soft_fail":false,"target_va":5375967552,"va":5383344035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20249483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383343584,"raw_hex":"e88bfb3401","size":5,"soft_fail":false,"target_va":5403593360,"va":5383343872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19636294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383343584,"raw_hex":"e846a02b01","size":5,"soft_fail":false,"target_va":5402980288,"va":5383343989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1324,"record_offset":1860,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1324,"record_offset":476,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1324,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":248,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1324,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1324,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":248,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1324,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":248,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1324,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":248,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":644,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":524,"parent_record_offset":476,"ancestor_record_offset":248,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":616,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":248,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":248,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":248,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1324,"record_offset":1668,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1324,"record_offset":1696,"parent_record_offset":1668,"ancestor_record_offset":248,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1324,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":248,"inlinee":7402,"source_line":53,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1324,"record_offset":552,"parent_record_offset":476,"ancestor_record_offset":248,"inlinee":8170,"source_line":37,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1324,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":248,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1324,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":248,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$24::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1324,"record_offset":1736,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":61721,"source_line":55,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1324,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":248,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":248,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":248,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":1284,"parent_record_offset":1148,"ancestor_record_offset":248,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":248,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":248,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":248,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":1492,"parent_record_offset":820,"ancestor_record_offset":248,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":1528,"parent_record_offset":792,"ancestor_record_offset":248,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":248,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":248,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":820,"parent_record_offset":792,"ancestor_record_offset":248,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":1028,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":1056,"parent_record_offset":1028,"ancestor_record_offset":248,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":1116,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":1000,"parent_record_offset":820,"ancestor_record_offset":248,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":1148,"parent_record_offset":820,"ancestor_record_offset":248,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":860,"parent_record_offset":820,"ancestor_record_offset":248,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":944,"parent_record_offset":820,"ancestor_record_offset":248,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":1400,"parent_record_offset":820,"ancestor_record_offset":248,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1324,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":248,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":248,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 记录形式: compiled_procedure; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":115,"record_offset":14524,"rva":1738560,"va":5370447680}
// 编译实例 RVAs: 0x1a8740, 0xdcad60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140065230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1324369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370447680,"raw_hex":"e8afcaebff","size":5,"soft_fail":false,"target_va":5369123376,"va":5370447740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2003088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370447680,"raw_hex":"e890901e00","size":5,"soft_fail":false,"target_va":5372451200,"va":5370448107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7193232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171424,"raw_hex":"e8703d92ff","size":5,"soft_fail":false,"target_va":5375978624,"va":5383171851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dded90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t81903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171424,"raw_hex":"e8ef3f0100","size":5,"soft_fail":false,"target_va":5383253392,"va":5383171484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209ce00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32458038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370447680,"raw_hex":"e83645ef01","size":5,"soft_fail":false,"target_va":5402906112,"va":5370448069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19786134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171424,"raw_hex":"e896e92d01","size":5,"soft_fail":false,"target_va":5402957952,"va":5383171813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t859129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370447680,"raw_hex":"e8f91b0d00","size":5,"soft_fail":false,"target_va":5371307104,"va":5370447970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t33145328","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370447680,"raw_hex":"e9f0c1f901","size":5,"soft_fail":false,"target_va":5403593360,"va":5370448027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":115,"record_offset":14760,"parent_record_offset":14524,"ancestor_record_offset":14524,"inlinee":10164,"source_line":717,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":115,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":14524,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":115,"record_offset":15892,"parent_record_offset":15856,"ancestor_record_offset":14524,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":115,"record_offset":15856,"parent_record_offset":14760,"ancestor_record_offset":14524,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":115,"record_offset":15472,"parent_record_offset":15448,"ancestor_record_offset":14524,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":115,"record_offset":15016,"parent_record_offset":14992,"ancestor_record_offset":14524,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":115,"record_offset":15100,"parent_record_offset":15076,"ancestor_record_offset":14524,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":115,"record_offset":15040,"parent_record_offset":15016,"ancestor_record_offset":14524,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":115,"record_offset":15216,"parent_record_offset":15192,"ancestor_record_offset":14524,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":115,"record_offset":15800,"parent_record_offset":14760,"ancestor_record_offset":14524,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":115,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":14524,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":115,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":14524,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":115,"record_offset":15948,"parent_record_offset":15856,"ancestor_record_offset":14524,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1318,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":21296,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":115,"record_offset":15448,"parent_record_offset":15424,"ancestor_record_offset":14524,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":21612,"parent_record_offset":21588,"ancestor_record_offset":21296,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":115,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":14524,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":21636,"parent_record_offset":21612,"ancestor_record_offset":21296,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":115,"record_offset":14864,"parent_record_offset":14840,"ancestor_record_offset":14524,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":22196,"parent_record_offset":22056,"ancestor_record_offset":21296,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":115,"record_offset":15424,"parent_record_offset":15284,"ancestor_record_offset":14524,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":15336,"parent_record_offset":15312,"ancestor_record_offset":14524,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":15312,"parent_record_offset":15284,"ancestor_record_offset":14524,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":15692,"parent_record_offset":15668,"ancestor_record_offset":14524,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":15632,"parent_record_offset":14944,"ancestor_record_offset":14524,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":15668,"parent_record_offset":14916,"ancestor_record_offset":14524,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":14916,"parent_record_offset":14892,"ancestor_record_offset":14524,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":14892,"parent_record_offset":14864,"ancestor_record_offset":14524,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":14944,"parent_record_offset":14916,"ancestor_record_offset":14524,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":115,"record_offset":15164,"parent_record_offset":15132,"ancestor_record_offset":14524,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":115,"record_offset":15192,"parent_record_offset":15164,"ancestor_record_offset":14524,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":115,"record_offset":15252,"parent_record_offset":15132,"ancestor_record_offset":14524,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":115,"record_offset":15132,"parent_record_offset":14944,"ancestor_record_offset":14524,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":15284,"parent_record_offset":14944,"ancestor_record_offset":14524,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":115,"record_offset":14992,"parent_record_offset":14944,"ancestor_record_offset":14524,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":115,"record_offset":15076,"parent_record_offset":14944,"ancestor_record_offset":14524,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":115,"record_offset":15540,"parent_record_offset":14944,"ancestor_record_offset":14524,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":115,"record_offset":15496,"parent_record_offset":15472,"ancestor_record_offset":14524,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":115,"record_offset":15828,"parent_record_offset":14760,"ancestor_record_offset":14524,"inlinee":7396,"source_line":75,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":115,"record_offset":14816,"parent_record_offset":14760,"ancestor_record_offset":14524,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1324,"record_offset":1912,"rva":14635008,"va":5383344128}
// 编译实例 RVAs: 0xdf5000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7377282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344128,"raw_hex":"e87e6e8fff","size":5,"soft_fail":false,"target_va":5375967552,"va":5383344829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dded90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-90866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344128,"raw_hex":"e80e9dfeff","size":5,"soft_fail":false,"target_va":5383253392,"va":5383344253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f2f9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1288539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344128,"raw_hex":"e85ba91300","size":5,"soft_fail":false,"target_va":5384632736,"va":5383344192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19613158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344128,"raw_hex":"e8e6452b01","size":5,"soft_fail":false,"target_va":5402957952,"va":5383344789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20248628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344128,"raw_hex":"e834f83401","size":5,"soft_fail":false,"target_va":5403593360,"va":5383344727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12037588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344128,"raw_hex":"e82c5248ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5383344687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1324,"record_offset":2140,"parent_record_offset":1912,"ancestor_record_offset":1912,"inlinee":61896,"source_line":512,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1324,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":1912,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1324,"record_offset":4612,"parent_record_offset":4576,"ancestor_record_offset":1912,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1324,"record_offset":4576,"parent_record_offset":2140,"ancestor_record_offset":1912,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1324,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":1912,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1324,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":1912,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1324,"record_offset":2580,"parent_record_offset":2552,"ancestor_record_offset":1912,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1324,"record_offset":2744,"parent_record_offset":2688,"ancestor_record_offset":1912,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1324,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":1912,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1324,"record_offset":3160,"parent_record_offset":3128,"ancestor_record_offset":1912,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1324,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":1912,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":4520,"parent_record_offset":2140,"ancestor_record_offset":1912,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":3288,"parent_record_offset":3260,"ancestor_record_offset":1912,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":3260,"parent_record_offset":3232,"ancestor_record_offset":1912,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":1912,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1324,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":1912,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1324,"record_offset":2300,"parent_record_offset":2264,"ancestor_record_offset":1912,"inlinee":61898,"source_line":521,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1324,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":1912,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":3544,"parent_record_offset":3512,"ancestor_record_offset":1912,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":3576,"parent_record_offset":3544,"ancestor_record_offset":1912,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":4144,"parent_record_offset":4004,"ancestor_record_offset":1912,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":1912,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":4032,"parent_record_offset":4004,"ancestor_record_offset":1912,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3232,"parent_record_offset":3204,"ancestor_record_offset":1912,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3356,"parent_record_offset":2384,"ancestor_record_offset":1912,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2784,"parent_record_offset":2468,"ancestor_record_offset":1912,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":4388,"parent_record_offset":3636,"ancestor_record_offset":1912,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":1912,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3612,"parent_record_offset":3576,"ancestor_record_offset":1912,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2384,"parent_record_offset":2356,"ancestor_record_offset":1912,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3204,"parent_record_offset":2384,"ancestor_record_offset":1912,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3664,"parent_record_offset":3636,"ancestor_record_offset":1912,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2468,"parent_record_offset":2432,"ancestor_record_offset":1912,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2648,"parent_record_offset":2468,"ancestor_record_offset":1912,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2812,"parent_record_offset":2468,"ancestor_record_offset":1912,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2844,"parent_record_offset":2812,"ancestor_record_offset":1912,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2432,"parent_record_offset":2384,"ancestor_record_offset":1912,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2524,"parent_record_offset":2468,"ancestor_record_offset":1912,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3104,"parent_record_offset":2432,"ancestor_record_offset":1912,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":3012,"parent_record_offset":2812,"ancestor_record_offset":1912,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":1912,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":3972,"parent_record_offset":3852,"ancestor_record_offset":1912,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":3852,"parent_record_offset":3664,"ancestor_record_offset":1912,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":1912,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":4004,"parent_record_offset":3664,"ancestor_record_offset":1912,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":1912,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":3328,"parent_record_offset":2384,"ancestor_record_offset":1912,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":2552,"parent_record_offset":2468,"ancestor_record_offset":1912,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":2900,"parent_record_offset":2812,"ancestor_record_offset":1912,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":3796,"parent_record_offset":3664,"ancestor_record_offset":1912,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":4260,"parent_record_offset":3664,"ancestor_record_offset":1912,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":2688,"parent_record_offset":2648,"ancestor_record_offset":1912,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1324,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":1912,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1324,"record_offset":2356,"parent_record_offset":2328,"ancestor_record_offset":1912,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1324,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":1912,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1324,"record_offset":4548,"parent_record_offset":2140,"ancestor_record_offset":1912,"inlinee":7396,"source_line":75,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":2264,"parent_record_offset":2140,"ancestor_record_offset":1912,"inlinee":12248,"source_line":2663,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":2208,"parent_record_offset":2140,"ancestor_record_offset":1912,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":3512,"parent_record_offset":2140,"ancestor_record_offset":1912,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":3452,"parent_record_offset":2264,"ancestor_record_offset":1912,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":2328,"parent_record_offset":2264,"ancestor_record_offset":1912,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":1912,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1318,"record_offset":22760,"rva":14462752,"va":5383171872}
// 编译实例 RVAs: 0xdcaf20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7748846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e812c389ff","size":5,"soft_fail":false,"target_va":5375423344,"va":5383172185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7196092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e8443292ff","size":5,"soft_fail":false,"target_va":5375978624,"va":5383174711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddcc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t72147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e8d3190100","size":5,"soft_fail":false,"target_va":5383244944,"va":5383172792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddd590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t73827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e863200100","size":5,"soft_fail":false,"target_va":5383247248,"va":5383173416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddda90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t75412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e894260100","size":5,"soft_fail":false,"target_va":5383248528,"va":5383173111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dddc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t76540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e8fc2a0100","size":5,"soft_fail":false,"target_va":5383249040,"va":5383172495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9880
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19783462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e826df2d01","size":5,"soft_fail":false,"target_va":5402957952,"va":5383174485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19784195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e803e22d01","size":5,"soft_fail":false,"target_va":5402958528,"va":5383174328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20421078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e8d6993701","size":5,"soft_fail":false,"target_va":5403593344,"va":5383172261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20420891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e81b993701","size":5,"soft_fail":false,"target_va":5403593360,"va":5383172464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-11865613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e8f3f14aff","size":5,"soft_fail":false,"target_va":5371307104,"va":5383172712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19804600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e8b8312e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5383174275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19805634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383171872,"raw_hex":"e8c2352e01","size":5,"soft_fail":false,"target_va":5402980288,"va":5383174649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5383171872,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5383172292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":1318,"record_offset":23012,"parent_record_offset":22760,"ancestor_record_offset":22760,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":24768,"parent_record_offset":22760,"ancestor_record_offset":22760,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1318,"record_offset":24528,"parent_record_offset":22760,"ancestor_record_offset":22760,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1318,"record_offset":25080,"parent_record_offset":22760,"ancestor_record_offset":22760,"inlinee":10164,"source_line":717,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1318,"record_offset":30084,"parent_record_offset":22760,"ancestor_record_offset":22760,"inlinee":10166,"source_line":666,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1318,"record_offset":30000,"parent_record_offset":22760,"ancestor_record_offset":22760,"inlinee":21976,"source_line":371,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1318,"record_offset":26240,"parent_record_offset":26216,"ancestor_record_offset":22760,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1318,"record_offset":26216,"parent_record_offset":26180,"ancestor_record_offset":22760,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1318,"record_offset":26180,"parent_record_offset":25080,"ancestor_record_offset":22760,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":1318,"record_offset":23040,"parent_record_offset":23012,"ancestor_record_offset":22760,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1318,"record_offset":24484,"parent_record_offset":24452,"ancestor_record_offset":22760,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1318,"record_offset":24428,"parent_record_offset":24040,"ancestor_record_offset":22760,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1318,"record_offset":24452,"parent_record_offset":24428,"ancestor_record_offset":22760,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1318,"record_offset":24068,"parent_record_offset":24040,"ancestor_record_offset":22760,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1318,"record_offset":24116,"parent_record_offset":24092,"ancestor_record_offset":22760,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1318,"record_offset":24092,"parent_record_offset":24068,"ancestor_record_offset":22760,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1318,"record_offset":24208,"parent_record_offset":24144,"ancestor_record_offset":22760,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1318,"record_offset":24296,"parent_record_offset":24272,"ancestor_record_offset":22760,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":1318,"record_offset":24040,"parent_record_offset":23012,"ancestor_record_offset":22760,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":1318,"record_offset":23772,"parent_record_offset":23748,"ancestor_record_offset":22760,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1318,"record_offset":23820,"parent_record_offset":23796,"ancestor_record_offset":22760,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1318,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":22760,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1318,"record_offset":23236,"parent_record_offset":23212,"ancestor_record_offset":22760,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1318,"record_offset":23320,"parent_record_offset":23296,"ancestor_record_offset":22760,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1318,"record_offset":23260,"parent_record_offset":23236,"ancestor_record_offset":22760,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1318,"record_offset":23432,"parent_record_offset":23408,"ancestor_record_offset":22760,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":30256,"parent_record_offset":30192,"ancestor_record_offset":22760,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":30132,"parent_record_offset":30084,"ancestor_record_offset":22760,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":24232,"parent_record_offset":24208,"ancestor_record_offset":22760,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":24008,"parent_record_offset":23040,"ancestor_record_offset":22760,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1318,"record_offset":24272,"parent_record_offset":24092,"ancestor_record_offset":22760,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1318,"record_offset":24376,"parent_record_offset":24352,"ancestor_record_offset":22760,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1318,"record_offset":31372,"parent_record_offset":31348,"ancestor_record_offset":22760,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1318,"record_offset":23660,"parent_record_offset":23636,"ancestor_record_offset":22760,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1318,"record_offset":23580,"parent_record_offset":23556,"ancestor_record_offset":22760,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1318,"record_offset":26272,"parent_record_offset":26180,"ancestor_record_offset":22760,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1318,"record_offset":24680,"parent_record_offset":24652,"ancestor_record_offset":22760,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1318,"record_offset":24708,"parent_record_offset":24680,"ancestor_record_offset":22760,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1318,"record_offset":24144,"parent_record_offset":24116,"ancestor_record_offset":22760,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1318,"record_offset":31280,"parent_record_offset":30192,"ancestor_record_offset":22760,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1318,"record_offset":24180,"parent_record_offset":24144,"ancestor_record_offset":22760,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1318,"record_offset":24352,"parent_record_offset":24320,"ancestor_record_offset":22760,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1318,"record_offset":31308,"parent_record_offset":31280,"ancestor_record_offset":22760,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1318,"record_offset":24624,"parent_record_offset":24596,"ancestor_record_offset":22760,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1318,"record_offset":30024,"parent_record_offset":30000,"ancestor_record_offset":22760,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1318,"record_offset":30048,"parent_record_offset":30024,"ancestor_record_offset":22760,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1318,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":22760,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1318,"record_offset":24556,"parent_record_offset":24528,"ancestor_record_offset":22760,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1318,"record_offset":30192,"parent_record_offset":30160,"ancestor_record_offset":22760,"inlinee":7402,"source_line":53,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1318,"record_offset":30160,"parent_record_offset":30084,"ancestor_record_offset":22760,"inlinee":8170,"source_line":37,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1318,"record_offset":30284,"parent_record_offset":30256,"ancestor_record_offset":22760,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1318,"record_offset":31348,"parent_record_offset":30192,"ancestor_record_offset":22760,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1318,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":22760,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":23064,"parent_record_offset":23040,"ancestor_record_offset":22760,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":23088,"parent_record_offset":23064,"ancestor_record_offset":22760,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":23700,"parent_record_offset":23500,"ancestor_record_offset":22760,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":25660,"parent_record_offset":25636,"ancestor_record_offset":22760,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":25636,"parent_record_offset":25608,"ancestor_record_offset":22760,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":26016,"parent_record_offset":25992,"ancestor_record_offset":22760,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":25956,"parent_record_offset":25268,"ancestor_record_offset":22760,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":25992,"parent_record_offset":25240,"ancestor_record_offset":22760,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":25240,"parent_record_offset":25216,"ancestor_record_offset":22760,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":25216,"parent_record_offset":25188,"ancestor_record_offset":22760,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":25268,"parent_record_offset":25240,"ancestor_record_offset":22760,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1318,"record_offset":25488,"parent_record_offset":25456,"ancestor_record_offset":22760,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1318,"record_offset":25516,"parent_record_offset":25488,"ancestor_record_offset":22760,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1318,"record_offset":25576,"parent_record_offset":25456,"ancestor_record_offset":22760,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1318,"record_offset":25456,"parent_record_offset":25268,"ancestor_record_offset":22760,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1318,"record_offset":25608,"parent_record_offset":25268,"ancestor_record_offset":22760,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1318,"record_offset":23612,"parent_record_offset":23528,"ancestor_record_offset":22760,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1318,"record_offset":23556,"parent_record_offset":23528,"ancestor_record_offset":22760,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1318,"record_offset":23956,"parent_record_offset":23164,"ancestor_record_offset":22760,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1318,"record_offset":23528,"parent_record_offset":23500,"ancestor_record_offset":22760,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1318,"record_offset":23140,"parent_record_offset":23116,"ancestor_record_offset":22760,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1318,"record_offset":23116,"parent_record_offset":23088,"ancestor_record_offset":22760,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1318,"record_offset":23164,"parent_record_offset":23140,"ancestor_record_offset":22760,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":1318,"record_offset":23380,"parent_record_offset":23352,"ancestor_record_offset":22760,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":1318,"record_offset":23408,"parent_record_offset":23380,"ancestor_record_offset":22760,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":1318,"record_offset":23468,"parent_record_offset":23352,"ancestor_record_offset":22760,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":1318,"record_offset":23352,"parent_record_offset":23164,"ancestor_record_offset":22760,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1318,"record_offset":23500,"parent_record_offset":23164,"ancestor_record_offset":22760,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":1318,"record_offset":23212,"parent_record_offset":23164,"ancestor_record_offset":22760,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":1318,"record_offset":23296,"parent_record_offset":23164,"ancestor_record_offset":22760,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":1318,"record_offset":23872,"parent_record_offset":23164,"ancestor_record_offset":22760,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1318,"record_offset":25316,"parent_record_offset":25268,"ancestor_record_offset":22760,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1318,"record_offset":25400,"parent_record_offset":25268,"ancestor_record_offset":22760,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1318,"record_offset":25864,"parent_record_offset":25268,"ancestor_record_offset":22760,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1318,"record_offset":24320,"parent_record_offset":24296,"ancestor_record_offset":22760,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1318,"record_offset":25820,"parent_record_offset":25796,"ancestor_record_offset":22760,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1318,"record_offset":26152,"parent_record_offset":25080,"ancestor_record_offset":22760,"inlinee":7396,"source_line":75,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1318,"record_offset":25140,"parent_record_offset":25080,"ancestor_record_offset":22760,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1324,"record_offset":4708,"rva":14635744,"va":5383344864}
// 编译实例 RVAs: 0xdf52e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7921838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e8521f87ff","size":5,"soft_fail":false,"target_va":5375423344,"va":5383345177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7380642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e85e618fff","size":5,"soft_fail":false,"target_va":5375967552,"va":5383348189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddcc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-101191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e8b974feff","size":5,"soft_fail":false,"target_va":5383244944,"va":5383346130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddda90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-98203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e86580feff","size":5,"soft_fail":false,"target_va":5383248528,"va":5383346726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dddc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-96534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e8ea86feff","size":5,"soft_fail":false,"target_va":5383249040,"va":5383345569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfabb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e89b4f0000","size":5,"soft_fail":false,"target_va":5383367600,"va":5383347216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f2f9a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1287242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e84aa41300","size":5,"soft_fail":false,"target_va":5384632736,"va":5383345489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9880
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19609959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e867392b01","size":5,"soft_fail":false,"target_va":5402957952,"va":5383347988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19610692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e8443c2b01","size":5,"soft_fail":false,"target_va":5402958528,"va":5383347831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20248086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e816f63401","size":5,"soft_fail":false,"target_va":5403593344,"va":5383345253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: bind_resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20247902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e85ef53401","size":5,"soft_fail":false,"target_va":5403593360,"va":5383345453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-12038891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e8154d48ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5383345990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19631097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e8f98b2b01","size":5,"soft_fail":false,"target_va":5402978880,"va":5383347778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19632181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383344864,"raw_hex":"e835902b01","size":5,"soft_fail":false,"target_va":5402980288,"va":5383348102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5383344864,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5383345284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":1324,"record_offset":4952,"parent_record_offset":4708,"ancestor_record_offset":4708,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":6708,"parent_record_offset":4708,"ancestor_record_offset":4708,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1324,"record_offset":6468,"parent_record_offset":4708,"ancestor_record_offset":4708,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1324,"record_offset":7020,"parent_record_offset":4708,"ancestor_record_offset":4708,"inlinee":61896,"source_line":512,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1324,"record_offset":14716,"parent_record_offset":4708,"ancestor_record_offset":4708,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1324,"record_offset":14632,"parent_record_offset":4708,"ancestor_record_offset":4708,"inlinee":21976,"source_line":371,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1324,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":4708,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1324,"record_offset":9464,"parent_record_offset":9428,"ancestor_record_offset":4708,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1324,"record_offset":9428,"parent_record_offset":7020,"ancestor_record_offset":4708,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":1324,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":4708,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1324,"record_offset":6424,"parent_record_offset":6392,"ancestor_record_offset":4708,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1324,"record_offset":6368,"parent_record_offset":5980,"ancestor_record_offset":4708,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1324,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":4708,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1324,"record_offset":6008,"parent_record_offset":5980,"ancestor_record_offset":4708,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1324,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":4708,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1324,"record_offset":6032,"parent_record_offset":6008,"ancestor_record_offset":4708,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1324,"record_offset":6148,"parent_record_offset":6084,"ancestor_record_offset":4708,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1324,"record_offset":6236,"parent_record_offset":6212,"ancestor_record_offset":4708,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":1324,"record_offset":5980,"parent_record_offset":4952,"ancestor_record_offset":4708,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":1324,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":4708,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1324,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":4708,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1324,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":4708,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1324,"record_offset":7788,"parent_record_offset":7764,"ancestor_record_offset":4708,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1324,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":4708,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1324,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":4708,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1324,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":4708,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1324,"record_offset":8024,"parent_record_offset":7992,"ancestor_record_offset":4708,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1324,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4708,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":14888,"parent_record_offset":14824,"ancestor_record_offset":4708,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":14764,"parent_record_offset":14716,"ancestor_record_offset":4708,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":4708,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":5948,"parent_record_offset":4980,"ancestor_record_offset":4708,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1324,"record_offset":6212,"parent_record_offset":6032,"ancestor_record_offset":4708,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1324,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":4708,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":4708,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":4708,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":4708,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1324,"record_offset":8244,"parent_record_offset":8220,"ancestor_record_offset":4708,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1324,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":4708,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1324,"record_offset":6620,"parent_record_offset":6592,"ancestor_record_offset":4708,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1324,"record_offset":6648,"parent_record_offset":6620,"ancestor_record_offset":4708,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1324,"record_offset":6084,"parent_record_offset":6056,"ancestor_record_offset":4708,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1324,"record_offset":15912,"parent_record_offset":14824,"ancestor_record_offset":4708,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1324,"record_offset":6120,"parent_record_offset":6084,"ancestor_record_offset":4708,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1324,"record_offset":6292,"parent_record_offset":6260,"ancestor_record_offset":4708,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1324,"record_offset":15940,"parent_record_offset":15912,"ancestor_record_offset":4708,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1324,"record_offset":6564,"parent_record_offset":6536,"ancestor_record_offset":4708,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1324,"record_offset":14656,"parent_record_offset":14632,"ancestor_record_offset":4708,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1324,"record_offset":14680,"parent_record_offset":14656,"ancestor_record_offset":4708,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1324,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":4708,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1324,"record_offset":6496,"parent_record_offset":6468,"ancestor_record_offset":4708,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1324,"record_offset":7180,"parent_record_offset":7148,"ancestor_record_offset":4708,"inlinee":61898,"source_line":521,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1324,"record_offset":14824,"parent_record_offset":14792,"ancestor_record_offset":4708,"inlinee":7402,"source_line":53,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1324,"record_offset":14792,"parent_record_offset":14716,"ancestor_record_offset":4708,"inlinee":8170,"source_line":37,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1324,"record_offset":14916,"parent_record_offset":14888,"ancestor_record_offset":4708,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1324,"record_offset":15980,"parent_record_offset":14824,"ancestor_record_offset":4708,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1324,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":4708,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":4708,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":5028,"parent_record_offset":5004,"ancestor_record_offset":4708,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1324,"record_offset":5640,"parent_record_offset":5440,"ancestor_record_offset":4708,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":4708,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8884,"parent_record_offset":8856,"ancestor_record_offset":4708,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8096,"parent_record_offset":8068,"ancestor_record_offset":4708,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8220,"parent_record_offset":7256,"ancestor_record_offset":4708,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7648,"parent_record_offset":7332,"ancestor_record_offset":4708,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":9240,"parent_record_offset":8488,"ancestor_record_offset":4708,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8488,"parent_record_offset":8464,"ancestor_record_offset":4708,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8464,"parent_record_offset":8432,"ancestor_record_offset":4708,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7256,"parent_record_offset":7232,"ancestor_record_offset":4708,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8068,"parent_record_offset":7256,"ancestor_record_offset":4708,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8516,"parent_record_offset":8488,"ancestor_record_offset":4708,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7332,"parent_record_offset":7300,"ancestor_record_offset":4708,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7512,"parent_record_offset":7332,"ancestor_record_offset":4708,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7676,"parent_record_offset":7332,"ancestor_record_offset":4708,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7708,"parent_record_offset":7676,"ancestor_record_offset":4708,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7300,"parent_record_offset":7256,"ancestor_record_offset":4708,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7388,"parent_record_offset":7332,"ancestor_record_offset":4708,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7968,"parent_record_offset":7300,"ancestor_record_offset":4708,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":7876,"parent_record_offset":7676,"ancestor_record_offset":4708,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":4708,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":8824,"parent_record_offset":8704,"ancestor_record_offset":4708,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1324,"record_offset":8704,"parent_record_offset":8516,"ancestor_record_offset":4708,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":4708,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8856,"parent_record_offset":8516,"ancestor_record_offset":4708,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1324,"record_offset":5552,"parent_record_offset":5468,"ancestor_record_offset":4708,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1324,"record_offset":5496,"parent_record_offset":5468,"ancestor_record_offset":4708,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1324,"record_offset":5896,"parent_record_offset":5104,"ancestor_record_offset":4708,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1324,"record_offset":5468,"parent_record_offset":5440,"ancestor_record_offset":4708,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1324,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":4708,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1324,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":4708,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1324,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4708,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":1324,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":4708,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":1324,"record_offset":5348,"parent_record_offset":5320,"ancestor_record_offset":4708,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":1324,"record_offset":5408,"parent_record_offset":5292,"ancestor_record_offset":4708,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":1324,"record_offset":5292,"parent_record_offset":5104,"ancestor_record_offset":4708,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":1324,"record_offset":5440,"parent_record_offset":5104,"ancestor_record_offset":4708,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":1324,"record_offset":5152,"parent_record_offset":5104,"ancestor_record_offset":4708,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":1324,"record_offset":5236,"parent_record_offset":5104,"ancestor_record_offset":4708,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":1324,"record_offset":5812,"parent_record_offset":5104,"ancestor_record_offset":4708,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":7732,"parent_record_offset":7708,"ancestor_record_offset":4708,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1324,"record_offset":8192,"parent_record_offset":7256,"ancestor_record_offset":4708,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":7416,"parent_record_offset":7332,"ancestor_record_offset":4708,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":7764,"parent_record_offset":7676,"ancestor_record_offset":4708,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=10
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":8648,"parent_record_offset":8516,"ancestor_record_offset":4708,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":9112,"parent_record_offset":8516,"ancestor_record_offset":4708,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1324,"record_offset":7552,"parent_record_offset":7512,"ancestor_record_offset":4708,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1324,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":4708,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1324,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":4708,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1324,"record_offset":6260,"parent_record_offset":6236,"ancestor_record_offset":4708,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1324,"record_offset":9068,"parent_record_offset":9044,"ancestor_record_offset":4708,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$76::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1324,"record_offset":9400,"parent_record_offset":7020,"ancestor_record_offset":4708,"inlinee":7396,"source_line":75,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":7148,"parent_record_offset":7020,"ancestor_record_offset":4708,"inlinee":12248,"source_line":2663,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":7092,"parent_record_offset":7020,"ancestor_record_offset":4708,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":8376,"parent_record_offset":7020,"ancestor_record_offset":4708,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":8316,"parent_record_offset":7148,"ancestor_record_offset":4708,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":7208,"parent_record_offset":7148,"ancestor_record_offset":4708,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01324.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1324,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":4708,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/01324.jsonl.gz"}
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
// 完整结构及原始类型记录: bind_resources.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bind_resources.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bind_resources.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 信息 ID: cfca2a6a1c8a520d54f59c8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_color_library -> ["country_core::render::pass_builder::compute_pass_builder::ComputePass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1a73f3 -> 0x1a8740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> > @ 0x1a7c26 -> 0x1a8740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers> @ 0xdcc2d6 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank> @ 0xdccbf6 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> @ 0xdcd516 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > > @ 0xdcde15 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0xdce736 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0xdcf056 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0xdcf976 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile> @ 0xdd0296 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketMain> @ 0xdd0bb6 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0xdd14d6 -> 0xdcad60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 信息 ID: 5e008628c6aa53d1ce9f508f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_color_library -> ["country_core::render::pass_builder::draw_builder::DrawBuilder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster @ 0x5eccc5 -> 0xdf5000 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common @ 0x6ae725 -> 0xdf5000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 信息 ID: 8d7885c4b515be3a21b7988f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing -> ["country_core::render::pass_builder::compute_pass_builder::ComputePass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug @ 0x5787b4 -> 0xdcaf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0x5eac50 -> 0xdcaf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0x5eb591 -> 0xdcaf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x6096c0 -> 0xdcaf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60ab8a -> 0xdcaf20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60af74 -> 0xdcaf20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 信息 ID: bfffa14129ca7b65d21dfab2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing -> ["country_core::render::pass_builder::draw_builder::DrawBuilder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster @ 0x5eccdb -> 0xdf52e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw @ 0x63c13d -> 0xdf52e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap<country_core::render::pass_builder::compute_pass_builder::ComputePass>
// 信息 ID: d366ebea7729a6d192af67e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap -> ["country_core::render::pass_builder::compute_pass_builder::ComputePass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows @ 0x552bc3 -> 0xdcab40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture @ 0x55411d -> 0xdcab40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug @ 0x5787a6 -> 0xdcab40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0x5ea964 -> 0xdcab40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0x5eb154 -> 0xdcab40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance @ 0x63f8b1 -> 0xdcab40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance @ 0x640015 -> 0xdcab40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance @ 0x640478 -> 0xdcab40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics @ 0x662754 -> 0xdcab40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap<country_core::render::pass_builder::draw_builder::DrawBuilder>
// 信息 ID: 5776c3738bbf232ce5a8d73a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap -> ["country_core::render::pass_builder::draw_builder::DrawBuilder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster @ 0x5ecab7 -> 0xdf4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60a385 -> 0xdf4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common @ 0x6ae55f -> 0xdf4de0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
