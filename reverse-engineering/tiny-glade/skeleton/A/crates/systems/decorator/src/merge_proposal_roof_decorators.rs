// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/merge_proposal_roof_decorators.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d24ba998d2d570c152a810611ee96a324267c6c46235efa063e22e1ac8fa5edb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: merge_proposal_roof_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2772, 2777
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
// 函数符号: system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2777,"record_offset":5668,"rva":25010416,"va":5393719536}
// 编译实例 RVAs: 0x17da0f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d9dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393719536,"raw_hex":"e8effaffff","size":5,"soft_fail":false,"target_va":5393718736,"va":5393720028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9199567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393719536,"raw_hex":"e8cf5f8c00","size":5,"soft_fail":false,"target_va":5402919776,"va":5393720204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15767590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393719536,"raw_hex":"e8da670fff","size":5,"soft_fail":false,"target_va":5377952256,"va":5393719841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2777,"record_offset":7252,"parent_record_offset":5668,"ancestor_record_offset":5668,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2777,"record_offset":6016,"parent_record_offset":5668,"ancestor_record_offset":5668,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":5960,"parent_record_offset":5668,"ancestor_record_offset":5668,"inlinee":60173,"source_line":134,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":6628,"parent_record_offset":5668,"ancestor_record_offset":5668,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2777,"record_offset":6848,"parent_record_offset":5668,"ancestor_record_offset":5668,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2777,"record_offset":8256,"parent_record_offset":8224,"ancestor_record_offset":5668,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2777,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":5668,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2777,"record_offset":8224,"parent_record_offset":7328,"ancestor_record_offset":5668,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2777,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":5668,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2777,"record_offset":8196,"parent_record_offset":7328,"ancestor_record_offset":5668,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2777,"record_offset":8316,"parent_record_offset":7328,"ancestor_record_offset":5668,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2777,"record_offset":8288,"parent_record_offset":7328,"ancestor_record_offset":5668,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2777,"record_offset":7304,"parent_record_offset":7252,"ancestor_record_offset":5668,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2777,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":5668,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2777,"record_offset":7168,"parent_record_offset":7136,"ancestor_record_offset":5668,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":6452,"parent_record_offset":6040,"ancestor_record_offset":5668,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":6360,"parent_record_offset":6304,"ancestor_record_offset":5668,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":5668,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":6164,"parent_record_offset":6084,"ancestor_record_offset":5668,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":6224,"parent_record_offset":6040,"ancestor_record_offset":5668,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2777,"record_offset":7372,"parent_record_offset":7328,"ancestor_record_offset":5668,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":2777,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":5668,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2777,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":5668,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2777,"record_offset":8156,"parent_record_offset":7420,"ancestor_record_offset":5668,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2777,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":5668,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2777,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":5668,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2777,"record_offset":6784,"parent_record_offset":6652,"ancestor_record_offset":5668,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2777,"record_offset":7052,"parent_record_offset":7020,"ancestor_record_offset":5668,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2777,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":5668,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2777,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":5668,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2777,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":5668,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2777,"record_offset":6480,"parent_record_offset":6040,"ancestor_record_offset":5668,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":6716,"parent_record_offset":6684,"ancestor_record_offset":5668,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":6684,"parent_record_offset":6652,"ancestor_record_offset":5668,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":6652,"parent_record_offset":6628,"ancestor_record_offset":5668,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2777,"record_offset":5984,"parent_record_offset":5960,"ancestor_record_offset":5668,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2777,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":5668,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2777,"record_offset":6084,"parent_record_offset":6040,"ancestor_record_offset":5668,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2777,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":5668,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2777,"record_offset":7484,"parent_record_offset":7452,"ancestor_record_offset":5668,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2777,"record_offset":7812,"parent_record_offset":7564,"ancestor_record_offset":5668,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2777,"record_offset":7020,"parent_record_offset":6928,"ancestor_record_offset":5668,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2777,"record_offset":7564,"parent_record_offset":7516,"ancestor_record_offset":5668,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2777,"record_offset":6900,"parent_record_offset":6872,"ancestor_record_offset":5668,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2777,"record_offset":6992,"parent_record_offset":6928,"ancestor_record_offset":5668,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2777,"record_offset":6964,"parent_record_offset":6928,"ancestor_record_offset":5668,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2777,"record_offset":6928,"parent_record_offset":6900,"ancestor_record_offset":5668,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2777,"record_offset":7712,"parent_record_offset":7628,"ancestor_record_offset":5668,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2777,"record_offset":7748,"parent_record_offset":7712,"ancestor_record_offset":5668,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2777,"record_offset":7676,"parent_record_offset":7628,"ancestor_record_offset":5668,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2777,"record_offset":7628,"parent_record_offset":7564,"ancestor_record_offset":5668,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2777,"record_offset":7516,"parent_record_offset":7484,"ancestor_record_offset":5668,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2777,"record_offset":7080,"parent_record_offset":7020,"ancestor_record_offset":5668,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2777,"record_offset":7136,"parent_record_offset":7080,"ancestor_record_offset":5668,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2777,"record_offset":7452,"parent_record_offset":7420,"ancestor_record_offset":5668,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2777,"record_offset":6748,"parent_record_offset":6716,"ancestor_record_offset":5668,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F78
// 首条位置: {"module":2777,"inlinee":89976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2777,"record_offset":3552,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2777,"record_offset":3604,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2777,"record_offset":3908,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":5440,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2777,"record_offset":4700,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2777,"record_offset":4788,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2777,"record_offset":4644,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2777,"record_offset":3852,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":2777,"record_offset":3472,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":89977,"source_line":219,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2777,"record_offset":3492,"parent_record_offset":3472,"ancestor_record_offset":3388,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":4348,"parent_record_offset":3932,"ancestor_record_offset":3388,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":4256,"parent_record_offset":4200,"ancestor_record_offset":3388,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":3388,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":4060,"parent_record_offset":3976,"ancestor_record_offset":3388,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2777,"record_offset":4120,"parent_record_offset":3932,"ancestor_record_offset":3388,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2777,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":3388,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2777,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3388,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2777,"record_offset":4728,"parent_record_offset":4700,"ancestor_record_offset":3388,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2777,"record_offset":4668,"parent_record_offset":4644,"ancestor_record_offset":3388,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2777,"record_offset":4816,"parent_record_offset":4788,"ancestor_record_offset":3388,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2777,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":3388,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2777,"record_offset":3576,"parent_record_offset":3552,"ancestor_record_offset":3388,"inlinee":25468,"source_line":17,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2777,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":3388,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2777,"record_offset":3760,"parent_record_offset":3640,"ancestor_record_offset":3388,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2777,"record_offset":3640,"parent_record_offset":3604,"ancestor_record_offset":3388,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2777,"record_offset":3700,"parent_record_offset":3668,"ancestor_record_offset":3388,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2777,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3388,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2777,"record_offset":4376,"parent_record_offset":3932,"ancestor_record_offset":3388,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":5528,"parent_record_offset":5496,"ancestor_record_offset":3388,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":5496,"parent_record_offset":5464,"ancestor_record_offset":3388,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2777,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":3388,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2777,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":3388,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2777,"record_offset":3976,"parent_record_offset":3932,"ancestor_record_offset":3388,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2777,"record_offset":5560,"parent_record_offset":5528,"ancestor_record_offset":3388,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2777,"record_offset":3812,"parent_record_offset":3760,"ancestor_record_offset":3388,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2777,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":3388,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2772,"record_offset":256,"rva":24990176,"va":5393699296}
// 编译实例 RVAs: 0x17d51e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: merge_proposal_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24086859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e8b57690fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5393700374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: merge_proposal_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19145873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e86fdbdbfe","size":5,"soft_fail":false,"target_va":5374554704,"va":5393700572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: merge_proposal_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18490815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e841dae5fe","size":5,"soft_fail":false,"target_va":5375209776,"va":5393700586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406bb180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: merge_proposal_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17933961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e87759eefe","size":5,"soft_fail":false,"target_va":5375766912,"va":5393700868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: merge_proposal_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11333604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e81c1053ff","size":5,"soft_fail":false,"target_va":5382367008,"va":5393700607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d9950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e84e460000","size":5,"soft_fail":false,"target_va":5393717584,"va":5393699581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9277000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e8488e8d00","size":5,"soft_fail":false,"target_va":5402978912,"va":5393701907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9276850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e8b28d8d00","size":5,"soft_fail":false,"target_va":5402978880,"va":5393702025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9276926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e8fe8d8d00","size":5,"soft_fail":false,"target_va":5402978944,"va":5393702013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19129894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e8da19dcfe","size":5,"soft_fail":false,"target_va":5374571120,"va":5393701009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19129887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e8e119dcfe","size":5,"soft_fail":false,"target_va":5374570784,"va":5393700666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18487870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e8c2e5e5fe","size":5,"soft_fail":false,"target_va":5375212688,"va":5393700553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17975889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e8afb5edfe","size":5,"soft_fail":false,"target_va":5375724992,"va":5393700876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17976215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e869b4edfe","size":5,"soft_fail":false,"target_va":5375724768,"va":5393700978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::edge_length
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15733602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e89eec0fff","size":5,"soft_fail":false,"target_va":5377967328,"va":5393700925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15739783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393699296,"raw_hex":"e879d40fff","size":5,"soft_fail":false,"target_va":5377961136,"va":5393700914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":524,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":12473,"source_line":929,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":580,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8709,"source_line":3310,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2772,"record_offset":440,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2772,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":256,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2772,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":256,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2772,"record_offset":3300,"parent_record_offset":3272,"ancestor_record_offset":256,"inlinee":6144,"source_line":1275,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2772,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":256,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2772,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":256,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2772,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":256,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2772,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":256,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2772,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":256,"inlinee":12475,"source_line":23,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2772,"record_offset":1084,"parent_record_offset":936,"ancestor_record_offset":256,"inlinee":25841,"source_line":39,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2772,"record_offset":900,"parent_record_offset":612,"ancestor_record_offset":256,"inlinee":25235,"source_line":144,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2772,"record_offset":648,"parent_record_offset":612,"ancestor_record_offset":256,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2772,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":256,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2772,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":256,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":2772,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":256,"inlinee":28256,"source_line":104,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2772,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":256,"inlinee":13759,"source_line":49,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2772,"record_offset":2780,"parent_record_offset":2728,"ancestor_record_offset":256,"inlinee":13764,"source_line":53,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2436,"parent_record_offset":2396,"ancestor_record_offset":256,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":256,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":3272,"parent_record_offset":1084,"ancestor_record_offset":256,"inlinee":6142,"source_line":3317,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":256,"inlinee":8710,"source_line":2618,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2632,"parent_record_offset":2396,"ancestor_record_offset":256,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":256,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2772,"record_offset":3448,"parent_record_offset":936,"ancestor_record_offset":256,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2772,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":256,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2772,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":256,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2772,"record_offset":3336,"parent_record_offset":3300,"ancestor_record_offset":256,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2772,"record_offset":3392,"parent_record_offset":3364,"ancestor_record_offset":256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2772,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":256,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$41::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2772,"record_offset":872,"parent_record_offset":612,"ancestor_record_offset":256,"inlinee":5368,"source_line":2496,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2772,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":256,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2772,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2772,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":256,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2772,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":256,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2772,"record_offset":828,"parent_record_offset":800,"ancestor_record_offset":256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2772,"record_offset":1028,"parent_record_offset":936,"ancestor_record_offset":256,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2772,"record_offset":768,"parent_record_offset":716,"ancestor_record_offset":256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2772,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":256,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2772,"record_offset":800,"parent_record_offset":716,"ancestor_record_offset":256,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2772,"record_offset":676,"parent_record_offset":648,"ancestor_record_offset":256,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2772,"record_offset":936,"parent_record_offset":900,"ancestor_record_offset":256,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2772,"record_offset":716,"parent_record_offset":676,"ancestor_record_offset":256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2772,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":256,"inlinee":25468,"source_line":17,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2772,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":256,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2772,"record_offset":2916,"parent_record_offset":2864,"ancestor_record_offset":256,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2772,"record_offset":2336,"parent_record_offset":2304,"ancestor_record_offset":256,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2772,"record_offset":2304,"parent_record_offset":1144,"ancestor_record_offset":256,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2772,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":256,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2772,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":256,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2772,"record_offset":3036,"parent_record_offset":1144,"ancestor_record_offset":256,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2772,"record_offset":2980,"parent_record_offset":1144,"ancestor_record_offset":256,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2772,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":256,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2772,"record_offset":1324,"parent_record_offset":1288,"ancestor_record_offset":256,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2772,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":256,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2772,"record_offset":2036,"parent_record_offset":1816,"ancestor_record_offset":256,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2772,"record_offset":1380,"parent_record_offset":1348,"ancestor_record_offset":256,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":256,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1924,"parent_record_offset":1816,"ancestor_record_offset":256,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":256,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":2244,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":256,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":256,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1408,"parent_record_offset":1348,"ancestor_record_offset":256,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":256,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2772,"record_offset":1696,"parent_record_offset":1668,"ancestor_record_offset":256,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2772,"record_offset":1724,"parent_record_offset":1696,"ancestor_record_offset":256,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2772,"record_offset":1784,"parent_record_offset":1668,"ancestor_record_offset":256,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2772,"record_offset":1668,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1500,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1816,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2772,"record_offset":1528,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2772,"record_offset":1612,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2772,"record_offset":2152,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2772,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":256,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2772,"record_offset":1256,"parent_record_offset":1144,"ancestor_record_offset":256,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2772,"record_offset":3092,"parent_record_offset":3060,"ancestor_record_offset":256,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_roof_decorators.rs
//     evidence={"module":2772,"record_offset":1144,"parent_record_offset":1084,"ancestor_record_offset":256,"inlinee":89919,"source_line":116,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_roof_decorators.rs
//     evidence={"module":2772,"record_offset":3364,"parent_record_offset":3336,"ancestor_record_offset":256,"inlinee":89923,"source_line":166,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2772,"record_offset":2396,"parent_record_offset":1144,"ancestor_record_offset":256,"inlinee":89921,"source_line":289,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2772,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":256,"inlinee":89922,"source_line":296,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2772,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":256,"inlinee":89889,"source_line":301,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2772,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":256,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F3F
// 首条位置: {"module":2772,"inlinee":89919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$41::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2777,"record_offset":2332,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":5368,"source_line":2496,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2772,"record_offset":2304,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2772,"record_offset":3036,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2772,"record_offset":2980,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2772,"record_offset":1256,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::try_merge_decorators
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2772,"record_offset":2396,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":89921,"source_line":289,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2772,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":1144,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2772,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1144,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2772,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":1144,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2772,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":1144,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2772,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":1144,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2772,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":1144,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2772,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":1144,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2772,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":1144,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":2772,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":1144,"inlinee":28256,"source_line":104,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2772,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":1144,"inlinee":13759,"source_line":49,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2772,"record_offset":2780,"parent_record_offset":2728,"ancestor_record_offset":1144,"inlinee":13764,"source_line":53,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2436,"parent_record_offset":2396,"ancestor_record_offset":1144,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":1144,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2632,"parent_record_offset":2396,"ancestor_record_offset":1144,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":1144,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2772,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":1144,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2772,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":1144,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2772,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1144,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2772,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":1144,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2772,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1144,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2772,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":1144,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$19::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2772,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":1144,"inlinee":25468,"source_line":17,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2772,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1144,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2772,"record_offset":2916,"parent_record_offset":2864,"ancestor_record_offset":1144,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2772,"record_offset":2336,"parent_record_offset":2304,"ancestor_record_offset":1144,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2772,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":1144,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2772,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":1144,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2772,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1144,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2772,"record_offset":1324,"parent_record_offset":1288,"ancestor_record_offset":1144,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2772,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":1144,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2772,"record_offset":2036,"parent_record_offset":1816,"ancestor_record_offset":1144,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2772,"record_offset":1380,"parent_record_offset":1348,"ancestor_record_offset":1144,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":1144,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1924,"parent_record_offset":1816,"ancestor_record_offset":1144,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":1144,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":2244,"parent_record_offset":1456,"ancestor_record_offset":1144,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":1144,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":1144,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1408,"parent_record_offset":1348,"ancestor_record_offset":1144,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1144,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2772,"record_offset":1696,"parent_record_offset":1668,"ancestor_record_offset":1144,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2772,"record_offset":1724,"parent_record_offset":1696,"ancestor_record_offset":1144,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2772,"record_offset":1784,"parent_record_offset":1668,"ancestor_record_offset":1144,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2772,"record_offset":1668,"parent_record_offset":1456,"ancestor_record_offset":1144,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1500,"parent_record_offset":1456,"ancestor_record_offset":1144,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2772,"record_offset":1816,"parent_record_offset":1456,"ancestor_record_offset":1144,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2772,"record_offset":1528,"parent_record_offset":1456,"ancestor_record_offset":1144,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2772,"record_offset":1612,"parent_record_offset":1456,"ancestor_record_offset":1144,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2772,"record_offset":2152,"parent_record_offset":1456,"ancestor_record_offset":1144,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2772,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":1144,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2772,"record_offset":3092,"parent_record_offset":3060,"ancestor_record_offset":1144,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2772,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":1144,"inlinee":89922,"source_line":296,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2772,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":1144,"inlinee":89889,"source_line":301,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2772,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":1144,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F43
// 首条位置: {"module":2772,"inlinee":89923}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2772,"record_offset":3392,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02772.jsonl.gz"}
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
// 完整结构及原始类型记录: merge_proposal_roof_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1>", "constructor": "core::option::Option::and_then", "arguments": ["ref$<country_core::resources::walls::decorator_storage::Decorators>", "system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal", "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: merge_proposal_roof_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: merge_proposal_roof_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position
// 信息 ID: a1250711d92f205c0f01c5f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> @ 0xd09fa2 -> 0x17d51e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure$0
// 信息 ID: 2410260ffa1b3adbce084405
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position; module=2772, inline_record=1144; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0>; module=2777, inline_record=564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure$1
// 信息 ID: 48e13c896931b5572472d9bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position; module=2772, inline_record=3364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure$0
// 信息 ID: 6a9f7d66565f59ec7ec6c2ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0> >; module=2777, inline_record=3388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0>
// 信息 ID: 33aff08fd6ef67ff34ff9171
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators -> ["system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0b252 -> 0x17da0f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
