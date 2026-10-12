// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/merge_proposal_wall_decorators.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: be3a5a260a93af7cc5c00f1f938934bd7ddd876ebc513a83dffc2786102b5a12
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: merge_proposal_wall_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2775, 2776, 3848
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2776,"record_offset":1224,"rva":25006544,"va":5393715664}
// 编译实例 RVAs: 0x17d91d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003390
//     owner=unresolved_namespace; count=2
//     target_aliases=1000 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24994486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e84a9d82fe","size":5,"soft_fail":false,"target_va":5368722320,"va":5393716801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24102345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e8373a90fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5393715860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20967487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e8c10fc0fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393717194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19161359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e8f19edbfe","size":5,"soft_fail":false,"target_va":5374554704,"va":5393716058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18506301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e8c39de5fe","size":5,"soft_fail":false,"target_va":5375209776,"va":5393716072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11460795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e8451f51ff","size":5,"soft_fail":false,"target_va":5382255760,"va":5393716550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11460906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e8d61e51ff","size":5,"soft_fail":false,"target_va":5382255840,"va":5393716741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11460387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e8dd2051ff","size":5,"soft_fail":false,"target_va":5382256288,"va":5393716670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11349090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e89ed352ff","size":5,"soft_fail":false,"target_va":5382367008,"va":5393716093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d8650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e855efffff","size":5,"soft_fail":false,"target_va":5393712720,"va":5393716982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9261684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e874528d00","size":5,"soft_fail":false,"target_va":5402978912,"va":5393717223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9261826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e802538d00","size":5,"soft_fail":false,"target_va":5402979152,"va":5393717321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9261642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e84a528d00","size":5,"soft_fail":false,"target_va":5402978944,"va":5393717297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19145459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e80ddddbfe","size":5,"soft_fail":false,"target_va":5374571104,"va":5393716558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19145417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e837dddbfe","size":5,"soft_fail":false,"target_va":5374570784,"va":5393716196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18503350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e84aa9e5fe","size":5,"soft_fail":false,"target_va":5375212688,"va":5393716033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18280001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393715664,"raw_hex":"e8bf11e9fe","size":5,"soft_fail":false,"target_va":5375436576,"va":5393716572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2776,"record_offset":3880,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2776,"record_offset":3852,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2776,"record_offset":3496,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$41::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2776,"record_offset":3072,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":5368,"source_line":2496,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2776,"record_offset":3908,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2776,"record_offset":2508,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2776,"record_offset":3556,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2776,"record_offset":3796,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2776,"record_offset":3640,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2776,"record_offset":1460,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::impl$1::snapping_direction
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2776,"record_offset":3100,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":89958,"source_line":277,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::try_merge_decorators
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2776,"record_offset":2644,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":89921,"source_line":289,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$0::classify_side
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2776,"record_offset":3612,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":89969,"source_line":43,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":2776,"record_offset":3732,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":21743,"source_line":197,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":2776,"record_offset":3348,"parent_record_offset":1224,"ancestor_record_offset":1224,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2776,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":1224,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2776,"record_offset":4076,"parent_record_offset":3968,"ancestor_record_offset":1224,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2776,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":1224,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2776,"record_offset":3968,"parent_record_offset":3936,"ancestor_record_offset":1224,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2776,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":1224,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2776,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":1224,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2776,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1224,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2776,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":1224,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2776,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":1224,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2776,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":1224,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2776,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":1224,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2776,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":1224,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":2776,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":1224,"inlinee":28256,"source_line":104,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2776,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":1224,"inlinee":13759,"source_line":49,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2776,"record_offset":3020,"parent_record_offset":2968,"ancestor_record_offset":1224,"inlinee":13764,"source_line":53,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2776,"record_offset":2676,"parent_record_offset":2644,"ancestor_record_offset":1224,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2776,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":1224,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2776,"record_offset":2872,"parent_record_offset":2644,"ancestor_record_offset":1224,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2776,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":1224,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2776,"record_offset":4040,"parent_record_offset":4004,"ancestor_record_offset":1224,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2776,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":1224,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2776,"record_offset":3672,"parent_record_offset":3640,"ancestor_record_offset":1224,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2776,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":1224,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2776,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":1224,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2776,"record_offset":3764,"parent_record_offset":3732,"ancestor_record_offset":1224,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2776,"record_offset":3936,"parent_record_offset":3908,"ancestor_record_offset":1224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2776,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":1224,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2776,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1224,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2776,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":1224,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2776,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":1224,"inlinee":25468,"source_line":17,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2776,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":1224,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2776,"record_offset":3256,"parent_record_offset":3164,"ancestor_record_offset":1224,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2776,"record_offset":2544,"parent_record_offset":2508,"ancestor_record_offset":1224,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2776,"record_offset":2604,"parent_record_offset":2572,"ancestor_record_offset":1224,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2776,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":1224,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2776,"record_offset":1528,"parent_record_offset":1492,"ancestor_record_offset":1224,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2776,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":1224,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2776,"record_offset":2240,"parent_record_offset":2020,"ancestor_record_offset":1224,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2776,"record_offset":1584,"parent_record_offset":1552,"ancestor_record_offset":1224,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":1224,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":2128,"parent_record_offset":2020,"ancestor_record_offset":1224,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":1224,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":2448,"parent_record_offset":1660,"ancestor_record_offset":1224,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":1224,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":1224,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":1612,"parent_record_offset":1552,"ancestor_record_offset":1224,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":1224,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2776,"record_offset":1900,"parent_record_offset":1872,"ancestor_record_offset":1224,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2776,"record_offset":1928,"parent_record_offset":1900,"ancestor_record_offset":1224,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2776,"record_offset":1988,"parent_record_offset":1872,"ancestor_record_offset":1224,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2776,"record_offset":1872,"parent_record_offset":1660,"ancestor_record_offset":1224,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":1704,"parent_record_offset":1660,"ancestor_record_offset":1224,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2776,"record_offset":2020,"parent_record_offset":1660,"ancestor_record_offset":1224,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2776,"record_offset":1732,"parent_record_offset":1660,"ancestor_record_offset":1224,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2776,"record_offset":1816,"parent_record_offset":1660,"ancestor_record_offset":1224,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2776,"record_offset":2356,"parent_record_offset":1660,"ancestor_record_offset":1224,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2776,"record_offset":1492,"parent_record_offset":1460,"ancestor_record_offset":1224,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2776,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":1224,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":2776,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":1224,"inlinee":89968,"source_line":108,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2776,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":1224,"inlinee":89922,"source_line":296,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2776,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":1224,"inlinee":89889,"source_line":301,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2776,"record_offset":3380,"parent_record_offset":3348,"ancestor_record_offset":1224,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2776,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":1224,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2776,"record_offset":3308,"parent_record_offset":3256,"ancestor_record_offset":1224,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2776,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":1224,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2776,"record_offset":256,"rva":25005216,"va":5393714336}
// 编译实例 RVAs: 0x17d8ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9263217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714336,"raw_hex":"e871588d00","size":5,"soft_fail":false,"target_va":5402978880,"va":5393715658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393714336,"raw_hex":"e849040000","size":5,"soft_fail":false,"target_va":5393715664,"va":5393714562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2776,"record_offset":432,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8709,"source_line":3310,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2776,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":256,"inlinee":6144,"source_line":1275,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2776,"record_offset":928,"parent_record_offset":844,"ancestor_record_offset":256,"inlinee":25841,"source_line":39,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2776,"record_offset":812,"parent_record_offset":464,"ancestor_record_offset":256,"inlinee":25235,"source_line":144,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2776,"record_offset":500,"parent_record_offset":464,"ancestor_record_offset":256,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2776,"record_offset":996,"parent_record_offset":928,"ancestor_record_offset":256,"inlinee":6142,"source_line":3317,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2776,"record_offset":464,"parent_record_offset":432,"ancestor_record_offset":256,"inlinee":8710,"source_line":2618,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2776,"record_offset":1172,"parent_record_offset":844,"ancestor_record_offset":256,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2776,"record_offset":720,"parent_record_offset":532,"ancestor_record_offset":256,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2776,"record_offset":1060,"parent_record_offset":1024,"ancestor_record_offset":256,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2776,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$41::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2776,"record_offset":784,"parent_record_offset":464,"ancestor_record_offset":256,"inlinee":5368,"source_line":2496,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2776,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2776,"record_offset":628,"parent_record_offset":572,"ancestor_record_offset":256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2776,"record_offset":660,"parent_record_offset":572,"ancestor_record_offset":256,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2776,"record_offset":532,"parent_record_offset":500,"ancestor_record_offset":256,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2776,"record_offset":844,"parent_record_offset":812,"ancestor_record_offset":256,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2776,"record_offset":572,"parent_record_offset":532,"ancestor_record_offset":256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":2776,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":256,"inlinee":89966,"source_line":49,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":2776,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":256,"inlinee":89967,"source_line":63,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F6E
// 首条位置: {"module":2776,"inlinee":89966}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F6F
// 首条位置: {"module":2776,"inlinee":89967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2776,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02776.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F70
// 首条位置: {"module":2776,"inlinee":89968}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F64
// 首条位置: {"module":2775,"inlinee":89956}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2775,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2775,"record_offset":988,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2775,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":852,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2775,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":852,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2775,"record_offset":1124,"parent_record_offset":1036,"ancestor_record_offset":852,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2775,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":852,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2775,"record_offset":1064,"parent_record_offset":1036,"ancestor_record_offset":852,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2775,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":852,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02775.jsonl.gz"}
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
// 完整结构及原始类型记录: merge_proposal_wall_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: merge_proposal_wall_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: merge_proposal_wall_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position
// 信息 ID: 010c86fec2e5a79ad596fbb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a4db -> 0x17d8ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
// 信息 ID: 277abba30e443837f36b9943
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=2776, inline_record=744; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=2776, inline_record=968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$1
// 信息 ID: ba709860a9df994bc4d2e153
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=2776, inline_record=1088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner
// 信息 ID: a5dc43362b5794e5d646a7f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position @ 0x17d8d82 -> 0x17d91d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position @ 0x17d8fa4 -> 0x17d91d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
// 信息 ID: b050778b0586572671549fa8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=2776, inline_record=3524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$1
// 信息 ID: 5bb8354b8695cce679651ae9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=2775, inline_record=852; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=2775, inline_record=2420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
