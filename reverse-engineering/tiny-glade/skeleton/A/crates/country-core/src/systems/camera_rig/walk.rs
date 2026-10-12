// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/camera_rig/walk.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b9ecdd21a9a54695417521749ca25ebc39316512c0ec63f7f5d88660924e6f34
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: walk.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 452
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
// 函数符号: country_core::systems::camera_rig::walk::impl$1::translate_screen_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":452,"record_offset":29724,"rva":6396784,"va":5375105904}
// 编译实例 RVAs: 0x619b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1836628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375105904,"raw_hex":"e854061c00","size":5,"soft_fail":false,"target_va":5376944000,"va":5375107367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28486917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375105904,"raw_hex":"e805adb201","size":5,"soft_fail":false,"target_va":5403594064,"va":5375107142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27871077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375105904,"raw_hex":"e86547a901","size":5,"soft_fail":false,"target_va":5402978944,"va":5375107862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::Raycasting::spherecast
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-598503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375105904,"raw_hex":"e819def6ff","size":5,"soft_fail":false,"target_va":5374508224,"va":5375106722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*72(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375105904,"raw_hex":"ff5048","size":3,"soft_fail":false,"va":5375106500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":31836,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":452,"record_offset":32944,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::clamp_to_glade_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":452,"record_offset":33232,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":22320,"source_line":739,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::position::Position::translate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":452,"record_offset":35156,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":22798,"source_line":34,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":31360,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":22274,"source_line":48,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":34724,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":22229,"source_line":31,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":452,"record_offset":30016,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":14675,"source_line":875,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":31804,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":31100,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":29960,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":32224,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":30904,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::project_onto_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":32484,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":25936,"source_line":611,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$10::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":32132,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":25935,"source_line":1059,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":33116,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":32044,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":31748,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":32612,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":31296,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":31184,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":452,"record_offset":33204,"parent_record_offset":29724,"ancestor_record_offset":29724,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":29724,"inlinee":8015,"source_line":263,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":29724,"inlinee":5890,"source_line":227,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":31612,"parent_record_offset":31588,"ancestor_record_offset":29724,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":31708,"parent_record_offset":31680,"ancestor_record_offset":29724,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":452,"record_offset":32992,"parent_record_offset":32968,"ancestor_record_offset":29724,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":452,"record_offset":30064,"parent_record_offset":30040,"ancestor_record_offset":29724,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":452,"record_offset":30780,"parent_record_offset":30148,"ancestor_record_offset":29724,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":452,"record_offset":30688,"parent_record_offset":30632,"ancestor_record_offset":29724,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":452,"record_offset":30244,"parent_record_offset":30220,"ancestor_record_offset":29724,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":452,"record_offset":30276,"parent_record_offset":30192,"ancestor_record_offset":29724,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":452,"record_offset":30336,"parent_record_offset":30148,"ancestor_record_offset":29724,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":33588,"parent_record_offset":33560,"ancestor_record_offset":29724,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":33616,"parent_record_offset":33560,"ancestor_record_offset":29724,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":452,"record_offset":35116,"parent_record_offset":35044,"ancestor_record_offset":29724,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":452,"record_offset":32968,"parent_record_offset":32944,"ancestor_record_offset":29724,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":31208,"parent_record_offset":31184,"ancestor_record_offset":29724,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":33144,"parent_record_offset":33116,"ancestor_record_offset":29724,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":31328,"parent_record_offset":31296,"ancestor_record_offset":29724,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":32072,"parent_record_offset":32044,"ancestor_record_offset":29724,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":32640,"parent_record_offset":32612,"ancestor_record_offset":29724,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":32164,"parent_record_offset":32132,"ancestor_record_offset":29724,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":31772,"parent_record_offset":31748,"ancestor_record_offset":29724,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":31656,"parent_record_offset":31360,"ancestor_record_offset":29724,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":31528,"parent_record_offset":31504,"ancestor_record_offset":29724,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":31476,"parent_record_offset":31440,"ancestor_record_offset":29724,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":31504,"parent_record_offset":31440,"ancestor_record_offset":29724,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":34868,"parent_record_offset":34804,"ancestor_record_offset":29724,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":31412,"parent_record_offset":31388,"ancestor_record_offset":29724,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":31440,"parent_record_offset":31412,"ancestor_record_offset":29724,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":34776,"parent_record_offset":34752,"ancestor_record_offset":29724,"inlinee":22195,"source_line":313,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":34804,"parent_record_offset":34776,"ancestor_record_offset":29724,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_circle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":452,"record_offset":33264,"parent_record_offset":33232,"ancestor_record_offset":29724,"inlinee":22246,"source_line":747,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::clamp_to_rounded_box
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":452,"record_offset":33508,"parent_record_offset":33232,"ancestor_record_offset":29724,"inlinee":22248,"source_line":751,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":31388,"parent_record_offset":31360,"ancestor_record_offset":29724,"inlinee":22200,"source_line":58,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":34752,"parent_record_offset":34724,"ancestor_record_offset":29724,"inlinee":22194,"source_line":41,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":31680,"parent_record_offset":31656,"ancestor_record_offset":29724,"inlinee":22204,"source_line":49,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":35044,"parent_record_offset":35020,"ancestor_record_offset":29724,"inlinee":22199,"source_line":32,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":31564,"parent_record_offset":31412,"ancestor_record_offset":29724,"inlinee":22202,"source_line":61,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":34928,"parent_record_offset":34776,"ancestor_record_offset":29724,"inlinee":22197,"source_line":44,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":452,"record_offset":31124,"parent_record_offset":31100,"ancestor_record_offset":29724,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":452,"record_offset":30040,"parent_record_offset":30016,"ancestor_record_offset":29724,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":452,"record_offset":30148,"parent_record_offset":30040,"ancestor_record_offset":29724,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":452,"record_offset":30392,"parent_record_offset":30148,"ancestor_record_offset":29724,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":452,"record_offset":30112,"parent_record_offset":30088,"ancestor_record_offset":29724,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":452,"record_offset":30088,"parent_record_offset":30064,"ancestor_record_offset":29724,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":33536,"parent_record_offset":33508,"ancestor_record_offset":29724,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":33288,"parent_record_offset":33264,"ancestor_record_offset":29724,"inlinee":22247,"source_line":741,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":33344,"parent_record_offset":33320,"ancestor_record_offset":29724,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":33320,"parent_record_offset":33288,"ancestor_record_offset":29724,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":33560,"parent_record_offset":33536,"ancestor_record_offset":29724,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":33920,"parent_record_offset":33508,"ancestor_record_offset":29724,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":33652,"parent_record_offset":33508,"ancestor_record_offset":29724,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":29984,"parent_record_offset":29960,"ancestor_record_offset":29724,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":30988,"parent_record_offset":30964,"ancestor_record_offset":29724,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":30964,"parent_record_offset":30932,"ancestor_record_offset":29724,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":30932,"parent_record_offset":30904,"ancestor_record_offset":29724,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":35184,"parent_record_offset":35156,"ancestor_record_offset":29724,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":452,"record_offset":30220,"parent_record_offset":30192,"ancestor_record_offset":29724,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":452,"record_offset":30192,"parent_record_offset":30148,"ancestor_record_offset":29724,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":452,"record_offset":31052,"parent_record_offset":31020,"ancestor_record_offset":29724,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::WalkCamera::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":452,"record_offset":25712,"rva":6394528,"va":5375103648}
// 编译实例 RVAs: 0x6192a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=1
//     target_aliases=31 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2868369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375103648,"raw_hex":"e86f3bd4ff","size":5,"soft_fail":false,"target_va":5372235632,"va":5375103996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140515930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1063396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375103648,"raw_hex":"e81cc6efff","size":5,"soft_fail":false,"target_va":5374040368,"va":5375103759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140515bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1062738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375103648,"raw_hex":"e8aec8efff","size":5,"soft_fail":false,"target_va":5374041008,"va":5375103741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375103648,"raw_hex":"ff5030","size":3,"soft_fail":false,"va":5375103840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] dolly::drivers::position::Position::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/drivers/position.rs
//     evidence={"module":452,"record_offset":25888,"parent_record_offset":25712,"ancestor_record_offset":25712,"inlinee":22214,"source_line":24,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::builder
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":25860,"parent_record_offset":25712,"ancestor_record_offset":25712,"inlinee":22212,"source_line":86,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRigBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":25916,"parent_record_offset":25712,"ancestor_record_offset":25712,"inlinee":22218,"source_line":105,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":452,"record_offset":26084,"parent_record_offset":25916,"ancestor_record_offset":25712,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":26044,"parent_record_offset":26000,"ancestor_record_offset":25712,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":26000,"parent_record_offset":25956,"ancestor_record_offset":25712,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::update
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":25956,"parent_record_offset":25916,"ancestor_record_offset":25712,"inlinee":22205,"source_line":67,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::WalkCamera::teleport
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":452,"record_offset":26120,"rva":6394896,"va":5375104016}
// 编译实例 RVAs: 0x619410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$glam..euler..EulerRot$u20$as$u20$glam..euler..EulerFromQuaternion$LT$glam..f32..sse2..quat..Quat$GT$$GT$::convert_quat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11483189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375104016,"raw_hex":"e83538af00","size":5,"soft_fail":false,"target_va":5386587488,"va":5375104294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed78f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: walk.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9167828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375104016,"raw_hex":"e8d4e38b00","size":5,"soft_fail":false,"target_va":5384272112,"va":5375104279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27874464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375104016,"raw_hex":"e8a054a901","size":5,"soft_fail":false,"target_va":5402978944,"va":5375104475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*80(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375104016,"raw_hex":"ff5050","size":3,"soft_fail":false,"va":5375104132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":452,"record_offset":26276,"parent_record_offset":26120,"ancestor_record_offset":26120,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::drivers::yaw_pitch::YawPitch::set_rotation_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/drivers/yaw_pitch.rs
//     evidence={"module":452,"record_offset":27096,"parent_record_offset":26120,"ancestor_record_offset":26120,"inlinee":25929,"source_line":75,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":26332,"parent_record_offset":26120,"ancestor_record_offset":26120,"inlinee":22229,"source_line":31,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":26556,"parent_record_offset":26536,"ancestor_record_offset":26120,"inlinee":8015,"source_line":263,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":26580,"parent_record_offset":26556,"ancestor_record_offset":26120,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":26672,"parent_record_offset":26648,"ancestor_record_offset":26120,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_degrees
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":27152,"parent_record_offset":27096,"ancestor_record_offset":26120,"inlinee":25931,"source_line":884,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":26624,"parent_record_offset":26332,"ancestor_record_offset":26120,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":26500,"parent_record_offset":26476,"ancestor_record_offset":26120,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":26448,"parent_record_offset":26412,"ancestor_record_offset":26120,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":26476,"parent_record_offset":26412,"ancestor_record_offset":26120,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":26384,"parent_record_offset":26360,"ancestor_record_offset":26120,"inlinee":22195,"source_line":313,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":26412,"parent_record_offset":26384,"ancestor_record_offset":26120,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":26360,"parent_record_offset":26332,"ancestor_record_offset":26120,"inlinee":22194,"source_line":41,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":26648,"parent_record_offset":26624,"ancestor_record_offset":26120,"inlinee":22199,"source_line":32,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":26536,"parent_record_offset":26384,"ancestor_record_offset":26120,"inlinee":22197,"source_line":44,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::to_euler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":452,"record_offset":27124,"parent_record_offset":27096,"ancestor_record_offset":26120,"inlinee":25930,"source_line":397,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_mint::impl$34::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_mint.rs
//     evidence={"module":452,"record_offset":26300,"parent_record_offset":26276,"ancestor_record_offset":26120,"inlinee":22719,"source_line":46,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$1::rotate_yaw_pitch
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":452,"record_offset":27188,"rva":6172288,"va":5374881408}
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
//     evidence={"module":452,"record_offset":27340,"parent_record_offset":27188,"ancestor_record_offset":27188,"inlinee":22229,"source_line":31,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":27564,"parent_record_offset":27544,"ancestor_record_offset":27188,"inlinee":8015,"source_line":263,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":27588,"parent_record_offset":27564,"ancestor_record_offset":27188,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":27700,"parent_record_offset":27656,"ancestor_record_offset":27188,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":452,"record_offset":27728,"parent_record_offset":27656,"ancestor_record_offset":27188,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":27632,"parent_record_offset":27340,"ancestor_record_offset":27188,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":27508,"parent_record_offset":27484,"ancestor_record_offset":27188,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":27456,"parent_record_offset":27420,"ancestor_record_offset":27188,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":27484,"parent_record_offset":27420,"ancestor_record_offset":27188,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":27392,"parent_record_offset":27368,"ancestor_record_offset":27188,"inlinee":22195,"source_line":313,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":27420,"parent_record_offset":27392,"ancestor_record_offset":27188,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":27368,"parent_record_offset":27340,"ancestor_record_offset":27188,"inlinee":22194,"source_line":41,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":27656,"parent_record_offset":27632,"ancestor_record_offset":27188,"inlinee":22199,"source_line":32,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":27544,"parent_record_offset":27392,"ancestor_record_offset":27188,"inlinee":22197,"source_line":44,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":452,"record_offset":27772,"rva":6395376,"va":5375104496}
// 编译实例 RVAs: 0x6195f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27873042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375104496,"raw_hex":"e8124fa901","size":5,"soft_fail":false,"target_va":5402978944,"va":5375105897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::Raycasting::spherecast
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-596687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375104496,"raw_hex":"e831e5f6ff","size":5,"soft_fail":false,"target_va":5374508224,"va":5375104906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1834397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375104496,"raw_hex":"e89dfd1b00","size":5,"soft_fail":false,"target_va":5376939952,"va":5375105550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1834273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375104496,"raw_hex":"e821fd1b00","size":5,"soft_fail":false,"target_va":5376939968,"va":5375105690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*80(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375104496,"raw_hex":"ff5050","size":3,"soft_fail":false,"va":5375104644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":29664,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":452,"record_offset":28628,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":21417,"source_line":126,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig/walk.rs
//     evidence={"module":452,"record_offset":28448,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":25933,"source_line":92,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::rig::CameraRig::driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":27960,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":22229,"source_line":31,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":452,"record_offset":28392,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":452,"record_offset":29524,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":452,"record_offset":28656,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":28188,"parent_record_offset":28164,"ancestor_record_offset":27772,"inlinee":8015,"source_line":263,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":28212,"parent_record_offset":28188,"ancestor_record_offset":27772,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":452,"record_offset":28324,"parent_record_offset":28280,"ancestor_record_offset":27772,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":452,"record_offset":29044,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":28504,"parent_record_offset":28448,"ancestor_record_offset":27772,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":28568,"parent_record_offset":28448,"ancestor_record_offset":27772,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":28924,"parent_record_offset":28868,"ancestor_record_offset":27772,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":452,"record_offset":28352,"parent_record_offset":28280,"ancestor_record_offset":27772,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":28984,"parent_record_offset":28960,"ancestor_record_offset":27772,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":28416,"parent_record_offset":28392,"ancestor_record_offset":27772,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":28756,"parent_record_offset":28732,"ancestor_record_offset":27772,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":452,"record_offset":29460,"parent_record_offset":29432,"ancestor_record_offset":27772,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":28256,"parent_record_offset":27960,"ancestor_record_offset":27772,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":28128,"parent_record_offset":28104,"ancestor_record_offset":27772,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":28076,"parent_record_offset":28040,"ancestor_record_offset":27772,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":28104,"parent_record_offset":28040,"ancestor_record_offset":27772,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":28012,"parent_record_offset":27988,"ancestor_record_offset":27772,"inlinee":22195,"source_line":313,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":28040,"parent_record_offset":28012,"ancestor_record_offset":27772,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":452,"record_offset":29164,"parent_record_offset":29128,"ancestor_record_offset":27772,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::CameraRig::try_driver_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":27988,"parent_record_offset":27960,"ancestor_record_offset":27772,"inlinee":22194,"source_line":41,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":28280,"parent_record_offset":28256,"ancestor_record_offset":27772,"inlinee":22199,"source_line":32,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] dolly::rig::impl$0::try_driver_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/rig.rs
//     evidence={"module":452,"record_offset":28164,"parent_record_offset":28012,"ancestor_record_offset":27772,"inlinee":22197,"source_line":44,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":29016,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":28844,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":28868,"parent_record_offset":28844,"ancestor_record_offset":27772,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":28788,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":29432,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":28732,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":452,"record_offset":28960,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":452,"record_offset":29100,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":452,"record_offset":29068,"parent_record_offset":29044,"ancestor_record_offset":27772,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":452,"record_offset":29128,"parent_record_offset":28656,"ancestor_record_offset":27772,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x654D
// 首条位置: {"module":452,"inlinee":25933}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":28504,"parent_record_offset":28448,"ancestor_record_offset":28448,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":28568,"parent_record_offset":28448,"ancestor_record_offset":28448,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::walk::impl$1::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":452,"record_offset":35408,"rva":6174240,"va":5374883360}
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
//     evidence={"module":452,"record_offset":35552,"parent_record_offset":35408,"ancestor_record_offset":35408,"inlinee":22205,"source_line":67,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] dolly::transform::Transform::into_position_rotation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"dolly","version":"0.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dolly-0.6.0/src/transform.rs
//     evidence={"module":452,"record_offset":35680,"parent_record_offset":35408,"ancestor_record_offset":35408,"inlinee":24122,"source_line":31,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":35640,"parent_record_offset":35600,"ancestor_record_offset":35408,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":35600,"parent_record_offset":35552,"ancestor_record_offset":35408,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00452.jsonl.gz"}
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
// 完整结构及原始类型记录: walk.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::camera_rig::walk::WalkCamera
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::camera_rig::walk::WalkCamera>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::camera_rig::walk::WalkCamera"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: walk.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: walk.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::WalkCamera::new
// 信息 ID: 9b6d2dd33185b89c70e37c4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::camera_rig::MainCamera::new @ 0x516f55 -> 0x6192a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::WalkCamera::teleport
// 信息 ID: d54e189dab4b07f71e3e4c51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::camera_rig::MainCamera::init_walk_camera @ 0x51651b -> 0x619410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain
// 信息 ID: 900369563a581cc603ecb612
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain::closure$0
// 信息 ID: 353783eac760094a10cb29b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain; module=452, inline_record=28448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$1::rotate_yaw_pitch
// 信息 ID: 8c971bca10c3e349247f92a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$1::translate_screen_space
// 信息 ID: 23f771c9bb1cb10629445650
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::walk::impl$1::update
// 信息 ID: 7f8d52a0a66aa3d061c427cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
