// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/pass_builder/uniform_binder.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d84d9eceea6b51551f3d7b341e3b35adc2cf0d7a56c5466b375eebc343014d66
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: uniform_binder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 41, 67, 115, 120, 219, 228, 389, 398, 424, 464, 490, 1318, 1319, 1324
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::UniformBinder::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=14
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x563E
// 首条位置: {"module":389,"record_offset":8432,"rva":5301392,"va":5374010512}
// 编译实例 RVAs: 0x50e490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2251812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374010512,"raw_hex":"e8245c2200","size":5,"soft_fail":false,"target_va":5376262400,"va":5374010583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2251044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374010512,"raw_hex":"e824592200","size":5,"soft_fail":false,"target_va":5376261648,"va":5374010599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":389,"record_offset":8588,"parent_record_offset":8432,"ancestor_record_offset":8432,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FEA
// 首条位置: {"module":115,"inlinee":8170}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=18
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":115,"record_offset":10936,"parent_record_offset":10908,"ancestor_record_offset":10908,"inlinee":7402,"source_line":53,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":115,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":10908,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":115,"record_offset":11272,"parent_record_offset":11248,"ancestor_record_offset":10908,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":115,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":10908,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":115,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":10908,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":115,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10908,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":115,"record_offset":11000,"parent_record_offset":10936,"ancestor_record_offset":10908,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":115,"record_offset":10972,"parent_record_offset":10936,"ancestor_record_offset":10908,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":115,"record_offset":12148,"parent_record_offset":12124,"ancestor_record_offset":10908,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":115,"record_offset":11632,"parent_record_offset":11608,"ancestor_record_offset":10908,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":115,"record_offset":11608,"parent_record_offset":11584,"ancestor_record_offset":10908,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":115,"record_offset":12056,"parent_record_offset":10936,"ancestor_record_offset":10908,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":115,"record_offset":12084,"parent_record_offset":12056,"ancestor_record_offset":10908,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=18
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":115,"record_offset":11028,"parent_record_offset":11000,"ancestor_record_offset":10908,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=18
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":115,"record_offset":12124,"parent_record_offset":10936,"ancestor_record_offset":10908,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$24::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1318,"record_offset":21120,"parent_record_offset":19964,"ancestor_record_offset":19936,"inlinee":61721,"source_line":55,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1318,"record_offset":20692,"parent_record_offset":20668,"ancestor_record_offset":19936,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":115,"record_offset":11696,"parent_record_offset":11672,"ancestor_record_offset":10908,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":19936,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":115,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":10908,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":19936,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":115,"record_offset":11100,"parent_record_offset":11076,"ancestor_record_offset":10908,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20668,"parent_record_offset":20532,"ancestor_record_offset":19936,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":115,"record_offset":11672,"parent_record_offset":11536,"ancestor_record_offset":10908,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11584,"parent_record_offset":11560,"ancestor_record_offset":10908,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11560,"parent_record_offset":11536,"ancestor_record_offset":10908,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":10908,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11880,"parent_record_offset":11176,"ancestor_record_offset":10908,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11916,"parent_record_offset":11148,"ancestor_record_offset":10908,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11148,"parent_record_offset":11124,"ancestor_record_offset":10908,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11124,"parent_record_offset":11100,"ancestor_record_offset":10908,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11176,"parent_record_offset":11148,"ancestor_record_offset":10908,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":115,"record_offset":11416,"parent_record_offset":11388,"ancestor_record_offset":10908,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":115,"record_offset":11444,"parent_record_offset":11416,"ancestor_record_offset":10908,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":115,"record_offset":11504,"parent_record_offset":11388,"ancestor_record_offset":10908,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":115,"record_offset":11388,"parent_record_offset":11176,"ancestor_record_offset":10908,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11220,"parent_record_offset":11176,"ancestor_record_offset":10908,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":115,"record_offset":11536,"parent_record_offset":11176,"ancestor_record_offset":10908,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":115,"record_offset":11248,"parent_record_offset":11176,"ancestor_record_offset":10908,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=36
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":115,"record_offset":11332,"parent_record_offset":11176,"ancestor_record_offset":10908,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":115,"record_offset":11788,"parent_record_offset":11176,"ancestor_record_offset":10908,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=18
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":115,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":10908,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":115,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":10908,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1319,"record_offset":252,"rva":14529904,"va":5383239024}
// 编译实例 RVAs: 0xddb570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19741011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383239024,"raw_hex":"e853392d01","size":5,"soft_fail":false,"target_va":5402980288,"va":5383239272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1319,"record_offset":552,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7402,"source_line":53,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1319,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":252,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1319,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":252,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1319,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":252,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1319,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":252,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1319,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":252,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1319,"record_offset":612,"parent_record_offset":552,"ancestor_record_offset":252,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1319,"record_offset":588,"parent_record_offset":552,"ancestor_record_offset":252,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":252,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":252,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":252,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1319,"record_offset":1668,"parent_record_offset":552,"ancestor_record_offset":252,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1319,"record_offset":1696,"parent_record_offset":1668,"ancestor_record_offset":252,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1319,"record_offset":640,"parent_record_offset":612,"ancestor_record_offset":252,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1319,"record_offset":1736,"parent_record_offset":552,"ancestor_record_offset":252,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1319,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":252,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":252,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":252,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":1284,"parent_record_offset":1148,"ancestor_record_offset":252,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":252,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":252,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":252,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":1492,"parent_record_offset":788,"ancestor_record_offset":252,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":1528,"parent_record_offset":760,"ancestor_record_offset":252,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":252,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":252,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":252,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":1028,"parent_record_offset":1000,"ancestor_record_offset":252,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":1056,"parent_record_offset":1028,"ancestor_record_offset":252,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":1116,"parent_record_offset":1000,"ancestor_record_offset":252,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":1000,"parent_record_offset":788,"ancestor_record_offset":252,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":832,"parent_record_offset":788,"ancestor_record_offset":252,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":1148,"parent_record_offset":788,"ancestor_record_offset":252,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":860,"parent_record_offset":788,"ancestor_record_offset":252,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":944,"parent_record_offset":788,"ancestor_record_offset":252,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":1400,"parent_record_offset":788,"ancestor_record_offset":252,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1319,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":252,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1319,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1319,"record_offset":1828,"rva":14530160,"va":5383239280}
// 编译实例 RVAs: 0xddb670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19740778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383239280,"raw_hex":"e86a382d01","size":5,"soft_fail":false,"target_va":5402980288,"va":5383239505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1319,"record_offset":2092,"parent_record_offset":1828,"ancestor_record_offset":1828,"inlinee":7402,"source_line":53,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1319,"record_offset":2864,"parent_record_offset":2840,"ancestor_record_offset":1828,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1319,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":1828,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1319,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":1828,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1319,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":1828,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1319,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":1828,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1319,"record_offset":2152,"parent_record_offset":2092,"ancestor_record_offset":1828,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1319,"record_offset":2128,"parent_record_offset":2092,"ancestor_record_offset":1828,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":3292,"parent_record_offset":3268,"ancestor_record_offset":1828,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1828,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":1828,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1319,"record_offset":3200,"parent_record_offset":2092,"ancestor_record_offset":1828,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1319,"record_offset":3228,"parent_record_offset":3200,"ancestor_record_offset":1828,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1319,"record_offset":2180,"parent_record_offset":2152,"ancestor_record_offset":1828,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1319,"record_offset":3268,"parent_record_offset":2092,"ancestor_record_offset":1828,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1319,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":1828,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":1828,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":1828,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":2816,"parent_record_offset":2680,"ancestor_record_offset":1828,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":1828,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":1828,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":1828,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":3024,"parent_record_offset":2328,"ancestor_record_offset":1828,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":3060,"parent_record_offset":2300,"ancestor_record_offset":1828,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1828,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1828,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":2328,"parent_record_offset":2300,"ancestor_record_offset":1828,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":1828,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":1828,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":2648,"parent_record_offset":2540,"ancestor_record_offset":1828,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":2540,"parent_record_offset":2328,"ancestor_record_offset":1828,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":2372,"parent_record_offset":2328,"ancestor_record_offset":1828,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":2680,"parent_record_offset":2328,"ancestor_record_offset":1828,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":2400,"parent_record_offset":2328,"ancestor_record_offset":1828,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":2484,"parent_record_offset":2328,"ancestor_record_offset":1828,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":2932,"parent_record_offset":2328,"ancestor_record_offset":1828,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1319,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":1828,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1319,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":1828,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
// 记录形式: inline_instance, inlinee_declaration; instances=113
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CEA
// 首条位置: {"module":41,"inlinee":7402}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":41,"record_offset":1856,"parent_record_offset":1788,"ancestor_record_offset":1788,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":41,"record_offset":1828,"parent_record_offset":1788,"ancestor_record_offset":1788,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":41,"record_offset":3020,"parent_record_offset":1788,"ancestor_record_offset":1788,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure$3
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":41,"record_offset":2960,"parent_record_offset":1788,"ancestor_record_offset":1788,"inlinee":7422,"source_line":858,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure$5
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":228,"record_offset":18980,"parent_record_offset":17812,"ancestor_record_offset":17812,"inlinee":8165,"source_line":687,"archive":"_pdb/A/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure$7
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":228,"record_offset":10700,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":8163,"source_line":713,"archive":"_pdb/A/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=48
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":41,"record_offset":3088,"parent_record_offset":1788,"ancestor_record_offset":1788,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::uniform::impl$24::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1318,"record_offset":21120,"parent_record_offset":19964,"ancestor_record_offset":19964,"inlinee":61721,"source_line":55,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$28::as_mut
//     owner=private_source_ownership_candidate; count=34
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":41,"record_offset":2988,"parent_record_offset":1788,"ancestor_record_offset":1788,"inlinee":7417,"source_line":600,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":41,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":1788,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":41,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":1788,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=108
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":41,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":1788,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=106
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":41,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":1788,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":41,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1788,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=52
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":41,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":1788,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=108
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":41,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1788,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=108
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":41,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":1788,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":41,"record_offset":3048,"parent_record_offset":3020,"ancestor_record_offset":1788,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::pass_resource_access::impl$2::dims
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":464,"record_offset":21180,"parent_record_offset":21156,"ancestor_record_offset":19984,"inlinee":25226,"source_line":74,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::pass_resource_access::impl$3::dims
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":464,"record_offset":11876,"parent_record_offset":11852,"ancestor_record_offset":10680,"inlinee":26467,"source_line":89,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::pass_resource_access::impl$4::dims
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":424,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":18356,"inlinee":23132,"source_line":104,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::pass_resource_access::impl$5::dims
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":464,"record_offset":16680,"parent_record_offset":16656,"ancestor_record_offset":15484,"inlinee":26470,"source_line":119,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=54
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":41,"record_offset":1892,"parent_record_offset":1856,"ancestor_record_offset":1788,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1318,"record_offset":21144,"parent_record_offset":21120,"ancestor_record_offset":19964,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1318,"record_offset":20692,"parent_record_offset":20668,"ancestor_record_offset":19964,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=35
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":41,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":1788,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":19964,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=35
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":41,"record_offset":1952,"parent_record_offset":1920,"ancestor_record_offset":1788,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":19964,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=35
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":41,"record_offset":1984,"parent_record_offset":1952,"ancestor_record_offset":1788,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20668,"parent_record_offset":20532,"ancestor_record_offset":19964,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=35
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":41,"record_offset":2576,"parent_record_offset":2440,"ancestor_record_offset":1788,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":1788,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":1788,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":1788,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2784,"parent_record_offset":2080,"ancestor_record_offset":1788,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2820,"parent_record_offset":2048,"ancestor_record_offset":1788,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2048,"parent_record_offset":2016,"ancestor_record_offset":1788,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2016,"parent_record_offset":1984,"ancestor_record_offset":1788,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2080,"parent_record_offset":2048,"ancestor_record_offset":1788,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":41,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":1788,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":41,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":1788,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":41,"record_offset":2408,"parent_record_offset":2292,"ancestor_record_offset":1788,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":41,"record_offset":2292,"parent_record_offset":2080,"ancestor_record_offset":1788,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=48
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2124,"parent_record_offset":2080,"ancestor_record_offset":1788,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2440,"parent_record_offset":2080,"ancestor_record_offset":1788,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":41,"record_offset":2152,"parent_record_offset":2080,"ancestor_record_offset":1788,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=108
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":41,"record_offset":2236,"parent_record_offset":2080,"ancestor_record_offset":1788,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=54
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":41,"record_offset":2692,"parent_record_offset":2080,"ancestor_record_offset":1788,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":424,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":18356,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=54
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":41,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":1788,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":41,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":1788,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TextureFromDisk>,array$<u32,2> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1319,"record_offset":3360,"rva":14530400,"va":5383239520}
// 编译实例 RVAs: 0xddb760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19740485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383239520,"raw_hex":"e845372d01","size":5,"soft_fail":false,"target_va":5402980288,"va":5383239798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8885008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383239520,"raw_hex":"e8f06c78ff","size":5,"soft_fail":false,"target_va":5374354736,"va":5383239739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6897993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383239520,"raw_hex":"e8b7be96ff","size":5,"soft_fail":false,"target_va":5376341760,"va":5383239748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1319,"record_offset":3764,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1319,"record_offset":3740,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1319,"record_offset":4908,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1319,"record_offset":4820,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":7422,"source_line":858,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1319,"record_offset":4976,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$28::as_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1319,"record_offset":4876,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":7417,"source_line":600,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1319,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":3360,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1319,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":3360,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1319,"record_offset":4120,"parent_record_offset":4096,"ancestor_record_offset":3360,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1319,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3360,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1319,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":3360,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":3360,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":3360,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":3360,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1319,"record_offset":4936,"parent_record_offset":4908,"ancestor_record_offset":3360,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::pass_resource_access::impl$1::dims
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":1319,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":3360,"inlinee":26901,"source_line":57,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1319,"record_offset":3792,"parent_record_offset":3764,"ancestor_record_offset":3360,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1319,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":3360,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":3360,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":3864,"parent_record_offset":3840,"ancestor_record_offset":3360,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":4436,"parent_record_offset":4300,"ancestor_record_offset":3360,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":3360,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":3360,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":3360,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":4644,"parent_record_offset":3940,"ancestor_record_offset":3360,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":4680,"parent_record_offset":3912,"ancestor_record_offset":3360,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":3360,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":3888,"parent_record_offset":3864,"ancestor_record_offset":3360,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":3940,"parent_record_offset":3912,"ancestor_record_offset":3360,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":4180,"parent_record_offset":4152,"ancestor_record_offset":3360,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":4208,"parent_record_offset":4180,"ancestor_record_offset":3360,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":4268,"parent_record_offset":4152,"ancestor_record_offset":3360,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":4152,"parent_record_offset":3940,"ancestor_record_offset":3360,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":3984,"parent_record_offset":3940,"ancestor_record_offset":3360,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":4300,"parent_record_offset":3940,"ancestor_record_offset":3360,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":4012,"parent_record_offset":3940,"ancestor_record_offset":3360,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":4096,"parent_record_offset":3940,"ancestor_record_offset":3360,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":4552,"parent_record_offset":3940,"ancestor_record_offset":3360,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1319,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":3360,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1319,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":3360,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TexturePreset>,array$<u32,2> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1319,"record_offset":6716,"rva":14530976,"va":5383240096}
// 编译实例 RVAs: 0xddb9a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19739902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383240096,"raw_hex":"e8fe342d01","size":5,"soft_fail":false,"target_va":5402980288,"va":5383240381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8884390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383240096,"raw_hex":"e85a6f78ff","size":5,"soft_fail":false,"target_va":5374355936,"va":5383240321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6898576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383240096,"raw_hex":"e870bc96ff","size":5,"soft_fail":false,"target_va":5376341760,"va":5383240331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1319,"record_offset":7116,"parent_record_offset":6716,"ancestor_record_offset":6716,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1319,"record_offset":7092,"parent_record_offset":6716,"ancestor_record_offset":6716,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1319,"record_offset":8292,"parent_record_offset":6716,"ancestor_record_offset":6716,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1319,"record_offset":8172,"parent_record_offset":6716,"ancestor_record_offset":6716,"inlinee":7422,"source_line":858,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":1319,"record_offset":8360,"parent_record_offset":6716,"ancestor_record_offset":6716,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$28::as_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1319,"record_offset":8260,"parent_record_offset":6716,"ancestor_record_offset":6716,"inlinee":7417,"source_line":600,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1319,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":6716,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1319,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":6716,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1319,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":6716,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1319,"record_offset":7412,"parent_record_offset":7388,"ancestor_record_offset":6716,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1319,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":6716,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":8384,"parent_record_offset":8360,"ancestor_record_offset":6716,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":7748,"parent_record_offset":7724,"ancestor_record_offset":6716,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1319,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":6716,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1319,"record_offset":8320,"parent_record_offset":8292,"ancestor_record_offset":6716,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::pass_resource_access::impl$0::dims
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":1319,"record_offset":8196,"parent_record_offset":8172,"ancestor_record_offset":6716,"inlinee":26900,"source_line":42,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":1319,"record_offset":7144,"parent_record_offset":7116,"ancestor_record_offset":6716,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1319,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":6716,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":6716,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":6716,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1319,"record_offset":7788,"parent_record_offset":7652,"ancestor_record_offset":6716,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":6716,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":6716,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":6716,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":7996,"parent_record_offset":7292,"ancestor_record_offset":6716,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":8032,"parent_record_offset":7264,"ancestor_record_offset":6716,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":6716,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":6716,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":7292,"parent_record_offset":7264,"ancestor_record_offset":6716,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":6716,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":6716,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":7620,"parent_record_offset":7504,"ancestor_record_offset":6716,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1319,"record_offset":7504,"parent_record_offset":7292,"ancestor_record_offset":6716,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":7336,"parent_record_offset":7292,"ancestor_record_offset":6716,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1319,"record_offset":7652,"parent_record_offset":7292,"ancestor_record_offset":6716,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":7364,"parent_record_offset":7292,"ancestor_record_offset":6716,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":7448,"parent_record_offset":7292,"ancestor_record_offset":6716,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1319,"record_offset":7904,"parent_record_offset":7292,"ancestor_record_offset":6716,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1319,"record_offset":8224,"parent_record_offset":8196,"ancestor_record_offset":6716,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1319,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":6716,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1319,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":6716,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::raw::texture::Texture> >,array$<u32,2> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":67,"record_offset":232,"rva":877072,"va":5369586192}
// 编译实例 RVAs: 0xd6210, 0xddb880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33393814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369586192,"raw_hex":"e8968cfd01","size":5,"soft_fail":false,"target_va":5402980288,"va":5369586469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4768321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369586192,"raw_hex":"e841c24800","size":5,"soft_fail":false,"target_va":5374354736,"va":5369586410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6755336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369586192,"raw_hex":"e808146700","size":5,"soft_fail":false,"target_va":5376341760,"va":5369586419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":67,"record_offset":608,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":67,"record_offset":584,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":67,"record_offset":1728,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure$4
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":67,"record_offset":1664,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8164,"source_line":665,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::uniform::impl$23::write_uniform_to_buffer
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/uniform.rs
//     evidence={"module":67,"record_offset":1796,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":7419,"source_line":43,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$28::as_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":67,"record_offset":1696,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":7417,"source_line":600,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":67,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":232,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":67,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":232,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":67,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":232,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":67,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":67,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":232,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":67,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":232,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":67,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":232,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":67,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":232,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":67,"record_offset":1756,"parent_record_offset":1728,"ancestor_record_offset":232,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/uniform_binder.rs
//     evidence={"module":67,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":232,"inlinee":7404,"source_line":65,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":67,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":232,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":67,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":232,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":67,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":232,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":67,"record_offset":1280,"parent_record_offset":1144,"ancestor_record_offset":232,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":232,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":1168,"parent_record_offset":1144,"ancestor_record_offset":232,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":232,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":1488,"parent_record_offset":784,"ancestor_record_offset":232,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":1524,"parent_record_offset":756,"ancestor_record_offset":232,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":232,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":232,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":784,"parent_record_offset":756,"ancestor_record_offset":232,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":67,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":232,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":67,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":232,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":67,"record_offset":1112,"parent_record_offset":996,"ancestor_record_offset":232,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":67,"record_offset":996,"parent_record_offset":784,"ancestor_record_offset":232,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":828,"parent_record_offset":784,"ancestor_record_offset":232,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":67,"record_offset":1144,"parent_record_offset":784,"ancestor_record_offset":232,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":67,"record_offset":856,"parent_record_offset":784,"ancestor_record_offset":232,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":67,"record_offset":940,"parent_record_offset":784,"ancestor_record_offset":232,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":67,"record_offset":1396,"parent_record_offset":784,"ancestor_record_offset":232,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":67,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":232,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":67,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":232,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00067.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=135
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CEC
// 首条位置: {"module":41,"inlinee":7404}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":41,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":1892,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":41,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":1892,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":41,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":1892,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=116
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":41,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":1892,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":41,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":1892,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":41,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1892,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=116
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":41,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1892,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=116
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":41,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":1892,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":1318,"record_offset":20692,"parent_record_offset":20668,"ancestor_record_offset":20056,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=39
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":41,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":1892,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":20056,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=39
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":41,"record_offset":1952,"parent_record_offset":1920,"ancestor_record_offset":1892,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":20056,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=39
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":41,"record_offset":1984,"parent_record_offset":1952,"ancestor_record_offset":1892,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":1318,"record_offset":20668,"parent_record_offset":20532,"ancestor_record_offset":20056,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=39
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":41,"record_offset":2576,"parent_record_offset":2440,"ancestor_record_offset":1892,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":1892,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":1892,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":1892,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2784,"parent_record_offset":2080,"ancestor_record_offset":1892,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2820,"parent_record_offset":2048,"ancestor_record_offset":1892,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2048,"parent_record_offset":2016,"ancestor_record_offset":1892,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2016,"parent_record_offset":1984,"ancestor_record_offset":1892,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2080,"parent_record_offset":2048,"ancestor_record_offset":1892,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":41,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":1892,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":41,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":1892,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":41,"record_offset":2408,"parent_record_offset":2292,"ancestor_record_offset":1892,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":41,"record_offset":2292,"parent_record_offset":2080,"ancestor_record_offset":1892,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=52
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2124,"parent_record_offset":2080,"ancestor_record_offset":1892,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":41,"record_offset":2440,"parent_record_offset":2080,"ancestor_record_offset":1892,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":41,"record_offset":2152,"parent_record_offset":2080,"ancestor_record_offset":1892,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=116
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":41,"record_offset":2236,"parent_record_offset":2080,"ancestor_record_offset":1892,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=58
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":41,"record_offset":2692,"parent_record_offset":2080,"ancestor_record_offset":1892,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00041.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$16::eq
//     owner=private_source_ownership_candidate; count=58
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":41,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":1892,"inlinee":7394,"source_line":15,"archive":"_pdb/A/modules/00041.jsonl.gz"}
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
// 完整结构及原始类型记录: uniform_binder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::uniform_binder::UniformBinder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::raw::texture::Texture> >,array$<u32,2> >", "constructor": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform", "arguments": ["country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::raw::texture::Texture> >", "array$<u32,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>", "constructor": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform", "arguments": ["country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>", "glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>", "constructor": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform", "arguments": ["country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>", "i32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TextureFromDisk>,array$<u32,2> >", "constructor": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform", "arguments": ["country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TextureFromDisk>", "array$<u32,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TexturePreset>,array$<u32,2> >", "constructor": "country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform", "arguments": ["country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TexturePreset>", "array$<u32,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: uniform_binder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: uniform_binder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::UniformBinder::new
// 信息 ID: 3af99afa6718b7047f0c5d0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1a7183 -> 0x50e490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> > @ 0x1a79b6 -> 0x50e490 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::DrawBuilder::new; module=490, inline_record=30664; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers>; module=1318, inline_record=35312; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank>; module=1318, inline_record=38512; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush>; module=1318, inline_record=41704; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > >; module=1318, inline_record=44880; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >; module=1318, inline_record=47984; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof>; module=1318, inline_record=51148; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes>; module=1318, inline_record=54300; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform
// 信息 ID: c37aa5e0d25edea7395597fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 54 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=1788; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=4056; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<array$<u32,2> >; module=115, inline_record=10936; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<u32>; module=115, inline_record=12444; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=1784; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=4052; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=7176; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=9444; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TextureFromDisk>,array$<u32,2> >
// 信息 ID: 04d5dd2fe672d83a62c16eb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3 -> ["country_core::resources::texture_presets::TextureFromDisk"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform -> ["country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TextureFromDisk>", "array$<u32,2>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk> @ 0x54040d -> 0xddb760 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk> @ 0x540cdd -> 0xddb760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TexturePreset>,array$<u32,2> >
// 信息 ID: 02aec38daf163dd2ad1ea754
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3 -> ["country_core::resources::texture_presets::TexturePreset"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform -> ["country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass::closure_env$3<country_core::resources::texture_presets::TexturePreset>", "array$<u32,2>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x540800 -> 0xddb9a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset> @ 0x541412 -> 0xddb9a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::raw::texture::Texture> >,array$<u32,2> >
// 信息 ID: 15cb8a8c84885f14e952a0a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["rhapsody::raw::texture::Texture"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4 -> ["ref$<rhapsody::raw::texture::Texture>"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform -> ["country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::raw::texture::Texture> >", "array$<u32,2>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x358280 -> 0xd6210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk> @ 0x540c9a -> 0xddb880 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x644d9b -> 0xddb880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform
// 信息 ID: 539b3e6f19565f1bfa5ef965
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<array$<u32,2> >; module=115, inline_record=10908; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<u32>; module=115, inline_record=12416; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap<country_core::render::pass_builder::compute_pass_builder::ComputePass>; module=1318, inline_record=19936; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>; module=1318, inline_record=30160; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>; module=1318, inline_record=31604; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<i32>; module=1318, inline_record=139984; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<array$<i32,2> >; module=1318, inline_record=141504; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<f32>; module=1318, inline_record=143012; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
// 信息 ID: 6831be1eccacf4e74fc6c496
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0 -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform -> ["country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>", "glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 19 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute @ 0x5090dd -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute @ 0x509229 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus @ 0x50fa89 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery @ 0x555d94 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery @ 0x555df2 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cd342 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cd663 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cd6c1 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cda3a -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cda87 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cdecb -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cdf18 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x687312 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x68736a -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x6873b6 -> 0xddb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x6876c3 -> 0xddb570 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
// 信息 ID: 370183d515bc8f5e086bc60d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0 -> ["i32"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform -> ["country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>", "i32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings @ 0x53686e -> 0xddb670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings @ 0x536cf0 -> 0xddb670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu @ 0x5d466d -> 0xddb670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation @ 0x606ecf -> 0xddb670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation @ 0x6071fd -> 0xddb670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation @ 0x6cfe64 -> 0xddb670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation @ 0x6d022e -> 0xddb670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation @ 0x6d05b1 -> 0xddb670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation @ 0x6d097c -> 0xddb670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform::closure$0
// 信息 ID: ab28d054ed6efc2e94975a31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::uniform_binder::impl$0::write_optional_uniform", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 58 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=1892; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=4148; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::raw::texture::Texture> >,array$<u32,2> >; module=67, inline_record=636; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<array$<u32,2> >; module=115, inline_record=11028; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<u32>; module=115, inline_record=12536; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=1888; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=4144; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=7280; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
