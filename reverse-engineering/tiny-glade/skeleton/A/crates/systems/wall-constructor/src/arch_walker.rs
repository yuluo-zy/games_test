// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/arch_walker.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 093ebc0b0b18bc34bd034563571fce4ef5200f340474b22820e80e935ed15dc4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: arch_walker.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 361, 1232, 1235
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::new
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5378
// 首条位置: {"module":361,"inlinee":21368}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1232,"record_offset":7916,"rva":13348864,"va":5382057984}
// 编译实例 RVAs: 0xcbb000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5114202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382057984,"raw_hex":"e8a6f6b1ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5382058197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t196747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382057984,"raw_hex":"e88b000300","size":5,"soft_fail":false,"target_va":5382255968,"va":5382059216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t197088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382057984,"raw_hex":"e8e0010300","size":5,"soft_fail":false,"target_va":5382256288,"va":5382059195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21535464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382057984,"raw_hex":"e8e89a4801","size":5,"soft_fail":false,"target_va":5403594096,"va":5382058627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20919268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382057984,"raw_hex":"e8e4333f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382059879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20919528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382057984,"raw_hex":"e8e8343f01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382059811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20918960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382057984,"raw_hex":"e8b0323f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382059979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_arches::construct_arches::closure$3
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382057984,"raw_hex":"e8cdebffff","size":5,"soft_fail":false,"target_va":5382054224,"va":5382059390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":8700,"parent_record_offset":7916,"ancestor_record_offset":7916,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1232,"record_offset":8188,"parent_record_offset":7916,"ancestor_record_offset":7916,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$50::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1232,"record_offset":9024,"parent_record_offset":7916,"ancestor_record_offset":7916,"inlinee":59719,"source_line":35,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":8572,"parent_record_offset":7916,"ancestor_record_offset":7916,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":8484,"parent_record_offset":7916,"ancestor_record_offset":7916,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1232,"record_offset":8672,"parent_record_offset":7916,"ancestor_record_offset":7916,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1232,"record_offset":9108,"parent_record_offset":7916,"ancestor_record_offset":7916,"inlinee":59720,"source_line":122,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1232,"record_offset":8160,"parent_record_offset":7916,"ancestor_record_offset":7916,"inlinee":59717,"source_line":163,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1232,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":7916,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1232,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":7916,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1232,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":7916,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":9568,"parent_record_offset":9420,"ancestor_record_offset":7916,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":9364,"parent_record_offset":9108,"ancestor_record_offset":7916,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1232,"record_offset":9596,"parent_record_offset":9420,"ancestor_record_offset":7916,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1232,"record_offset":8840,"parent_record_offset":8812,"ancestor_record_offset":7916,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":8344,"parent_record_offset":8312,"ancestor_record_offset":7916,"inlinee":23169,"source_line":26,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":8228,"parent_record_offset":8188,"ancestor_record_offset":7916,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":8288,"parent_record_offset":8188,"ancestor_record_offset":7916,"inlinee":23167,"source_line":94,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":7916,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":7916,"inlinee":23168,"source_line":340,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1232,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":7916,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1232,"record_offset":8928,"parent_record_offset":8812,"ancestor_record_offset":7916,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1232,"record_offset":8812,"parent_record_offset":8788,"ancestor_record_offset":7916,"inlinee":15501,"source_line":305,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1232,"record_offset":8788,"parent_record_offset":8756,"ancestor_record_offset":7916,"inlinee":11441,"source_line":357,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":7916,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":8512,"parent_record_offset":8484,"ancestor_record_offset":7916,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1232,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":7916,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1232,"record_offset":9456,"parent_record_offset":9420,"ancestor_record_offset":7916,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":8600,"parent_record_offset":8572,"ancestor_record_offset":7916,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":9272,"parent_record_offset":9108,"ancestor_record_offset":7916,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":9180,"parent_record_offset":9108,"ancestor_record_offset":7916,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1232,"record_offset":8636,"parent_record_offset":8600,"ancestor_record_offset":7916,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1232,"record_offset":9392,"parent_record_offset":9108,"ancestor_record_offset":7916,"inlinee":59697,"source_line":25,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1232,"record_offset":9420,"parent_record_offset":9108,"ancestor_record_offset":7916,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1232,"record_offset":2300,"rva":13346016,"va":5382055136}
// 编译实例 RVAs: 0xcba4e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12243374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e8522e45ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382056921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5111361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e8bf01b2ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5382055356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-186585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e82727fdff","size":5,"soft_fail":false,"target_va":5381870240,"va":5382056820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-137230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e8f2e7fdff","size":5,"soft_fail":false,"target_va":5381919744,"va":5382056969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t199514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e85a0b0300","size":5,"soft_fail":false,"target_va":5382255968,"va":5382056449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t199854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e8ae0c0300","size":5,"soft_fail":false,"target_va":5382256288,"va":5382056429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21538299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e8fba54801","size":5,"soft_fail":false,"target_va":5403594096,"va":5382055792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20921297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e8d13b3f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382057850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20921557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e8d53c3f01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382057782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20920989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382055136,"raw_hex":"e89d3a3f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382057950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":3192,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1232,"record_offset":2680,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$50::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1232,"record_offset":3516,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":59719,"source_line":35,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":3064,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":2976,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1232,"record_offset":3164,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1232,"record_offset":3600,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":59720,"source_line":122,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1232,"record_offset":2652,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":59717,"source_line":163,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1232,"record_offset":5992,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":59722,"source_line":578,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1232,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":2300,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1232,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":2300,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1232,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2300,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1232,"record_offset":4516,"parent_record_offset":4116,"ancestor_record_offset":2300,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":5096,"parent_record_offset":5040,"ancestor_record_offset":2300,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":2300,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":4656,"parent_record_offset":4600,"ancestor_record_offset":2300,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":4056,"parent_record_offset":3908,"ancestor_record_offset":2300,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":3852,"parent_record_offset":3600,"ancestor_record_offset":2300,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":4424,"parent_record_offset":4392,"ancestor_record_offset":2300,"inlinee":21457,"source_line":867,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1232,"record_offset":4084,"parent_record_offset":3908,"ancestor_record_offset":2300,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1232,"record_offset":3332,"parent_record_offset":3304,"ancestor_record_offset":2300,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":2836,"parent_record_offset":2804,"ancestor_record_offset":2300,"inlinee":23169,"source_line":26,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":2720,"parent_record_offset":2680,"ancestor_record_offset":2300,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":2780,"parent_record_offset":2680,"ancestor_record_offset":2300,"inlinee":23167,"source_line":94,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":2300,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1232,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":2300,"inlinee":23168,"source_line":340,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1232,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":2300,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1232,"record_offset":3420,"parent_record_offset":3304,"ancestor_record_offset":2300,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1232,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":2300,"inlinee":15501,"source_line":305,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1232,"record_offset":3280,"parent_record_offset":3248,"ancestor_record_offset":2300,"inlinee":11441,"source_line":357,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":4452,"parent_record_offset":4392,"ancestor_record_offset":2300,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":2300,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":3004,"parent_record_offset":2976,"ancestor_record_offset":2300,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1232,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":2300,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1232,"record_offset":3944,"parent_record_offset":3908,"ancestor_record_offset":2300,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1232,"record_offset":4784,"parent_record_offset":4116,"ancestor_record_offset":2300,"inlinee":21492,"source_line":127,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1232,"record_offset":4896,"parent_record_offset":4116,"ancestor_record_offset":2300,"inlinee":21497,"source_line":538,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":2300,"inlinee":21494,"source_line":148,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":2300,"inlinee":21500,"source_line":526,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":2300,"inlinee":21503,"source_line":516,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":2300,"inlinee":21495,"source_line":66,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4932,"parent_record_offset":4896,"ancestor_record_offset":2300,"inlinee":21499,"source_line":661,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4988,"parent_record_offset":4956,"ancestor_record_offset":2300,"inlinee":21502,"source_line":676,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4600,"parent_record_offset":4116,"ancestor_record_offset":2300,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":2300,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$13::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":2300,"inlinee":21507,"source_line":1077,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":2300,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$8::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":2300,"inlinee":21505,"source_line":1039,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":2300,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":3760,"parent_record_offset":3600,"ancestor_record_offset":2300,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":2300,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4368,"parent_record_offset":4116,"ancestor_record_offset":2300,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":3668,"parent_record_offset":3600,"ancestor_record_offset":2300,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1232,"record_offset":5276,"parent_record_offset":5248,"ancestor_record_offset":2300,"inlinee":21509,"source_line":178,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1232,"record_offset":3128,"parent_record_offset":3092,"ancestor_record_offset":2300,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1232,"record_offset":5924,"parent_record_offset":4116,"ancestor_record_offset":2300,"inlinee":59723,"source_line":93,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1232,"record_offset":4116,"parent_record_offset":3600,"ancestor_record_offset":2300,"inlinee":59722,"source_line":578,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1232,"record_offset":3880,"parent_record_offset":3600,"ancestor_record_offset":2300,"inlinee":59697,"source_line":25,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1232,"record_offset":3908,"parent_record_offset":3600,"ancestor_record_offset":2300,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE948
// 首条位置: {"module":1232,"inlinee":59720}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":3852,"parent_record_offset":3600,"ancestor_record_offset":3600,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":3760,"parent_record_offset":3600,"ancestor_record_offset":3600,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":3668,"parent_record_offset":3600,"ancestor_record_offset":3600,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1232,"record_offset":4116,"parent_record_offset":3600,"ancestor_record_offset":3600,"inlinee":59722,"source_line":578,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1232,"record_offset":3880,"parent_record_offset":3600,"ancestor_record_offset":3600,"inlinee":59697,"source_line":25,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1232,"record_offset":3908,"parent_record_offset":3600,"ancestor_record_offset":3600,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1232,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":3600,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1232,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":3600,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1232,"record_offset":4516,"parent_record_offset":4116,"ancestor_record_offset":3600,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":5096,"parent_record_offset":5040,"ancestor_record_offset":3600,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":3600,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":4656,"parent_record_offset":4600,"ancestor_record_offset":3600,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":4056,"parent_record_offset":3908,"ancestor_record_offset":3600,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":4424,"parent_record_offset":4392,"ancestor_record_offset":3600,"inlinee":21457,"source_line":867,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1232,"record_offset":4084,"parent_record_offset":3908,"ancestor_record_offset":3600,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":4452,"parent_record_offset":4392,"ancestor_record_offset":3600,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":3700,"parent_record_offset":3668,"ancestor_record_offset":3600,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1232,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":3600,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1232,"record_offset":3944,"parent_record_offset":3908,"ancestor_record_offset":3600,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1232,"record_offset":4784,"parent_record_offset":4116,"ancestor_record_offset":3600,"inlinee":21492,"source_line":127,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1232,"record_offset":4896,"parent_record_offset":4116,"ancestor_record_offset":3600,"inlinee":21497,"source_line":538,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":3600,"inlinee":21494,"source_line":148,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":3600,"inlinee":21500,"source_line":526,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":3600,"inlinee":21503,"source_line":516,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":3600,"inlinee":21495,"source_line":66,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4932,"parent_record_offset":4896,"ancestor_record_offset":3600,"inlinee":21499,"source_line":661,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4988,"parent_record_offset":4956,"ancestor_record_offset":3600,"inlinee":21502,"source_line":676,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4600,"parent_record_offset":4116,"ancestor_record_offset":3600,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":3600,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$13::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":3600,"inlinee":21507,"source_line":1077,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":3600,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$8::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":3600,"inlinee":21505,"source_line":1039,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":3600,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4268,"parent_record_offset":4116,"ancestor_record_offset":3600,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":3600,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4368,"parent_record_offset":4116,"ancestor_record_offset":3600,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4180,"parent_record_offset":4116,"ancestor_record_offset":3600,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1232,"record_offset":5276,"parent_record_offset":5248,"ancestor_record_offset":3600,"inlinee":21509,"source_line":178,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1232,"record_offset":3816,"parent_record_offset":3784,"ancestor_record_offset":3600,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1232,"record_offset":5924,"parent_record_offset":4116,"ancestor_record_offset":3600,"inlinee":59723,"source_line":93,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::curve_to_arch_ws
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE945
// 首条位置: {"module":1232,"inlinee":59717}
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
// 完整结构及原始类型记录: arch_walker.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::arch_walker::ArchWalker
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_arches::construct_arches::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: arch_walker.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: arch_walker.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
// 信息 ID: c0a5f03edaea496c313bb4c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1232, inline_record=3600; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>; module=1232, inline_record=9108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::new
// 信息 ID: 4808abe8bc55b909b55caa45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=23316; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=9528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>
// 信息 ID: 6abdd3b0b611d2b9c49c98f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_arches::construct_arches::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8c05 -> 0xcbb000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >
// 信息 ID: cea41f75419e9213907c179c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0 -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbd9d8 -> 0xcba4e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::curve_to_arch_ws
// 信息 ID: c29e854d1bd51d7a0487873d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1232, inline_record=2652; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1232, inline_record=2948; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1232, inline_record=3220; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1232, inline_record=3572; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1232, inline_record=5964; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>; module=1232, inline_record=8160; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>; module=1232, inline_record=8456; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>; module=1232, inline_record=8728; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
