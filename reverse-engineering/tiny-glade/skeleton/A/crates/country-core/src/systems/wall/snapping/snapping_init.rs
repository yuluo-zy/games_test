// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/snapping/snapping_init.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9a0738f8809db01558bb62c686ce56f5b169b750c8d4912d04aa02fbd3086caa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapping_init.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 401, 1321, 1395
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_init::snapping_init
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":401,"record_offset":15396,"rva":5578256,"va":5374287376}
// 编译实例 RVAs: 0x551e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: snapping_init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4673966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374287376,"raw_hex":"e852aeb8ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374287481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28690892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374287376,"raw_hex":"e8ccc9b501","size":5,"soft_fail":false,"target_va":5402978880,"va":5374287983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":401,"record_offset":15576,"parent_record_offset":15396,"ancestor_record_offset":15396,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":16772,"parent_record_offset":15396,"ancestor_record_offset":15396,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":17184,"parent_record_offset":15396,"ancestor_record_offset":15396,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":401,"record_offset":15632,"parent_record_offset":15396,"ancestor_record_offset":15396,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":401,"record_offset":15660,"parent_record_offset":15396,"ancestor_record_offset":15396,"inlinee":22067,"source_line":105,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17044,"parent_record_offset":15396,"ancestor_record_offset":15396,"inlinee":23092,"source_line":413,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":16716,"parent_record_offset":15396,"ancestor_record_offset":15396,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::segments
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":16744,"parent_record_offset":15396,"ancestor_record_offset":15396,"inlinee":14651,"source_line":114,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":401,"record_offset":15600,"parent_record_offset":15576,"ancestor_record_offset":15396,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":15396,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":401,"record_offset":15936,"parent_record_offset":15912,"ancestor_record_offset":15396,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":401,"record_offset":16020,"parent_record_offset":15996,"ancestor_record_offset":15396,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":401,"record_offset":15960,"parent_record_offset":15936,"ancestor_record_offset":15396,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":401,"record_offset":17360,"parent_record_offset":17256,"ancestor_record_offset":15396,"inlinee":21157,"source_line":36,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":401,"record_offset":17232,"parent_record_offset":17208,"ancestor_record_offset":15396,"inlinee":21155,"source_line":64,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":17208,"parent_record_offset":17184,"ancestor_record_offset":15396,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":17388,"parent_record_offset":17360,"ancestor_record_offset":15396,"inlinee":6183,"source_line":96,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":401,"record_offset":16944,"parent_record_offset":16800,"ancestor_record_offset":15396,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":401,"record_offset":16832,"parent_record_offset":16800,"ancestor_record_offset":15396,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":16800,"parent_record_offset":16772,"ancestor_record_offset":15396,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":17416,"parent_record_offset":17388,"ancestor_record_offset":15396,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":17256,"parent_record_offset":17232,"ancestor_record_offset":15396,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":401,"record_offset":16132,"parent_record_offset":16108,"ancestor_record_offset":15396,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17748,"parent_record_offset":17720,"ancestor_record_offset":15396,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17776,"parent_record_offset":17720,"ancestor_record_offset":15396,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17112,"parent_record_offset":17072,"ancestor_record_offset":15396,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":401,"record_offset":16684,"parent_record_offset":15660,"ancestor_record_offset":15396,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":401,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":15396,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":401,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":15396,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":16908,"parent_record_offset":16872,"ancestor_record_offset":15396,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":16872,"parent_record_offset":16832,"ancestor_record_offset":15396,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":401,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":15396,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::snapping::snapping_init::snapping_init::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/snapping/snapping_init.rs
//     evidence={"module":401,"record_offset":17444,"parent_record_offset":17416,"ancestor_record_offset":15396,"inlinee":23094,"source_line":53,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17144,"parent_record_offset":17072,"ancestor_record_offset":15396,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17900,"parent_record_offset":17876,"ancestor_record_offset":15396,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":15396,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17072,"parent_record_offset":17044,"ancestor_record_offset":15396,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":15396,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":401,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":15396,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":401,"record_offset":15712,"parent_record_offset":15688,"ancestor_record_offset":15396,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":401,"record_offset":15736,"parent_record_offset":15712,"ancestor_record_offset":15396,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":401,"record_offset":16420,"parent_record_offset":16200,"ancestor_record_offset":15396,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":401,"record_offset":15764,"parent_record_offset":15736,"ancestor_record_offset":15396,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":16332,"parent_record_offset":16308,"ancestor_record_offset":15396,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":16308,"parent_record_offset":16200,"ancestor_record_offset":15396,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":15396,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":16628,"parent_record_offset":15840,"ancestor_record_offset":15396,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":16224,"parent_record_offset":16200,"ancestor_record_offset":15396,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":15816,"parent_record_offset":15792,"ancestor_record_offset":15396,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":15792,"parent_record_offset":15736,"ancestor_record_offset":15396,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":15840,"parent_record_offset":15816,"ancestor_record_offset":15396,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":401,"record_offset":16080,"parent_record_offset":16052,"ancestor_record_offset":15396,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":401,"record_offset":16108,"parent_record_offset":16080,"ancestor_record_offset":15396,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":401,"record_offset":16168,"parent_record_offset":16052,"ancestor_record_offset":15396,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":401,"record_offset":16052,"parent_record_offset":15840,"ancestor_record_offset":15396,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":15884,"parent_record_offset":15840,"ancestor_record_offset":15396,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":401,"record_offset":16200,"parent_record_offset":15840,"ancestor_record_offset":15396,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":401,"record_offset":15912,"parent_record_offset":15840,"ancestor_record_offset":15396,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":401,"record_offset":15996,"parent_record_offset":15840,"ancestor_record_offset":15396,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":401,"record_offset":16536,"parent_record_offset":15840,"ancestor_record_offset":15396,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":401,"record_offset":15688,"parent_record_offset":15660,"ancestor_record_offset":15396,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":17956,"parent_record_offset":17924,"ancestor_record_offset":15396,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17472,"parent_record_offset":17444,"ancestor_record_offset":15396,"inlinee":23095,"source_line":161,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17612,"parent_record_offset":17528,"ancestor_record_offset":15396,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17876,"parent_record_offset":17472,"ancestor_record_offset":15396,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17672,"parent_record_offset":17500,"ancestor_record_offset":15396,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17500,"parent_record_offset":17472,"ancestor_record_offset":15396,"inlinee":23096,"source_line":176,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17528,"parent_record_offset":17500,"ancestor_record_offset":15396,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_init::snapping_init::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A36
// 首条位置: {"module":401,"inlinee":23094}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17472,"parent_record_offset":17444,"ancestor_record_offset":17444,"inlinee":23095,"source_line":161,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17748,"parent_record_offset":17720,"ancestor_record_offset":17444,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17776,"parent_record_offset":17720,"ancestor_record_offset":17444,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":17444,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17636,"parent_record_offset":17612,"ancestor_record_offset":17444,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17900,"parent_record_offset":17876,"ancestor_record_offset":17444,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":17444,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17556,"parent_record_offset":17528,"ancestor_record_offset":17444,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":17444,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":17956,"parent_record_offset":17924,"ancestor_record_offset":17444,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17612,"parent_record_offset":17528,"ancestor_record_offset":17444,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17876,"parent_record_offset":17472,"ancestor_record_offset":17444,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17672,"parent_record_offset":17500,"ancestor_record_offset":17444,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17500,"parent_record_offset":17472,"ancestor_record_offset":17444,"inlinee":23096,"source_line":176,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17528,"parent_record_offset":17500,"ancestor_record_offset":17444,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00401.jsonl.gz"}
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
// 完整结构及原始类型记录: snapping_init.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapping_init.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapping_init.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_init::snapping_init
// 信息 ID: 38263b7006718f27dc03df16
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f9ddaadb95b766eb3bde342a: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0xc9a68 -> 0x551e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> @ 0x3c2d16 -> 0x551e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_init::snapping_init::closure$0
// 信息 ID: 5309951b7b1ad462eeb82ba6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::snapping::snapping_init::snapping_init", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=401, inline_record=17444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
