// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/camera_rig/free.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d465b1be056bfb38d43179d748589570e0b52e924c698f59984cf1ed5c4b4f17
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: free.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 438
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use dolly; // namespace candidate
//   package identities: [{"name":"dolly","source":"PDB_registry_path_only","version":"0.6.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::translate_screen_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":13988,"rva":6172592,"va":5374881712}
// 编译实例 RVAs: 0x5e2fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: free.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28711966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374881712,"raw_hex":"e81e1cb601","size":5,"soft_fail":false,"target_va":5403594064,"va":5374882093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28095700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374881712,"raw_hex":"e8d4b4ac01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374883239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2057273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374881712,"raw_hex":"e839641f00","size":5,"soft_fail":false,"target_va":5376939952,"va":5374882674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*72(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374881712,"raw_hex":"ff5048","size":3,"soft_fail":false,"va":5374882036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":438,"record_offset":15488,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":438,"record_offset":18128,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":438,"record_offset":15796,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":22206,"source_line":61,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::position::Position::translate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":438,"record_offset":18404,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":22798,"source_line":34,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":15072,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":22274,"source_line":48,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":17604,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":22229,"source_line":31,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":14192,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":14675,"source_line":875,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":15396,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":15704,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":15520,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":18312,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":15612,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":438,"record_offset":15368,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":17832,"parent_record_offset":17808,"ancestor_record_offset":13988,"inlinee":8015,"source_line":263,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":15296,"parent_record_offset":15272,"ancestor_record_offset":13988,"inlinee":5890,"source_line":227,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":15320,"parent_record_offset":15296,"ancestor_record_offset":13988,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":17968,"parent_record_offset":17924,"ancestor_record_offset":13988,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":438,"record_offset":14240,"parent_record_offset":14216,"ancestor_record_offset":13988,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":14740,"parent_record_offset":14324,"ancestor_record_offset":13988,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":14648,"parent_record_offset":14592,"ancestor_record_offset":13988,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":13988,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":14452,"parent_record_offset":14368,"ancestor_record_offset":13988,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":438,"record_offset":14512,"parent_record_offset":14324,"ancestor_record_offset":13988,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":438,"record_offset":16108,"parent_record_offset":16080,"ancestor_record_offset":13988,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":438,"record_offset":16136,"parent_record_offset":16080,"ancestor_record_offset":13988,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":438,"record_offset":17996,"parent_record_offset":17924,"ancestor_record_offset":13988,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":15644,"parent_record_offset":15612,"ancestor_record_offset":13988,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":15428,"parent_record_offset":15396,"ancestor_record_offset":13988,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":15968,"parent_record_offset":15944,"ancestor_record_offset":13988,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":15736,"parent_record_offset":15704,"ancestor_record_offset":13988,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":438,"record_offset":15552,"parent_record_offset":15520,"ancestor_record_offset":13988,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":438,"record_offset":17900,"parent_record_offset":17604,"ancestor_record_offset":13988,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":15236,"parent_record_offset":15212,"ancestor_record_offset":13988,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":15184,"parent_record_offset":15148,"ancestor_record_offset":13988,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":15212,"parent_record_offset":15148,"ancestor_record_offset":13988,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":17748,"parent_record_offset":17684,"ancestor_record_offset":13988,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":13988,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":15148,"parent_record_offset":15120,"ancestor_record_offset":13988,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":17656,"parent_record_offset":17632,"ancestor_record_offset":13988,"inlinee":22195,"source_line":313,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":17684,"parent_record_offset":17656,"ancestor_record_offset":13988,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":438,"record_offset":16376,"parent_record_offset":16340,"ancestor_record_offset":13988,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$3::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":438,"record_offset":15828,"parent_record_offset":15796,"ancestor_record_offset":13988,"inlinee":22208,"source_line":175,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":13988,"inlinee":22200,"source_line":58,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":17632,"parent_record_offset":17604,"ancestor_record_offset":13988,"inlinee":22194,"source_line":41,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":18580,"parent_record_offset":18556,"ancestor_record_offset":13988,"inlinee":22204,"source_line":49,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":17924,"parent_record_offset":17900,"ancestor_record_offset":13988,"inlinee":22199,"source_line":32,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":15272,"parent_record_offset":15120,"ancestor_record_offset":13988,"inlinee":22202,"source_line":61,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":17808,"parent_record_offset":17656,"ancestor_record_offset":13988,"inlinee":22197,"source_line":44,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":14216,"parent_record_offset":14192,"ancestor_record_offset":13988,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":438,"record_offset":14324,"parent_record_offset":14216,"ancestor_record_offset":13988,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":438,"record_offset":14768,"parent_record_offset":14324,"ancestor_record_offset":13988,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":438,"record_offset":14288,"parent_record_offset":14264,"ancestor_record_offset":13988,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":438,"record_offset":14264,"parent_record_offset":14240,"ancestor_record_offset":13988,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":16228,"parent_record_offset":15860,"ancestor_record_offset":13988,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":16056,"parent_record_offset":15860,"ancestor_record_offset":13988,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":13988,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":16000,"parent_record_offset":15860,"ancestor_record_offset":13988,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":17208,"parent_record_offset":15860,"ancestor_record_offset":13988,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":15944,"parent_record_offset":15860,"ancestor_record_offset":13988,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":438,"record_offset":16172,"parent_record_offset":15860,"ancestor_record_offset":13988,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":18436,"parent_record_offset":18404,"ancestor_record_offset":13988,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":438,"record_offset":14396,"parent_record_offset":14368,"ancestor_record_offset":13988,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":438,"record_offset":14368,"parent_record_offset":14324,"ancestor_record_offset":13988,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":438,"record_offset":16312,"parent_record_offset":15860,"ancestor_record_offset":13988,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":438,"record_offset":16280,"parent_record_offset":16256,"ancestor_record_offset":13988,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":438,"record_offset":16408,"parent_record_offset":16376,"ancestor_record_offset":13988,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":438,"record_offset":16340,"parent_record_offset":15860,"ancestor_record_offset":13988,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":438,"record_offset":15860,"parent_record_offset":15828,"ancestor_record_offset":13988,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::FreeCamera::get_position
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":11440,"rva":6171184,"va":5374880304}
// 编译实例 RVAs: 0x5e2a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28098392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374880304,"raw_hex":"e858bfac01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374880547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*72(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374880304,"raw_hex":"ff5048","size":3,"soft_fail":false,"va":5374880368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":438,"record_offset":12024,"parent_record_offset":11440,"ancestor_record_offset":11440,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":11596,"parent_record_offset":11440,"ancestor_record_offset":11440,"inlinee":22274,"source_line":48,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":11820,"parent_record_offset":11800,"ancestor_record_offset":11440,"inlinee":5890,"source_line":227,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":11844,"parent_record_offset":11820,"ancestor_record_offset":11440,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":11956,"parent_record_offset":11912,"ancestor_record_offset":11440,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":438,"record_offset":11984,"parent_record_offset":11912,"ancestor_record_offset":11440,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":438,"record_offset":11888,"parent_record_offset":11596,"ancestor_record_offset":11440,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":11764,"parent_record_offset":11740,"ancestor_record_offset":11440,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":11712,"parent_record_offset":11676,"ancestor_record_offset":11440,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":11740,"parent_record_offset":11676,"ancestor_record_offset":11440,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":11648,"parent_record_offset":11624,"ancestor_record_offset":11440,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":11676,"parent_record_offset":11648,"ancestor_record_offset":11440,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":11624,"parent_record_offset":11596,"ancestor_record_offset":11440,"inlinee":22200,"source_line":58,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":11912,"parent_record_offset":11888,"ancestor_record_offset":11440,"inlinee":22204,"source_line":49,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":11800,"parent_record_offset":11648,"ancestor_record_offset":11440,"inlinee":22202,"source_line":61,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":438,"record_offset":12072,"parent_record_offset":12048,"ancestor_record_offset":11440,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$33::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_mint.rs
//     evidence={"module":438,"record_offset":12048,"parent_record_offset":12024,"ancestor_record_offset":11440,"inlinee":25308,"source_line":40,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::FreeCamera::get_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":12112,"rva":6171440,"va":5374880560}
// 编译实例 RVAs: 0x5e2b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28098150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374880560,"raw_hex":"e866beac01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374880789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*72(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374880560,"raw_hex":"ff5048","size":3,"soft_fail":false,"va":5374880624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":12264,"parent_record_offset":12112,"ancestor_record_offset":12112,"inlinee":22274,"source_line":48,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":12484,"parent_record_offset":12464,"ancestor_record_offset":12112,"inlinee":5890,"source_line":227,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":12508,"parent_record_offset":12484,"ancestor_record_offset":12112,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":12616,"parent_record_offset":12576,"ancestor_record_offset":12112,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":438,"record_offset":12644,"parent_record_offset":12576,"ancestor_record_offset":12112,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":438,"record_offset":12552,"parent_record_offset":12264,"ancestor_record_offset":12112,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":12112,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":12376,"parent_record_offset":12340,"ancestor_record_offset":12112,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":12404,"parent_record_offset":12340,"ancestor_record_offset":12112,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":12312,"parent_record_offset":12288,"ancestor_record_offset":12112,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":12340,"parent_record_offset":12312,"ancestor_record_offset":12112,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":12288,"parent_record_offset":12264,"ancestor_record_offset":12112,"inlinee":22200,"source_line":58,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":12576,"parent_record_offset":12552,"ancestor_record_offset":12112,"inlinee":22204,"source_line":49,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":12464,"parent_record_offset":12312,"ancestor_record_offset":12112,"inlinee":22202,"source_line":61,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::FreeCamera::set
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":12688,"rva":6171680,"va":5374880800}
// 编译实例 RVAs: 0x5e2c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=2
//     target_aliases=31 candidates; complete list: free.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2645531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374880800,"raw_hex":"e8e5a1d7ff","size":5,"soft_fail":false,"target_va":5372235632,"va":5374881158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140515930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: free.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-840573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374880800,"raw_hex":"e8832cf3ff","size":5,"soft_fail":false,"target_va":5374040368,"va":5374880936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140515a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: free.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-840287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374880800,"raw_hex":"e8a12df3ff","size":5,"soft_fail":false,"target_va":5374040704,"va":5374880986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140515bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: free.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-839912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374880800,"raw_hex":"e8182ff3ff","size":5,"soft_fail":false,"target_va":5374041008,"va":5374880915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374880800,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5374881072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":13188,"parent_record_offset":12688,"ancestor_record_offset":12688,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::position::Position::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":438,"record_offset":12872,"parent_record_offset":12688,"ancestor_record_offset":12688,"inlinee":22214,"source_line":24,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::smooth::Smooth::new_position_rotation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/drivers/smooth.rs
//     evidence={"module":438,"record_offset":12956,"parent_record_offset":12688,"ancestor_record_offset":12688,"inlinee":22216,"source_line":61,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::builder
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":12844,"parent_record_offset":12688,"ancestor_record_offset":12688,"inlinee":22212,"source_line":86,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRigBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":12984,"parent_record_offset":12688,"ancestor_record_offset":12688,"inlinee":22218,"source_line":105,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":438,"record_offset":12896,"parent_record_offset":12872,"ancestor_record_offset":12688,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":438,"record_offset":13156,"parent_record_offset":12984,"ancestor_record_offset":12688,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":13116,"parent_record_offset":13072,"ancestor_record_offset":12688,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":13072,"parent_record_offset":13028,"ancestor_record_offset":12688,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":13028,"parent_record_offset":12984,"ancestor_record_offset":12688,"inlinee":22205,"source_line":67,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$34::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_mint.rs
//     evidence={"module":438,"record_offset":12920,"parent_record_offset":12896,"ancestor_record_offset":12688,"inlinee":22719,"source_line":46,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":13248,"rva":6172288,"va":5374881408}
// 编译实例 RVAs: 0x5e2e80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28097234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374881408,"raw_hex":"e8d2baac01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374881705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] dolly::drivers::yaw_pitch::YawPitch::rotate_yaw_pitch
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t10984047","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374881408,"raw_hex":"e96f9aa700","size":5,"soft_fail":false,"target_va":5385865648,"va":5374881596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*80(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374881408,"raw_hex":"ff5050","size":3,"soft_fail":false,"va":5374881504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":13400,"parent_record_offset":13248,"ancestor_record_offset":13248,"inlinee":22229,"source_line":31,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":13624,"parent_record_offset":13604,"ancestor_record_offset":13248,"inlinee":8015,"source_line":263,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":13248,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":438,"record_offset":13760,"parent_record_offset":13716,"ancestor_record_offset":13248,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":438,"record_offset":13788,"parent_record_offset":13716,"ancestor_record_offset":13248,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":438,"record_offset":13692,"parent_record_offset":13400,"ancestor_record_offset":13248,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":13248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":13516,"parent_record_offset":13480,"ancestor_record_offset":13248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":13544,"parent_record_offset":13480,"ancestor_record_offset":13248,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":13248,"inlinee":22195,"source_line":313,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":13480,"parent_record_offset":13452,"ancestor_record_offset":13248,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":13428,"parent_record_offset":13400,"ancestor_record_offset":13248,"inlinee":22194,"source_line":41,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":13716,"parent_record_offset":13692,"ancestor_record_offset":13248,"inlinee":22199,"source_line":32,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":13604,"parent_record_offset":13452,"ancestor_record_offset":13248,"inlinee":22197,"source_line":44,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<std::thread::local::AccessError>
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":252,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::constrain_to_be_above_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":13832,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::free::impl$1::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":438,"record_offset":18696,"rva":6174240,"va":5374883360}
// 编译实例 RVAs: 0x5e3620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374883360,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5374883474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":438,"record_offset":18840,"parent_record_offset":18696,"ancestor_record_offset":18696,"inlinee":22205,"source_line":67,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_direct] dolly::transform::Transform::into_position_rotation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/transform.rs
//     evidence={"module":438,"record_offset":18968,"parent_record_offset":18696,"ancestor_record_offset":18696,"inlinee":24122,"source_line":31,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":438,"record_offset":18928,"parent_record_offset":18888,"ancestor_record_offset":18696,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":438,"record_offset":18888,"parent_record_offset":18840,"ancestor_record_offset":18696,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00438.jsonl.gz"}
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
// 完整结构及原始类型记录: free.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::free::FreeCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 3
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: free.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: free.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 115; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)
// 候选参数类型: ["bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >", "bevy_ecs::system::commands::Commands", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>,bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>", "bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>", "bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<std::thread::local::AccessError>
// 信息 ID: 656af1092ff58da1ab67d9a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// [类型关系] core::ptr::drop_in_place -> ["std::thread::local::AccessError"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::FreeCamera::get_position
// 信息 ID: 2656251f87a5cb2894decb6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_debug::do_freecam_save @ 0x34ed81 -> 0x5e2a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::FreeCamera::get_yaw_pitch
// 信息 ID: 940fa654b615a9417e37777e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_debug::do_freecam_save @ 0x34ed89 -> 0x5e2b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::FreeCamera::set
// 信息 ID: df954c812abfcaa953f78b9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_debug::do_freecam_save @ 0x34ee47 -> 0x5e2c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::constrain_to_be_above_terrain
// 信息 ID: b20327ae697b7b73ffc0780a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::rotate_yaw_pitch
// 信息 ID: 13b799b272bdef91009afd4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::translate_screen_space
// 信息 ID: 97055da29410038bc1683f6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::free::impl$1::update
// 信息 ID: 121326cb5ca25c42196f700f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
