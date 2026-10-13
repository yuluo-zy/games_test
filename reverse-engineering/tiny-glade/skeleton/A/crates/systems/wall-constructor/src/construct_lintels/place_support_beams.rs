// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5197ecdfb2119ff545bcabd6367b03ae70df9cb6c994373f580ca8be9cf74f9e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: place_support_beams.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1232, 1235, 2707
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1235,"record_offset":2676,"rva":13357552,"va":5382066672}
// 编译实例 RVAs: 0xcbd1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12254091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382066672,"raw_hex":"e8750445ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382067638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-197393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382066672,"raw_hex":"e8effcfcff","size":5,"soft_fail":false,"target_va":5381870240,"va":5382067628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-147930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382066672,"raw_hex":"e826befdff","size":5,"soft_fail":false,"target_va":5381919744,"va":5382067669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc6920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382066672,"raw_hex":"e89b950000","size":5,"soft_fail":false,"target_va":5382105376,"va":5382067072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc7340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382066672,"raw_hex":"e8d19f0000","size":5,"soft_fail":false,"target_va":5382107968,"va":5382067050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t20911561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382066672,"raw_hex":"e8c9153f01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382067778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1235,"record_offset":4628,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1235,"record_offset":5352,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":21492,"source_line":127,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1235,"record_offset":5480,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":21497,"source_line":538,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1235,"record_offset":4712,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1235,"record_offset":4888,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":36061,"source_line":541,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":4388,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":4480,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":4212,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":4300,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":4156,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1235,"record_offset":6636,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":59723,"source_line":93,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1235,"record_offset":3460,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1235,"record_offset":3404,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1235,"record_offset":2976,"parent_record_offset":2676,"ancestor_record_offset":2676,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1235,"record_offset":5744,"parent_record_offset":5688,"ancestor_record_offset":2676,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1235,"record_offset":5060,"parent_record_offset":4888,"ancestor_record_offset":2676,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1235,"record_offset":4772,"parent_record_offset":4712,"ancestor_record_offset":2676,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1235,"record_offset":4744,"parent_record_offset":4712,"ancestor_record_offset":2676,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1235,"record_offset":5032,"parent_record_offset":4888,"ancestor_record_offset":2676,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1235,"record_offset":4828,"parent_record_offset":4712,"ancestor_record_offset":2676,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1235,"record_offset":4536,"parent_record_offset":4504,"ancestor_record_offset":2676,"inlinee":21457,"source_line":867,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1235,"record_offset":4000,"parent_record_offset":3968,"ancestor_record_offset":2676,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1235,"record_offset":4092,"parent_record_offset":4060,"ancestor_record_offset":2676,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1235,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":2676,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":3040,"parent_record_offset":2976,"ancestor_record_offset":2676,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":3068,"parent_record_offset":3040,"ancestor_record_offset":2676,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1235,"record_offset":5380,"parent_record_offset":5352,"ancestor_record_offset":2676,"inlinee":21494,"source_line":148,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1235,"record_offset":5544,"parent_record_offset":5516,"ancestor_record_offset":2676,"inlinee":21500,"source_line":526,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1235,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":2676,"inlinee":21503,"source_line":516,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1235,"record_offset":5408,"parent_record_offset":5380,"ancestor_record_offset":2676,"inlinee":21495,"source_line":66,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1235,"record_offset":5516,"parent_record_offset":5480,"ancestor_record_offset":2676,"inlinee":21499,"source_line":661,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1235,"record_offset":5576,"parent_record_offset":5544,"ancestor_record_offset":2676,"inlinee":21502,"source_line":676,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1235,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":2676,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$13::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1235,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":2676,"inlinee":21507,"source_line":1077,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1235,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":2676,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$8::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1235,"record_offset":5632,"parent_record_offset":5600,"ancestor_record_offset":2676,"inlinee":21505,"source_line":1039,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":2676,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":2676,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":4060,"parent_record_offset":3684,"ancestor_record_offset":2676,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":3876,"parent_record_offset":3684,"ancestor_record_offset":2676,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1235,"record_offset":3968,"parent_record_offset":3684,"ancestor_record_offset":2676,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1235,"record_offset":4944,"parent_record_offset":4916,"ancestor_record_offset":2676,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1235,"record_offset":4916,"parent_record_offset":4888,"ancestor_record_offset":2676,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1235,"record_offset":5980,"parent_record_offset":5896,"ancestor_record_offset":2676,"inlinee":21509,"source_line":178,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1235,"record_offset":4444,"parent_record_offset":4412,"ancestor_record_offset":2676,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1235,"record_offset":3012,"parent_record_offset":2976,"ancestor_record_offset":2676,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1235,"record_offset":3104,"parent_record_offset":2976,"ancestor_record_offset":2676,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1235,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":2676,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1235,"record_offset":260,"rva":13352976,"va":5382062096}
// 编译实例 RVAs: 0xcbc010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8912426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382062096,"raw_hex":"e8d60178ff","size":5,"soft_fail":false,"target_va":5373149840,"va":5382062261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20912481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382062096,"raw_hex":"e861193f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382066666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t20912869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382062096,"raw_hex":"e8e51a3f01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382066470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t3780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382062096,"raw_hex":"e8c40e0000","size":5,"soft_fail":false,"target_va":5382067904,"va":5382064119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t11249577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382062096,"raw_hex":"e8a9a7ab00","size":5,"soft_fail":false,"target_va":5393312432,"va":5382062850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382062096,"raw_hex":"e865080000","size":5,"soft_fail":false,"target_va":5382066672,"va":5382064518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::peekable::Peekable::peek
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\peekable.rs
//     evidence={"module":1235,"record_offset":852,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":16071,"source_line":215,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::peekable::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\peekable.rs
//     evidence={"module":1235,"record_offset":548,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":16075,"source_line":37,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1235,"record_offset":1368,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7276,"source_line":33,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":1568,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":2476,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1235,"record_offset":2116,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5483,"source_line":378,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":1312,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::accept
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1235,"record_offset":2364,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59746,"source_line":532,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1235,"record_offset":1236,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59741,"source_line":474,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1235,"record_offset":1996,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59744,"source_line":130,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1235,"record_offset":2056,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59745,"source_line":139,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1235,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":260,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::peekable::impl$5::peek::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\peekable.rs
//     evidence={"module":1235,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":260,"inlinee":16073,"source_line":217,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":260,"inlinee":14451,"source_line":213,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":260,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":1416,"parent_record_offset":1392,"ancestor_record_offset":260,"inlinee":7279,"source_line":798,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":260,"inlinee":7278,"source_line":967,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1235,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1235,"record_offset":752,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1235,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":260,"inlinee":14452,"source_line":664,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1235,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":260,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1235,"record_offset":1932,"parent_record_offset":1900,"ancestor_record_offset":260,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":880,"parent_record_offset":852,"ancestor_record_offset":260,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":260,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1235,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":260,"inlinee":59739,"source_line":60,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1235,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":260,"inlinee":59742,"source_line":80,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1235,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":260,"inlinee":59743,"source_line":83,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE95B
// 首条位置: {"module":1235,"inlinee":59739}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE95E
// 首条位置: {"module":1235,"inlinee":59742}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE95F
// 首条位置: {"module":1235,"inlinee":59743}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE960
// 首条位置: {"module":1235,"inlinee":59744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":1996,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE961
// 首条位置: {"module":1235,"inlinee":59745}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":2084,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 430
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1235,"record_offset":6860,"rva":13358784,"va":5382067904}
// 编译实例 RVAs: 0xcbd6c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9318798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e872ce71ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382068489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9318476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e8b4cf71ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382068183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cba4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e803cbffff","size":5,"soft_fail":false,"target_va":5382055136,"va":5382068696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141776c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11244760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e8d894ab00","size":5,"soft_fail":false,"target_va":5393312896,"va":5382068131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: place_support_beams.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21524853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e875714801","size":5,"soft_fail":false,"target_va":5403593392,"va":5382068534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20909314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e8020d3f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382068825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4541717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e8ebb2baff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382068832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20910097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e811103f01","size":5,"soft_fail":false,"target_va":5402978912,"va":5382068810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20911515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e89b153f01","size":5,"soft_fail":false,"target_va":5402980304,"va":5382068784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_overflow_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20911645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e81d163f01","size":5,"soft_fail":false,"target_va":5402980400,"va":5382068750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20911551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e8bf153f01","size":5,"soft_fail":false,"target_va":5402980320,"va":5382068764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::u_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-135112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382067904,"raw_hex":"e838f0fdff","size":5,"soft_fail":false,"target_va":5381933088,"va":5382068195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":7132,"parent_record_offset":6860,"ancestor_record_offset":6860,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1235,"record_offset":7556,"parent_record_offset":6860,"ancestor_record_offset":6860,"inlinee":59748,"source_line":542,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":6860,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":6860,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":6860,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":9172,"parent_record_offset":9076,"ancestor_record_offset":6860,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":7836,"parent_record_offset":7736,"ancestor_record_offset":6860,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":7768,"parent_record_offset":7736,"ancestor_record_offset":6860,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":9076,"parent_record_offset":9048,"ancestor_record_offset":6860,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":9048,"parent_record_offset":9020,"ancestor_record_offset":6860,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":9292,"parent_record_offset":9048,"ancestor_record_offset":6860,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":6860,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":9332,"parent_record_offset":8984,"ancestor_record_offset":6860,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":9020,"parent_record_offset":8984,"ancestor_record_offset":6860,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":8952,"parent_record_offset":7556,"ancestor_record_offset":6860,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1235,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":6860,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1235,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":6860,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1235,"record_offset":7964,"parent_record_offset":7936,"ancestor_record_offset":6860,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1235,"record_offset":8984,"parent_record_offset":8952,"ancestor_record_offset":6860,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1235,"record_offset":9108,"parent_record_offset":9076,"ancestor_record_offset":6860,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1235,"record_offset":9136,"parent_record_offset":9108,"ancestor_record_offset":6860,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":8756,"parent_record_offset":8720,"ancestor_record_offset":6860,"inlinee":14887,"source_line":1555,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":9424,"parent_record_offset":9396,"ancestor_record_offset":6860,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":8672,"parent_record_offset":7556,"ancestor_record_offset":6860,"inlinee":59750,"source_line":1265,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::count::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":8820,"parent_record_offset":8796,"ancestor_record_offset":6860,"inlinee":59752,"source_line":1271,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":9396,"parent_record_offset":9368,"ancestor_record_offset":6860,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":9368,"parent_record_offset":9332,"ancestor_record_offset":6860,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":8844,"parent_record_offset":8820,"ancestor_record_offset":6860,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":8876,"parent_record_offset":8844,"ancestor_record_offset":6860,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":7800,"parent_record_offset":7768,"ancestor_record_offset":6860,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1235,"record_offset":7204,"parent_record_offset":7164,"ancestor_record_offset":6860,"inlinee":27288,"source_line":454,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1235,"record_offset":8720,"parent_record_offset":8672,"ancestor_record_offset":6860,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":8796,"parent_record_offset":8672,"ancestor_record_offset":6860,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":8920,"parent_record_offset":8672,"ancestor_record_offset":6860,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1235,"record_offset":7512,"parent_record_offset":7488,"ancestor_record_offset":6860,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":6860,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":8024,"parent_record_offset":7964,"ancestor_record_offset":6860,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":6860,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":6860,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":9248,"parent_record_offset":9220,"ancestor_record_offset":6860,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":9460,"parent_record_offset":9332,"ancestor_record_offset":6860,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":8532,"parent_record_offset":7556,"ancestor_record_offset":6860,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":7488,"parent_record_offset":7452,"ancestor_record_offset":6860,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":7236,"parent_record_offset":7164,"ancestor_record_offset":6860,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$8::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":7164,"parent_record_offset":7132,"ancestor_record_offset":6860,"inlinee":20602,"source_line":583,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1235,"record_offset":7908,"parent_record_offset":7556,"ancestor_record_offset":6860,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1235,"record_offset":8092,"parent_record_offset":7556,"ancestor_record_offset":6860,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1235,"record_offset":7936,"parent_record_offset":7908,"ancestor_record_offset":6860,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::arch_walker::ArchWalker::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1235,"record_offset":9528,"parent_record_offset":7556,"ancestor_record_offset":6860,"inlinee":21368,"source_line":22,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::Rdp::into_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":1235,"record_offset":7656,"parent_record_offset":7556,"ancestor_record_offset":6860,"inlinee":59749,"source_line":59,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::new
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 474
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE95D
// 首条位置: {"module":1235,"inlinee":59741}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose
// 记录形式: compiled_procedure; instances=1
// 源码行: 484
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2707,"record_offset":260,"rva":24603312,"va":5393312432}
// 编译实例 RVAs: 0x1776ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9666475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312432,"raw_hex":"e8ab7f9300","size":5,"soft_fail":false,"target_va":5402979344,"va":5393312864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":864,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":448,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":892,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":660,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":828,"parent_record_offset":688,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":524,"parent_record_offset":480,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":552,"parent_record_offset":480,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":480,"parent_record_offset":448,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":756,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":720,"parent_record_offset":688,"ancestor_record_offset":260,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":260,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2707,"record_offset":592,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::accept
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 532
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE962
// 首条位置: {"module":1235,"inlinee":59746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 542
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE964
// 首条位置: {"module":1235,"inlinee":59748}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":8952,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":8672,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":59750,"source_line":1265,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":8212,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":8532,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1235,"record_offset":7908,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1235,"record_offset":8092,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1235,"record_offset":9528,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":21368,"source_line":22,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] utils::rdp::Rdp::into_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":1235,"record_offset":7656,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":59749,"source_line":59,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":7556,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":7556,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7556,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":9172,"parent_record_offset":9076,"ancestor_record_offset":7556,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":7836,"parent_record_offset":7736,"ancestor_record_offset":7556,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":7768,"parent_record_offset":7736,"ancestor_record_offset":7556,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":9076,"parent_record_offset":9048,"ancestor_record_offset":7556,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":9048,"parent_record_offset":9020,"ancestor_record_offset":7556,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":9292,"parent_record_offset":9048,"ancestor_record_offset":7556,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":7556,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":9332,"parent_record_offset":8984,"ancestor_record_offset":7556,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":9020,"parent_record_offset":8984,"ancestor_record_offset":7556,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1235,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":7556,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1235,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":7556,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1235,"record_offset":7964,"parent_record_offset":7936,"ancestor_record_offset":7556,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1235,"record_offset":8984,"parent_record_offset":8952,"ancestor_record_offset":7556,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1235,"record_offset":9108,"parent_record_offset":9076,"ancestor_record_offset":7556,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1235,"record_offset":9136,"parent_record_offset":9108,"ancestor_record_offset":7556,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":8756,"parent_record_offset":8720,"ancestor_record_offset":7556,"inlinee":14887,"source_line":1555,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":9424,"parent_record_offset":9396,"ancestor_record_offset":7556,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::count::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":8820,"parent_record_offset":8796,"ancestor_record_offset":7556,"inlinee":59752,"source_line":1271,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":9396,"parent_record_offset":9368,"ancestor_record_offset":7556,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1235,"record_offset":9368,"parent_record_offset":9332,"ancestor_record_offset":7556,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":8844,"parent_record_offset":8820,"ancestor_record_offset":7556,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":8876,"parent_record_offset":8844,"ancestor_record_offset":7556,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":7800,"parent_record_offset":7768,"ancestor_record_offset":7556,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1235,"record_offset":8596,"parent_record_offset":8560,"ancestor_record_offset":7556,"inlinee":27288,"source_line":454,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1235,"record_offset":8720,"parent_record_offset":8672,"ancestor_record_offset":7556,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":8796,"parent_record_offset":8672,"ancestor_record_offset":7556,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1235,"record_offset":8920,"parent_record_offset":8672,"ancestor_record_offset":7556,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":7556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":8024,"parent_record_offset":7964,"ancestor_record_offset":7556,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":7556,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":7556,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":9248,"parent_record_offset":9220,"ancestor_record_offset":7556,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":9460,"parent_record_offset":9332,"ancestor_record_offset":7556,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":8624,"parent_record_offset":8560,"ancestor_record_offset":7556,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$8::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":8560,"parent_record_offset":8532,"ancestor_record_offset":7556,"inlinee":20602,"source_line":583,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1235,"record_offset":7936,"parent_record_offset":7908,"ancestor_record_offset":7556,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 578
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE94A
// 首条位置: {"module":1232,"inlinee":59722}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1232,"record_offset":4516,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1232,"record_offset":4784,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":21492,"source_line":127,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1232,"record_offset":4896,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":21497,"source_line":538,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4600,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4268,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4368,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4180,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1232,"record_offset":5924,"parent_record_offset":4116,"ancestor_record_offset":4116,"inlinee":59723,"source_line":93,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":5096,"parent_record_offset":5040,"ancestor_record_offset":4116,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":4116,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1232,"record_offset":4656,"parent_record_offset":4600,"ancestor_record_offset":4116,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1232,"record_offset":4424,"parent_record_offset":4392,"ancestor_record_offset":4116,"inlinee":21457,"source_line":867,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":4452,"parent_record_offset":4392,"ancestor_record_offset":4116,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1232,"record_offset":4208,"parent_record_offset":4180,"ancestor_record_offset":4116,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":4116,"inlinee":21494,"source_line":148,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":4116,"inlinee":21500,"source_line":526,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":4116,"inlinee":21503,"source_line":516,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":4116,"inlinee":21495,"source_line":66,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4932,"parent_record_offset":4896,"ancestor_record_offset":4116,"inlinee":21499,"source_line":661,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1232,"record_offset":4988,"parent_record_offset":4956,"ancestor_record_offset":4116,"inlinee":21502,"source_line":676,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":4116,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$13::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":4116,"inlinee":21507,"source_line":1077,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":4116,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$8::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1232,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":4116,"inlinee":21505,"source_line":1039,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4296,"parent_record_offset":4268,"ancestor_record_offset":4116,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1232,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":4116,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1232,"record_offset":5276,"parent_record_offset":5248,"ancestor_record_offset":4116,"inlinee":21509,"source_line":178,"archive":"_pdb/A/modules/01232.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1232,"record_offset":4332,"parent_record_offset":4296,"ancestor_record_offset":4116,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01232.jsonl.gz"}
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
// 完整结构及原始类型记录: place_support_beams.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>", "constructor": "system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams", "arguments": ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: place_support_beams.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: place_support_beams.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 信息 ID: 6e56dd77eab31c70282053f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbcdc3 -> 0xcbd6c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbd032 -> 0xcbd6c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc7f7 -> 0xcbd6c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc872 -> 0xcbd6c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbcbb0 -> 0xcbd6c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::accept
// 信息 ID: 882b24242f3177cb2d325a52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=2364; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=2572; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=2644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::new
// 信息 ID: f1354635d973e29bdfcedc27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=1236; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=1272; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=2200; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=2436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose
// 信息 ID: 4198751727b0ed7136631507
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbcca3 -> 0x1776ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbcf13 -> 0x1776ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc302 -> 0x1776ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc33b -> 0x1776ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc724 -> 0x1776ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc773 -> 0x1776ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc3c2 -> 0x1776ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc54f -> 0x1776ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 信息 ID: fc50a322b184fe547fa326b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbce39 -> 0xcbd1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbd0b3 -> 0xcbd1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc986 -> 0xcbd1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbc9ef -> 0xcbd1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbca93 -> 0xcbd1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbcb2d -> 0xcbd1f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams
// 信息 ID: 8abf95280dda65a94a38f0cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=7556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
// 信息 ID: 138cf782924cab9d4f5623a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0 <- system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1232, inline_record=4116; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1232, inline_record=5992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$0
// 信息 ID: 52c47adc3b361ba6cdf2bc78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=800; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=1176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$1
// 信息 ID: 8412241fee814efd2403d201
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=1336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$2
// 信息 ID: 762933e7e41bb3156d6f26f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=1536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$3
// 信息 ID: f39c6d4d8c210fab376ea118
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=1996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams::closure$4
// 信息 ID: c228b4d872d8eeb56299d072
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::construct_lintels::place_support_beams::place_support_beams", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=2056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>
// 信息 ID: 5c1210adf97b8f6f15304890
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::place_support_beams -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4de7d5 -> 0xcbc010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
