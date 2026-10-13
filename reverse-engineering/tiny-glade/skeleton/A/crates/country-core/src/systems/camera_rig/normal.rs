// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/camera_rig/normal.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 93829f38e8af38f4fe47bb6c7b5b1728037231a99f4a02533ea0be900ea8f150
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: normal.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 391, 1202, 1308
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1202,"record_offset":31216,"rva":13171152,"va":5381880272}
// 编译实例 RVAs: 0xc8f9d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4937732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381880272,"raw_hex":"e8fca7b4ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5381881727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21713548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381880272,"raw_hex":"e88c524b01","size":5,"soft_fail":false,"target_va":5403594000,"va":5381880447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21713374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381880272,"raw_hex":"e8de514b01","size":5,"soft_fail":false,"target_va":5403594064,"va":5381880685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21713895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381880272,"raw_hex":"e8e7534b01","size":5,"soft_fail":false,"target_va":5403594336,"va":5381880436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1202,"record_offset":44664,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":6475,"source_line":1653,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":44636,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1202,"record_offset":44252,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1202,"record_offset":38584,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1202,"record_offset":38500,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":1202,"record_offset":38612,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":22231,"source_line":24,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::NormalCamera::rotation
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":31432,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":59385,"source_line":41,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":35640,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":59388,"source_line":213,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":35024,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":40412,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":14675,"source_line":875,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":32900,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":59386,"source_line":808,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":32812,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":33264,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":33176,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":44424,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":40232,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":38296,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1202,"record_offset":41292,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":32784,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1202,"record_offset":44744,"parent_record_offset":44696,"ancestor_record_offset":31216,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1202,"record_offset":44696,"parent_record_offset":44664,"ancestor_record_offset":31216,"inlinee":6476,"source_line":1679,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1202,"record_offset":45428,"parent_record_offset":45404,"ancestor_record_offset":31216,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1202,"record_offset":44800,"parent_record_offset":44772,"ancestor_record_offset":31216,"inlinee":6480,"source_line":1657,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1202,"record_offset":45404,"parent_record_offset":45380,"ancestor_record_offset":31216,"inlinee":6487,"source_line":1707,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1202,"record_offset":44772,"parent_record_offset":44696,"ancestor_record_offset":31216,"inlinee":6478,"source_line":1727,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1202,"record_offset":40460,"parent_record_offset":40436,"ancestor_record_offset":31216,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":32168,"parent_record_offset":31888,"ancestor_record_offset":31216,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":35372,"parent_record_offset":35316,"ancestor_record_offset":31216,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":34972,"parent_record_offset":34740,"ancestor_record_offset":31216,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=97
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":32000,"parent_record_offset":31888,"ancestor_record_offset":31216,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":31944,"parent_record_offset":31888,"ancestor_record_offset":31216,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":34944,"parent_record_offset":34740,"ancestor_record_offset":31216,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":35236,"parent_record_offset":35048,"ancestor_record_offset":31216,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":35804,"parent_record_offset":35684,"ancestor_record_offset":31216,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":36352,"parent_record_offset":36296,"ancestor_record_offset":31216,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":31484,"parent_record_offset":31456,"ancestor_record_offset":31216,"inlinee":22232,"source_line":903,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1202,"record_offset":45088,"parent_record_offset":45040,"ancestor_record_offset":31216,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1202,"record_offset":44276,"parent_record_offset":44252,"ancestor_record_offset":31216,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1202,"record_offset":45356,"parent_record_offset":44696,"ancestor_record_offset":31216,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":33112,"parent_record_offset":32988,"ancestor_record_offset":31216,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":32840,"parent_record_offset":32812,"ancestor_record_offset":31216,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":33056,"parent_record_offset":32988,"ancestor_record_offset":31216,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":35768,"parent_record_offset":35740,"ancestor_record_offset":31216,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":33292,"parent_record_offset":33264,"ancestor_record_offset":31216,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":44452,"parent_record_offset":44424,"ancestor_record_offset":31216,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":33204,"parent_record_offset":33176,"ancestor_record_offset":31216,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1202,"record_offset":44300,"parent_record_offset":44276,"ancestor_record_offset":31216,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1202,"record_offset":45380,"parent_record_offset":45356,"ancestor_record_offset":31216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1202,"record_offset":38548,"parent_record_offset":38524,"ancestor_record_offset":31216,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1202,"record_offset":38524,"parent_record_offset":38500,"ancestor_record_offset":31216,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":1202,"record_offset":31456,"parent_record_offset":31432,"ancestor_record_offset":31216,"inlinee":22231,"source_line":24,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_circle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":36004,"parent_record_offset":35972,"ancestor_record_offset":31216,"inlinee":22246,"source_line":747,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_glade_2d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":35972,"parent_record_offset":35684,"ancestor_record_offset":31216,"inlinee":22320,"source_line":739,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":36244,"parent_record_offset":35972,"ancestor_record_offset":31216,"inlinee":22248,"source_line":751,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::ActiveRotation::is_finished
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":44848,"parent_record_offset":44824,"ancestor_record_offset":31216,"inlinee":59394,"source_line":204,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::constrain_position_normal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":37056,"parent_record_offset":35640,"ancestor_record_offset":31216,"inlinee":59390,"source_line":226,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::constrain_position_semi_ortho
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":35684,"parent_record_offset":35640,"ancestor_record_offset":31216,"inlinee":59389,"source_line":245,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::normal::impl$1::update::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":44824,"parent_record_offset":44800,"ancestor_record_offset":31216,"inlinee":59392,"source_line":188,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::euler::impl$10::new_quat
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/euler.rs
//     evidence={"module":1202,"record_offset":31564,"parent_record_offset":31540,"ancestor_record_offset":31216,"inlinee":22235,"source_line":252,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::euler::impl$10::new_quat::rot_y
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/euler.rs
//     evidence={"module":1202,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31216,"inlinee":22237,"source_line":259,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1202,"record_offset":31648,"parent_record_offset":31624,"ancestor_record_offset":31216,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1202,"record_offset":32988,"parent_record_offset":32900,"ancestor_record_offset":31216,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31540,"parent_record_offset":31456,"ancestor_record_offset":31216,"inlinee":22233,"source_line":185,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_x
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31740,"parent_record_offset":31588,"ancestor_record_offset":31216,"inlinee":22238,"source_line":161,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31624,"parent_record_offset":31588,"ancestor_record_offset":31216,"inlinee":21673,"source_line":169,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31856,"parent_record_offset":31740,"ancestor_record_offset":31216,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31888,"parent_record_offset":31588,"ancestor_record_offset":31216,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":40436,"parent_record_offset":40412,"ancestor_record_offset":31216,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":35048,"parent_record_offset":35024,"ancestor_record_offset":31216,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":34716,"parent_record_offset":33496,"ancestor_record_offset":31216,"inlinee":22240,"source_line":485,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1202,"record_offset":35492,"parent_record_offset":35048,"ancestor_record_offset":31216,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1202,"record_offset":40508,"parent_record_offset":40484,"ancestor_record_offset":31216,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1202,"record_offset":40484,"parent_record_offset":40460,"ancestor_record_offset":31216,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1202,"record_offset":34740,"parent_record_offset":34716,"ancestor_record_offset":31216,"inlinee":22241,"source_line":522,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36272,"parent_record_offset":36244,"ancestor_record_offset":31216,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36028,"parent_record_offset":36004,"ancestor_record_offset":31216,"inlinee":22247,"source_line":741,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36080,"parent_record_offset":36056,"ancestor_record_offset":31216,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36056,"parent_record_offset":36028,"ancestor_record_offset":31216,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":31216,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36656,"parent_record_offset":36244,"ancestor_record_offset":31216,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35916,"parent_record_offset":35684,"ancestor_record_offset":31216,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35740,"parent_record_offset":35684,"ancestor_record_offset":31216,"inlinee":48695,"source_line":1205,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35860,"parent_record_offset":35684,"ancestor_record_offset":31216,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":36776,"parent_record_offset":35684,"ancestor_record_offset":31216,"inlinee":59386,"source_line":808,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":32956,"parent_record_offset":32932,"ancestor_record_offset":31216,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":32932,"parent_record_offset":32900,"ancestor_record_offset":31216,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":37276,"parent_record_offset":37056,"ancestor_record_offset":31216,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":38112,"parent_record_offset":37056,"ancestor_record_offset":31216,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":37100,"parent_record_offset":37056,"ancestor_record_offset":31216,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1202,"record_offset":35120,"parent_record_offset":35092,"ancestor_record_offset":31216,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1202,"record_offset":35092,"parent_record_offset":35048,"ancestor_record_offset":31216,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1202,"record_offset":34796,"parent_record_offset":34768,"ancestor_record_offset":31216,"inlinee":22243,"source_line":52,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1202,"record_offset":34768,"parent_record_offset":34740,"ancestor_record_offset":31216,"inlinee":22242,"source_line":77,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":44884,"parent_record_offset":44848,"ancestor_record_offset":31216,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":31696,"parent_record_offset":31672,"ancestor_record_offset":31216,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":31672,"parent_record_offset":31648,"ancestor_record_offset":31216,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":33028,"parent_record_offset":32988,"ancestor_record_offset":31216,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x56CC
// 首条位置: {"module":391,"inlinee":22220}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::NormalCamera::rotation
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE7F9
// 首条位置: {"module":1202,"inlinee":59385}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::camera_body::pitch_yaw_rot
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/camera_body.rs
//     evidence={"module":1202,"record_offset":31456,"parent_record_offset":31432,"ancestor_record_offset":31432,"inlinee":22231,"source_line":24,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":32168,"parent_record_offset":31888,"ancestor_record_offset":31432,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":34972,"parent_record_offset":34740,"ancestor_record_offset":33352,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":32000,"parent_record_offset":31888,"ancestor_record_offset":31432,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":31944,"parent_record_offset":31888,"ancestor_record_offset":31432,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":34944,"parent_record_offset":34740,"ancestor_record_offset":33352,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":31484,"parent_record_offset":31456,"ancestor_record_offset":31432,"inlinee":22232,"source_line":903,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::euler::impl$10::new_quat
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/euler.rs
//     evidence={"module":1202,"record_offset":31564,"parent_record_offset":31540,"ancestor_record_offset":31432,"inlinee":22235,"source_line":252,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::euler::impl$10::new_quat::rot_y
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/euler.rs
//     evidence={"module":1202,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31432,"inlinee":22237,"source_line":259,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1202,"record_offset":31648,"parent_record_offset":31624,"ancestor_record_offset":31432,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31540,"parent_record_offset":31456,"ancestor_record_offset":31432,"inlinee":22233,"source_line":185,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31740,"parent_record_offset":31588,"ancestor_record_offset":31432,"inlinee":22238,"source_line":161,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31624,"parent_record_offset":31588,"ancestor_record_offset":31432,"inlinee":21673,"source_line":169,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31856,"parent_record_offset":31740,"ancestor_record_offset":31432,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":31888,"parent_record_offset":31588,"ancestor_record_offset":31432,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":34716,"parent_record_offset":33496,"ancestor_record_offset":33352,"inlinee":22240,"source_line":485,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1202,"record_offset":34740,"parent_record_offset":34716,"ancestor_record_offset":33352,"inlinee":22241,"source_line":522,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1202,"record_offset":34796,"parent_record_offset":34768,"ancestor_record_offset":33352,"inlinee":22243,"source_line":52,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1202,"record_offset":34768,"parent_record_offset":34740,"ancestor_record_offset":33352,"inlinee":22242,"source_line":77,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":31696,"parent_record_offset":31672,"ancestor_record_offset":31432,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":31672,"parent_record_offset":31648,"ancestor_record_offset":31432,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1202,"record_offset":27388,"rva":13169744,"va":5381878864}
// 编译实例 RVAs: 0xc8f450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14069f3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6226190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381878864,"raw_hex":"e8f2fea0ff","size":5,"soft_fail":false,"target_va":5375652784,"va":5381878969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":27812,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":27784,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1202,"record_offset":27672,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1202,"record_offset":27756,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1202,"record_offset":27644,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1202,"record_offset":27560,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1202,"record_offset":27720,"parent_record_offset":27696,"ancestor_record_offset":27388,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1202,"record_offset":27696,"parent_record_offset":27672,"ancestor_record_offset":27388,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1202,"record_offset":27608,"parent_record_offset":27584,"ancestor_record_offset":27388,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1202,"record_offset":27584,"parent_record_offset":27560,"ancestor_record_offset":27388,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::constrain_to_be_above_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1202,"record_offset":27844,"rva":13169984,"va":5381879104}
// 编译实例 RVAs: 0xc8f540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4939790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381879104,"raw_hex":"e8f29fb4ff","size":5,"soft_fail":false,"target_va":5376939952,"va":5381879737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":29740,"parent_record_offset":27844,"ancestor_record_offset":27844,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1202,"record_offset":28112,"parent_record_offset":27844,"ancestor_record_offset":27844,"inlinee":22206,"source_line":61,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":28020,"parent_record_offset":27844,"ancestor_record_offset":27844,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":29648,"parent_record_offset":27844,"ancestor_record_offset":27844,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1202,"record_offset":28836,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":28432,"parent_record_offset":28404,"ancestor_record_offset":27844,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":28460,"parent_record_offset":28404,"ancestor_record_offset":27844,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":28520,"parent_record_offset":28496,"ancestor_record_offset":27844,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":28052,"parent_record_offset":28020,"ancestor_record_offset":27844,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":27844,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":28712,"parent_record_offset":28688,"ancestor_record_offset":27844,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1202,"record_offset":28656,"parent_record_offset":28612,"ancestor_record_offset":27844,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$3::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1202,"record_offset":28144,"parent_record_offset":28112,"ancestor_record_offset":27844,"inlinee":22208,"source_line":175,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":28552,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":28380,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":28404,"parent_record_offset":28380,"ancestor_record_offset":27844,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":28324,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":29544,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":28268,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":28496,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":1202,"record_offset":28580,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":1202,"record_offset":28860,"parent_record_offset":28836,"ancestor_record_offset":27844,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1202,"record_offset":28688,"parent_record_offset":28656,"ancestor_record_offset":27844,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":1202,"record_offset":28612,"parent_record_offset":28176,"ancestor_record_offset":27844,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":1202,"record_offset":28176,"parent_record_offset":28144,"ancestor_record_offset":27844,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::translate_screen_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1202,"record_offset":29772,"rva":13170784,"va":5381879904}
// 编译实例 RVAs: 0xc8f860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21713959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381879904,"raw_hex":"e827544b01","size":5,"soft_fail":false,"target_va":5403594000,"va":5381880036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: normal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21714306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381879904,"raw_hex":"e882554b01","size":5,"soft_fail":false,"target_va":5403594336,"va":5381880025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_direction
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1281514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381879904,"raw_hex":"e8ea8d1300","size":5,"soft_fail":false,"target_va":5383161488,"va":5381879969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1281470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381879904,"raw_hex":"e8be8d1300","size":5,"soft_fail":false,"target_va":5383161424,"va":5381879949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":29952,"parent_record_offset":29772,"ancestor_record_offset":29772,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":29980,"parent_record_offset":29772,"ancestor_record_offset":29772,"inlinee":22232,"source_line":903,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":30008,"parent_record_offset":29772,"ancestor_record_offset":29772,"inlinee":21673,"source_line":169,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":30156,"parent_record_offset":29772,"ancestor_record_offset":29772,"inlinee":14675,"source_line":875,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":31124,"parent_record_offset":29772,"ancestor_record_offset":29772,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":31036,"parent_record_offset":29772,"ancestor_record_offset":29772,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1202,"record_offset":30204,"parent_record_offset":30180,"ancestor_record_offset":29772,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":30704,"parent_record_offset":30288,"ancestor_record_offset":29772,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":30612,"parent_record_offset":30556,"ancestor_record_offset":29772,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":30384,"parent_record_offset":30360,"ancestor_record_offset":29772,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":30416,"parent_record_offset":30332,"ancestor_record_offset":29772,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1202,"record_offset":30476,"parent_record_offset":30288,"ancestor_record_offset":29772,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":31064,"parent_record_offset":31036,"ancestor_record_offset":29772,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":31152,"parent_record_offset":31124,"ancestor_record_offset":29772,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1202,"record_offset":30036,"parent_record_offset":30008,"ancestor_record_offset":29772,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":30124,"parent_record_offset":30008,"ancestor_record_offset":29772,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":30180,"parent_record_offset":30156,"ancestor_record_offset":29772,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1202,"record_offset":30288,"parent_record_offset":30180,"ancestor_record_offset":29772,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1202,"record_offset":30732,"parent_record_offset":30288,"ancestor_record_offset":29772,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1202,"record_offset":30252,"parent_record_offset":30228,"ancestor_record_offset":29772,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1202,"record_offset":30228,"parent_record_offset":30204,"ancestor_record_offset":29772,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1202,"record_offset":30360,"parent_record_offset":30332,"ancestor_record_offset":29772,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1202,"record_offset":30332,"parent_record_offset":30288,"ancestor_record_offset":29772,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":30084,"parent_record_offset":30060,"ancestor_record_offset":29772,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":30060,"parent_record_offset":30036,"ancestor_record_offset":29772,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::impl$1::update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE800
// 首条位置: {"module":1202,"inlinee":59392}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::ActiveRotation::is_finished
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":44848,"parent_record_offset":44824,"ancestor_record_offset":44824,"inlinee":59394,"source_line":204,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":44996,"parent_record_offset":44848,"ancestor_record_offset":44824,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":44884,"parent_record_offset":44848,"ancestor_record_offset":44824,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::ActiveRotation::is_finished
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE802
// 首条位置: {"module":1202,"inlinee":59394}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":44996,"parent_record_offset":44848,"ancestor_record_offset":44848,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":44884,"parent_record_offset":44848,"ancestor_record_offset":44848,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE7FC
// 首条位置: {"module":1202,"inlinee":59388}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position_normal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":37056,"parent_record_offset":35640,"ancestor_record_offset":35640,"inlinee":59390,"source_line":226,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::normal::constrain_position_semi_ortho
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/normal.rs
//     evidence={"module":1202,"record_offset":35684,"parent_record_offset":35640,"ancestor_record_offset":35640,"inlinee":59389,"source_line":245,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":35804,"parent_record_offset":35684,"ancestor_record_offset":35640,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":36352,"parent_record_offset":36296,"ancestor_record_offset":35640,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":35884,"parent_record_offset":35860,"ancestor_record_offset":35640,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":36680,"parent_record_offset":36656,"ancestor_record_offset":35640,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":36176,"parent_record_offset":36112,"ancestor_record_offset":35640,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":35768,"parent_record_offset":35740,"ancestor_record_offset":35640,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":35940,"parent_record_offset":35916,"ancestor_record_offset":35640,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_circle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":36004,"parent_record_offset":35972,"ancestor_record_offset":35640,"inlinee":22246,"source_line":747,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_glade_2d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":35972,"parent_record_offset":35684,"ancestor_record_offset":35640,"inlinee":22320,"source_line":739,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":36244,"parent_record_offset":35972,"ancestor_record_offset":35640,"inlinee":22248,"source_line":751,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1202,"record_offset":36112,"parent_record_offset":36028,"ancestor_record_offset":35640,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36272,"parent_record_offset":36244,"ancestor_record_offset":35640,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36028,"parent_record_offset":36004,"ancestor_record_offset":35640,"inlinee":22247,"source_line":741,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36080,"parent_record_offset":36056,"ancestor_record_offset":35640,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36056,"parent_record_offset":36028,"ancestor_record_offset":35640,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":35640,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36656,"parent_record_offset":36244,"ancestor_record_offset":35640,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35916,"parent_record_offset":35684,"ancestor_record_offset":35640,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35740,"parent_record_offset":35684,"ancestor_record_offset":35640,"inlinee":48695,"source_line":1205,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35860,"parent_record_offset":35684,"ancestor_record_offset":35640,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":36776,"parent_record_offset":35684,"ancestor_record_offset":35640,"inlinee":59386,"source_line":808,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":36832,"parent_record_offset":36808,"ancestor_record_offset":35640,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":36808,"parent_record_offset":36776,"ancestor_record_offset":35640,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":37276,"parent_record_offset":37056,"ancestor_record_offset":35640,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":38112,"parent_record_offset":37056,"ancestor_record_offset":35640,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":37100,"parent_record_offset":37056,"ancestor_record_offset":35640,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":36148,"parent_record_offset":36112,"ancestor_record_offset":35640,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position_normal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE7FE
// 首条位置: {"module":1202,"inlinee":59390}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::camera_rig::clamp_to_glade_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":37364,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":22320,"source_line":739,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":37276,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":38112,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":37100,"parent_record_offset":37056,"ancestor_record_offset":37056,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":37716,"parent_record_offset":37688,"ancestor_record_offset":37056,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":37744,"parent_record_offset":37688,"ancestor_record_offset":37056,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":37128,"parent_record_offset":37100,"ancestor_record_offset":37056,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":37304,"parent_record_offset":37276,"ancestor_record_offset":37056,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":37568,"parent_record_offset":37504,"ancestor_record_offset":37056,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":37804,"parent_record_offset":37780,"ancestor_record_offset":37056,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":37396,"parent_record_offset":37364,"ancestor_record_offset":37056,"inlinee":22246,"source_line":747,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":37636,"parent_record_offset":37364,"ancestor_record_offset":37056,"inlinee":22248,"source_line":751,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1202,"record_offset":37504,"parent_record_offset":37420,"ancestor_record_offset":37056,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":37664,"parent_record_offset":37636,"ancestor_record_offset":37056,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":37420,"parent_record_offset":37396,"ancestor_record_offset":37056,"inlinee":22247,"source_line":741,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":37472,"parent_record_offset":37448,"ancestor_record_offset":37056,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":37448,"parent_record_offset":37420,"ancestor_record_offset":37056,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":37688,"parent_record_offset":37664,"ancestor_record_offset":37056,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":38048,"parent_record_offset":37636,"ancestor_record_offset":37056,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":37780,"parent_record_offset":37636,"ancestor_record_offset":37056,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":37540,"parent_record_offset":37504,"ancestor_record_offset":37056,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::normal::constrain_position_semi_ortho
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE7FD
// 首条位置: {"module":1202,"inlinee":59389}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":35804,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::clamp_to_glade_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":35972,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":22320,"source_line":739,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36720,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35916,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35740,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":48695,"source_line":1205,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":35860,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":36776,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":59386,"source_line":808,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":36324,"parent_record_offset":36296,"ancestor_record_offset":35684,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1202,"record_offset":36352,"parent_record_offset":36296,"ancestor_record_offset":35684,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":35884,"parent_record_offset":35860,"ancestor_record_offset":35684,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":36680,"parent_record_offset":36656,"ancestor_record_offset":35684,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":36176,"parent_record_offset":36112,"ancestor_record_offset":35684,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":35768,"parent_record_offset":35740,"ancestor_record_offset":35684,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1202,"record_offset":35940,"parent_record_offset":35916,"ancestor_record_offset":35684,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":36004,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":22246,"source_line":747,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":1202,"record_offset":36244,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":22248,"source_line":751,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1202,"record_offset":36112,"parent_record_offset":36028,"ancestor_record_offset":35684,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36272,"parent_record_offset":36244,"ancestor_record_offset":35684,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36028,"parent_record_offset":36004,"ancestor_record_offset":35684,"inlinee":22247,"source_line":741,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36080,"parent_record_offset":36056,"ancestor_record_offset":35684,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36056,"parent_record_offset":36028,"ancestor_record_offset":35684,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":35684,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36656,"parent_record_offset":36244,"ancestor_record_offset":35684,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1202,"record_offset":36388,"parent_record_offset":36244,"ancestor_record_offset":35684,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":36832,"parent_record_offset":36808,"ancestor_record_offset":35684,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1202,"record_offset":36808,"parent_record_offset":36776,"ancestor_record_offset":35684,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1202,"record_offset":36148,"parent_record_offset":36112,"ancestor_record_offset":35684,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01202.jsonl.gz"}
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
// 完整结构及原始类型记录: normal.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::normal::ActiveRotation
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::systems::camera_rig::normal::ActiveRotation"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::systems::camera_rig::normal::ActiveRotation>", "constructor": "ref$", "arguments": ["country_core::systems::camera_rig::normal::ActiveRotation"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >", "constructor": "core::option::IntoIter", "arguments": ["ref$<country_core::systems::camera_rig::normal::ActiveRotation>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >", "country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> >", "constructor": "core::iter::traits::accum::impl$100::sum::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["ref$<country_core::systems::camera_rig::normal::ActiveRotation>", "f32", "f32", "country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0", "core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::fold<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >,f32,core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > > >", "constructor": "core::iter::adapters::chain::impl$1::fold", "arguments": ["core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>", "core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> >", "f32", "core::iter::adapters::map::map_fold::closure_env$0<ref$<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0,core::iter::traits::accum::impl$100::sum::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref$<country_core::systems::camera_rig::normal::ActiveRotation> > >,country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::systems::camera_rig::normal::ActiveRotation,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::systems::camera_rig::normal::ActiveRotation", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::normal::NormalCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::camera_rig::normal::NormalCamera>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::camera_rig::normal::NormalCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: normal.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: normal.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::ActiveRotation::is_finished
// 信息 ID: f8b551ab956e63d63155aef8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=44848; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=45164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::NormalCamera::rotation
// 信息 ID: c16e23e8e05a41088cbd6e11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=31432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=33352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=45476; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=46828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position
// 信息 ID: 2ff9cf78f1b4a40f5c538440
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=35640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position_normal
// 信息 ID: 9ade3484ca70cc4d742dd261
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=37056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::constrain_position_semi_ortho
// 信息 ID: 9e1146e7f03d26facd91e337
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=35684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::constrain_to_be_above_terrain
// 信息 ID: 7be452e922bbae1c9e0d5d2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch
// 信息 ID: f86d70825275a65af8d4d149
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::translate_screen_space
// 信息 ID: ca4fbb81e7ae10088ee95e4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::update
// 信息 ID: 49925c326ce29baf4c5222a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$1::update::closure$0
// 信息 ID: 9fd1d4bad32b4269a6d26548
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::camera_rig::normal::impl$1::update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=44824; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=45140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::normal::impl$3::default
// 信息 ID: 12a8ee5ae7644eeee2b28bb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::MainCamera::new; module=391, inline_record=8728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::MainCamera::new; module=391, inline_record=10668; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::MainCamera::new; module=391, inline_record=10724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
