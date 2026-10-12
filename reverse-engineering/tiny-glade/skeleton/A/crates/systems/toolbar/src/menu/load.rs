// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/menu/load.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 93340bccfe8046a989d80cdc799f5437c4b1d08f70bf08ab2a94e5b2f49d4c43
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1018, 2428, 2468, 2506
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_state; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"PDB_registry_path_only","version":"1.1.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$4::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2428,"record_offset":508,"rva":22991104,"va":5391700224}
// 编译实例 RVAs: 0x15ed100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18951806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391700224,"raw_hex":"e882d1defe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391701513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11277372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391700224,"raw_hex":"e83c14ac00","size":5,"soft_fail":false,"target_va":5402979152,"va":5391701775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::LoadMenuThumbnailMetrics::positions
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17301881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391700224,"raw_hex":"e887fef7fe","size":5,"soft_fail":false,"target_va":5374399312,"va":5391701188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11267257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391700224,"raw_hex":"e8b9ecab00","size":5,"soft_fail":false,"target_va":5402968992,"va":5391701730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11269046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391700224,"raw_hex":"e8b6f3ab00","size":5,"soft_fail":false,"target_va":5402970800,"va":5391701749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_mut
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14760112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391700224,"raw_hex":"e850c71eff","size":5,"soft_fail":false,"target_va":5376940320,"va":5391700427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2428,"record_offset":920,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2428,"record_offset":792,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2428,"record_offset":892,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2428,"record_offset":1268,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2428,"record_offset":864,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2428,"record_offset":820,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":23002,"source_line":2605,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2428,"record_offset":1916,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::bool::bvec2::BVec2::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/bvec2.rs
//     evidence={"module":2428,"record_offset":1884,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":17317,"source_line":65,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::bool::bvec2::impl$2::bitand
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/bvec2.rs
//     evidence={"module":2428,"record_offset":1856,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":17315,"source_line":117,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2428,"record_offset":1736,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":17311,"source_line":271,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2428,"record_offset":1800,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":17313,"source_line":293,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2428,"record_offset":1672,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2428,"record_offset":1012,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2428,"record_offset":1580,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2428,"record_offset":1068,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2428,"record_offset":1124,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2428,"record_offset":1460,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":2428,"record_offset":2248,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2428,"record_offset":764,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2428,"record_offset":704,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2428,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":508,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2428,"record_offset":2164,"parent_record_offset":2064,"ancestor_record_offset":508,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2428,"record_offset":2096,"parent_record_offset":2064,"ancestor_record_offset":508,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2428,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":508,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2428,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":508,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2428,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":508,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2428,"record_offset":1308,"parent_record_offset":1268,"ancestor_record_offset":508,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2428,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":508,"inlinee":17312,"source_line":1519,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2428,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":508,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2428,"record_offset":2128,"parent_record_offset":2096,"ancestor_record_offset":508,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2428,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":508,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2428,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":508,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2428,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":508,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2428,"record_offset":1180,"parent_record_offset":1124,"ancestor_record_offset":508,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2428,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":508,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2428,"record_offset":1388,"parent_record_offset":1308,"ancestor_record_offset":508,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2428,"record_offset":1352,"parent_record_offset":1308,"ancestor_record_offset":508,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2428,"record_offset":1416,"parent_record_offset":1388,"ancestor_record_offset":508,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2428,"record_offset":2364,"parent_record_offset":2276,"ancestor_record_offset":508,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2428,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":508,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2428,"record_offset":2516,"parent_record_offset":2484,"ancestor_record_offset":508,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2428,"record_offset":2308,"parent_record_offset":2276,"ancestor_record_offset":508,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2428,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":508,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2428,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":508,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":2428,"record_offset":2456,"parent_record_offset":2428,"ancestor_record_offset":508,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2428,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":508,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2428,"record_offset":2484,"parent_record_offset":2456,"ancestor_record_offset":508,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2428,"record_offset":2616,"rva":22992672,"va":5391701792}
// 编译实例 RVAs: 0x15ed720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22090388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e86cedaefe","size":5,"soft_fail":false,"target_va":5369613520,"va":5391703903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22087545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e887f8aefe","size":5,"soft_fail":false,"target_va":5369616080,"va":5391703620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22087165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e803faaefe","size":5,"soft_fail":false,"target_va":5369616480,"va":5391703640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22084294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e83a05affe","size":5,"soft_fail":false,"target_va":5369617648,"va":5391701937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18548558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8b2f8e4fe","size":5,"soft_fail":false,"target_va":5373154400,"va":5391702953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18524432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8f056e5fe","size":5,"soft_fail":false,"target_va":5373179424,"va":5391703851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17303602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8cef7f7fe","size":5,"soft_fail":false,"target_va":5374398944,"va":5391702541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a597b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12140897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e89fbe46ff","size":5,"soft_fail":false,"target_va":5379561392,"va":5391702284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59b30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12139847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8b9c246ff","size":5,"soft_fail":false,"target_va":5379562288,"va":5391702130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5b750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12132625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8efde46ff","size":5,"soft_fail":false,"target_va":5379569488,"va":5391702108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416183d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t173268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8d4a40200","size":5,"soft_fail":false,"target_va":5391877072,"va":5391703799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141618ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t176093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8ddaf0200","size":5,"soft_fail":false,"target_va":5391879072,"va":5391702974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161b490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t186305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8c1d70200","size":5,"soft_fail":false,"target_va":5391889552,"va":5391703242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-20393202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e80ed3c8fe","size":5,"soft_fail":false,"target_va":5371308800,"va":5391701997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11980288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8003249ff","size":5,"soft_fail":false,"target_va":5379722224,"va":5391702507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11274438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8c608ac00","size":5,"soft_fail":false,"target_va":5402978912,"va":5391704469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::NextSession::load
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17445557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e84bcdf5fe","size":5,"soft_fail":false,"target_va":5374256944,"va":5391702496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::is_session_ok_to_load
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15913757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8e32c0dff","size":5,"soft_fail":false,"target_va":5375789248,"va":5391703000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-15550957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e813b612ff","size":5,"soft_fail":false,"target_va":5376152880,"va":5391703832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-18542577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e80f10e5fe","size":5,"soft_fail":false,"target_va":5373159392,"va":5391701964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18542797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e8330fe5fe","size":5,"soft_fail":false,"target_va":5373160864,"va":5391703656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::delete_button
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391701792,"raw_hex":"e898040000","size":5,"soft_fail":false,"target_va":5391704480,"va":5391703299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2428,"record_offset":3140,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2428,"record_offset":2800,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2428,"record_offset":3936,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_state/src/state/resources.rs
//     evidence={"module":2428,"record_offset":3788,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":8074,"source_line":130,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2428,"record_offset":3588,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2428,"record_offset":3504,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":12684,"source_line":1444,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2428,"record_offset":3420,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2428,"record_offset":4852,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":2916,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":7513,"source_line":338,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":4340,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":50927,"source_line":360,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":5284,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":7454,"source_line":349,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":6836,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":5508,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2428,"record_offset":5732,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::clicked_item
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2428,"record_offset":3532,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":83131,"source_line":118,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::hovered_item
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2428,"record_offset":3476,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":83130,"source_line":110,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::hovered_item_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2428,"record_offset":4200,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":83132,"source_line":114,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::initial_focused_item
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2428,"record_offset":3236,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":83127,"source_line":122,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::lock
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2428,"record_offset":3360,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":83129,"source_line":162,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::min_scroll_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2428,"record_offset":3296,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":83128,"source_line":168,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2428,"record_offset":3068,"parent_record_offset":3036,"ancestor_record_offset":2616,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2428,"record_offset":3036,"parent_record_offset":3004,"ancestor_record_offset":2616,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2428,"record_offset":3004,"parent_record_offset":2940,"ancestor_record_offset":2616,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2428,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":2616,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2428,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":2616,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2428,"record_offset":4020,"parent_record_offset":3936,"ancestor_record_offset":2616,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2428,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":2616,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2428,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":2616,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2428,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":2616,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2428,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":2616,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2428,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":2616,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2428,"record_offset":6100,"parent_record_offset":6076,"ancestor_record_offset":2616,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2428,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":2616,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2428,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":2616,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2428,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":2616,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2428,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":2616,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2428,"record_offset":4928,"parent_record_offset":4904,"ancestor_record_offset":2616,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2428,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":2616,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2428,"record_offset":6412,"parent_record_offset":6388,"ancestor_record_offset":2616,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2428,"record_offset":3100,"parent_record_offset":3004,"ancestor_record_offset":2616,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2428,"record_offset":3176,"parent_record_offset":3140,"ancestor_record_offset":2616,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2428,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2616,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2428,"record_offset":3612,"parent_record_offset":3588,"ancestor_record_offset":2616,"inlinee":17530,"source_line":114,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2428,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":2616,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2428,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":2616,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2428,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":2616,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2428,"record_offset":6560,"parent_record_offset":6340,"ancestor_record_offset":2616,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2428,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":2616,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":2616,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":6448,"parent_record_offset":6340,"ancestor_record_offset":2616,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":6388,"parent_record_offset":6364,"ancestor_record_offset":2616,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":6768,"parent_record_offset":5980,"ancestor_record_offset":2616,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":2616,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":2616,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":5932,"parent_record_offset":5872,"ancestor_record_offset":2616,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":2616,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2428,"record_offset":6220,"parent_record_offset":6192,"ancestor_record_offset":2616,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2428,"record_offset":6248,"parent_record_offset":6220,"ancestor_record_offset":2616,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2428,"record_offset":6308,"parent_record_offset":6192,"ancestor_record_offset":2616,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2428,"record_offset":6192,"parent_record_offset":5980,"ancestor_record_offset":2616,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":6024,"parent_record_offset":5980,"ancestor_record_offset":2616,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2428,"record_offset":6340,"parent_record_offset":5980,"ancestor_record_offset":2616,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2428,"record_offset":6052,"parent_record_offset":5980,"ancestor_record_offset":2616,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2428,"record_offset":6136,"parent_record_offset":5980,"ancestor_record_offset":2616,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2428,"record_offset":6676,"parent_record_offset":5980,"ancestor_record_offset":2616,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2428,"record_offset":5816,"parent_record_offset":5788,"ancestor_record_offset":2616,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2428,"record_offset":5788,"parent_record_offset":5760,"ancestor_record_offset":2616,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":2616,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":3264,"parent_record_offset":3236,"ancestor_record_offset":2616,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":5760,"parent_record_offset":5732,"ancestor_record_offset":2616,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2428,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":2616,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":2976,"parent_record_offset":2940,"ancestor_record_offset":2616,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":2616,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":2616,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":5584,"parent_record_offset":5532,"ancestor_record_offset":2616,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":2616,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":2428,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":2616,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$0::thumbnail_scroll<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1018,"record_offset":115868,"rva":10860368,"va":5379569488}
// 编译实例 RVAs: 0xa5b750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9956250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379569488,"raw_hex":"e8661468ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5379569765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9953524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379569488,"raw_hex":"e80c1f68ff","size":5,"soft_fail":false,"target_va":5369616080,"va":5379569599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9953145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379569488,"raw_hex":"e8872068ff","size":5,"soft_fail":false,"target_va":5369616480,"va":5379569620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9951907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379569488,"raw_hex":"e85d2568ff","size":5,"soft_fail":false,"target_va":5369617648,"va":5379569550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d06f0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7909692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379569488,"raw_hex":"e8c44e87ff","size":5,"soft_fail":false,"target_va":5371660016,"va":5379569703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379569488,"raw_hex":"e825e3ffff","size":5,"soft_fail":false,"target_va":5379562288,"va":5379569670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23408667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379569488,"raw_hex":"e81b306501","size":5,"soft_fail":false,"target_va":5402978880,"va":5379570208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6408776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379569488,"raw_hex":"e8b8359eff","size":5,"soft_fail":false,"target_va":5373160864,"va":5379569635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":119004,"parent_record_offset":115868,"ancestor_record_offset":115868,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":116056,"parent_record_offset":115868,"ancestor_record_offset":115868,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::grab_focus
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":117920,"parent_record_offset":115868,"ancestor_record_offset":115868,"inlinee":17412,"source_line":839,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":116856,"parent_record_offset":115868,"ancestor_record_offset":115868,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":116028,"parent_record_offset":115868,"ancestor_record_offset":115868,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":116288,"parent_record_offset":116252,"ancestor_record_offset":115868,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":116252,"parent_record_offset":116224,"ancestor_record_offset":115868,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":117672,"parent_record_offset":117648,"ancestor_record_offset":115868,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":117140,"parent_record_offset":117116,"ancestor_record_offset":115868,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":117224,"parent_record_offset":117200,"ancestor_record_offset":115868,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":117164,"parent_record_offset":117140,"ancestor_record_offset":115868,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":117336,"parent_record_offset":117312,"ancestor_record_offset":115868,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":117888,"parent_record_offset":116856,"ancestor_record_offset":115868,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":116224,"parent_record_offset":116184,"ancestor_record_offset":115868,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":117584,"parent_record_offset":117560,"ancestor_record_offset":115868,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":117476,"parent_record_offset":117452,"ancestor_record_offset":115868,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1018,"record_offset":116320,"parent_record_offset":116288,"ancestor_record_offset":115868,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":116348,"parent_record_offset":116288,"ancestor_record_offset":115868,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":116380,"parent_record_offset":116252,"ancestor_record_offset":115868,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":117648,"parent_record_offset":117624,"ancestor_record_offset":115868,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":116936,"parent_record_offset":116912,"ancestor_record_offset":115868,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":116912,"parent_record_offset":116888,"ancestor_record_offset":115868,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":117624,"parent_record_offset":117404,"ancestor_record_offset":115868,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":116968,"parent_record_offset":116936,"ancestor_record_offset":115868,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117536,"parent_record_offset":117512,"ancestor_record_offset":115868,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117512,"parent_record_offset":117404,"ancestor_record_offset":115868,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117452,"parent_record_offset":117428,"ancestor_record_offset":115868,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117832,"parent_record_offset":117044,"ancestor_record_offset":115868,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117428,"parent_record_offset":117404,"ancestor_record_offset":115868,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117020,"parent_record_offset":116996,"ancestor_record_offset":115868,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":116996,"parent_record_offset":116936,"ancestor_record_offset":115868,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117044,"parent_record_offset":117020,"ancestor_record_offset":115868,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":117284,"parent_record_offset":117256,"ancestor_record_offset":115868,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":117312,"parent_record_offset":117284,"ancestor_record_offset":115868,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":117372,"parent_record_offset":117256,"ancestor_record_offset":115868,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":117256,"parent_record_offset":117044,"ancestor_record_offset":115868,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117088,"parent_record_offset":117044,"ancestor_record_offset":115868,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":117404,"parent_record_offset":117044,"ancestor_record_offset":115868,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":117116,"parent_record_offset":117044,"ancestor_record_offset":115868,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":117200,"parent_record_offset":117044,"ancestor_record_offset":115868,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":117740,"parent_record_offset":117044,"ancestor_record_offset":115868,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":116888,"parent_record_offset":116856,"ancestor_record_offset":115868,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":116128,"parent_record_offset":116056,"ancestor_record_offset":115868,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":118944,"parent_record_offset":117920,"ancestor_record_offset":115868,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":117696,"parent_record_offset":117672,"ancestor_record_offset":115868,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":116100,"parent_record_offset":116056,"ancestor_record_offset":115868,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":116152,"parent_record_offset":116128,"ancestor_record_offset":115868,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::impl$0::thumbnail_scroll::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1018,"record_offset":116184,"parent_record_offset":116056,"ancestor_record_offset":115868,"inlinee":50923,"source_line":42,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$0::thumbnail_scroll::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6EB
// 首条位置: {"module":1018,"inlinee":50923}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":116224,"parent_record_offset":116184,"ancestor_record_offset":116184,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":116288,"parent_record_offset":116252,"ancestor_record_offset":116184,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":116252,"parent_record_offset":116224,"ancestor_record_offset":116184,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1018,"record_offset":116320,"parent_record_offset":116288,"ancestor_record_offset":116184,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":116348,"parent_record_offset":116288,"ancestor_record_offset":116184,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":116380,"parent_record_offset":116252,"ancestor_record_offset":116184,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$8::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6E5
// 首条位置: {"module":1018,"inlinee":50917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":94424,"parent_record_offset":94396,"ancestor_record_offset":94396,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":94620,"parent_record_offset":94596,"ancestor_record_offset":94396,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":94644,"parent_record_offset":94620,"ancestor_record_offset":94396,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":94564,"parent_record_offset":94536,"ancestor_record_offset":94396,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":94596,"parent_record_offset":94564,"ancestor_record_offset":94396,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1018,"record_offset":94536,"parent_record_offset":94508,"ancestor_record_offset":94396,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":94508,"parent_record_offset":94424,"ancestor_record_offset":94396,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":94672,"parent_record_offset":94644,"ancestor_record_offset":94396,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$27::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":1018,"record_offset":94476,"parent_record_offset":94452,"ancestor_record_offset":94396,"inlinee":12031,"source_line":167,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1018,"record_offset":94452,"parent_record_offset":94424,"ancestor_record_offset":94396,"inlinee":9750,"source_line":422,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$1::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6E3
// 首条位置: {"module":1018,"inlinee":50915}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":94732,"parent_record_offset":94348,"ancestor_record_offset":94348,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1018,"record_offset":94396,"parent_record_offset":94348,"ancestor_record_offset":94348,"inlinee":50917,"source_line":57,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":94620,"parent_record_offset":94596,"ancestor_record_offset":94348,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":94644,"parent_record_offset":94620,"ancestor_record_offset":94348,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":94564,"parent_record_offset":94536,"ancestor_record_offset":94348,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":94596,"parent_record_offset":94564,"ancestor_record_offset":94348,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1018,"record_offset":94536,"parent_record_offset":94508,"ancestor_record_offset":94348,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":94996,"parent_record_offset":94960,"ancestor_record_offset":94348,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":94508,"parent_record_offset":94424,"ancestor_record_offset":94348,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":94780,"parent_record_offset":94756,"ancestor_record_offset":94348,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":94424,"parent_record_offset":94396,"ancestor_record_offset":94348,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":94756,"parent_record_offset":94732,"ancestor_record_offset":94348,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1018,"record_offset":95028,"parent_record_offset":94996,"ancestor_record_offset":94348,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":94672,"parent_record_offset":94644,"ancestor_record_offset":94348,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":94816,"parent_record_offset":94780,"ancestor_record_offset":94348,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":94840,"parent_record_offset":94816,"ancestor_record_offset":94348,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":94872,"parent_record_offset":94780,"ancestor_record_offset":94348,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$27::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":1018,"record_offset":94476,"parent_record_offset":94452,"ancestor_record_offset":94348,"inlinee":12031,"source_line":167,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1018,"record_offset":94452,"parent_record_offset":94424,"ancestor_record_offset":94348,"inlinee":9750,"source_line":422,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::hovered_item
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144BA
// 首条位置: {"module":2428,"inlinee":83130}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::hovered_item_pos
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144BC
// 首条位置: {"module":2428,"inlinee":83132}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::clicked_item
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144BB
// 首条位置: {"module":2428,"inlinee":83131}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::initial_focused_item
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144B7
// 首条位置: {"module":2428,"inlinee":83127}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":3264,"parent_record_offset":3236,"ancestor_record_offset":3236,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::tile_height
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144BD
// 首条位置: {"module":2506,"inlinee":83133}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2506,"record_offset":900,"parent_record_offset":872,"ancestor_record_offset":872,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02506.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::gap
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144BE
// 首条位置: {"module":2506,"inlinee":83134}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2506,"record_offset":960,"parent_record_offset":932,"ancestor_record_offset":932,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02506.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::run_count
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144BF
// 首条位置: {"module":2506,"inlinee":83135}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2506,"record_offset":1080,"parent_record_offset":1052,"ancestor_record_offset":1052,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02506.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::horizontal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144C0
// 首条位置: {"module":2506,"inlinee":83136}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2506,"record_offset":1020,"parent_record_offset":992,"ancestor_record_offset":992,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02506.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::lock
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144B9
// 首条位置: {"module":2428,"inlinee":83129}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":3388,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::min_scroll_pos
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x144B8
// 首条位置: {"module":2428,"inlinee":83128}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":3328,"parent_record_offset":3296,"ancestor_record_offset":3296,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$4::on_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2428,"record_offset":244,"rva":22991056,"va":5391700176}
// 编译实例 RVAs: 0x15ed0d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$4::prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2428,"record_offset":360,"rva":9083936,"va":5377793056}
// 编译实例 RVAs: 0x8a9c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":480,"parent_record_offset":360,"ancestor_record_offset":360,"inlinee":17206,"source_line":977,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14683
// 首条位置: {"module":2468,"inlinee":83587}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":19832,"parent_record_offset":19804,"ancestor_record_offset":19804,"inlinee":17396,"source_line":238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":16868,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":16928,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":17225,"source_line":220,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":16664,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$24::deref_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":16988,"parent_record_offset":16640,"ancestor_record_offset":16640,"inlinee":50904,"source_line":138,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":17836,"parent_record_offset":17812,"ancestor_record_offset":16640,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":16640,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":17388,"parent_record_offset":17364,"ancestor_record_offset":16640,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":16640,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":17500,"parent_record_offset":17476,"ancestor_record_offset":16640,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":19920,"parent_record_offset":19896,"ancestor_record_offset":19804,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":17748,"parent_record_offset":17724,"ancestor_record_offset":16640,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":17640,"parent_record_offset":17616,"ancestor_record_offset":16640,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":17812,"parent_record_offset":17788,"ancestor_record_offset":16640,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":17100,"parent_record_offset":17072,"ancestor_record_offset":16640,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":17072,"parent_record_offset":17044,"ancestor_record_offset":16640,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":17788,"parent_record_offset":17568,"ancestor_record_offset":16640,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":17132,"parent_record_offset":17100,"ancestor_record_offset":16640,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17700,"parent_record_offset":17676,"ancestor_record_offset":16640,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17676,"parent_record_offset":17568,"ancestor_record_offset":16640,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17616,"parent_record_offset":17592,"ancestor_record_offset":16640,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17996,"parent_record_offset":17208,"ancestor_record_offset":16640,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17592,"parent_record_offset":17568,"ancestor_record_offset":16640,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17184,"parent_record_offset":17160,"ancestor_record_offset":16640,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17160,"parent_record_offset":17100,"ancestor_record_offset":16640,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17208,"parent_record_offset":17184,"ancestor_record_offset":16640,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":17448,"parent_record_offset":17420,"ancestor_record_offset":16640,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":17476,"parent_record_offset":17448,"ancestor_record_offset":16640,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":17536,"parent_record_offset":17420,"ancestor_record_offset":16640,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":17420,"parent_record_offset":17208,"ancestor_record_offset":16640,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17252,"parent_record_offset":17208,"ancestor_record_offset":16640,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":17568,"parent_record_offset":17208,"ancestor_record_offset":16640,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":17280,"parent_record_offset":17208,"ancestor_record_offset":16640,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":17364,"parent_record_offset":17208,"ancestor_record_offset":16640,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":17904,"parent_record_offset":17208,"ancestor_record_offset":16640,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":17044,"parent_record_offset":17016,"ancestor_record_offset":16640,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":16776,"parent_record_offset":16716,"ancestor_record_offset":16640,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":16716,"parent_record_offset":16664,"ancestor_record_offset":16640,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":16896,"parent_record_offset":16868,"ancestor_record_offset":16640,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":17016,"parent_record_offset":16988,"ancestor_record_offset":16640,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2468,"record_offset":19868,"parent_record_offset":19832,"ancestor_record_offset":19804,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":16688,"parent_record_offset":16664,"ancestor_record_offset":16640,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":17860,"parent_record_offset":17836,"ancestor_record_offset":16640,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":16748,"parent_record_offset":16716,"ancestor_record_offset":16640,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":16800,"parent_record_offset":16776,"ancestor_record_offset":16640,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":16832,"parent_record_offset":16716,"ancestor_record_offset":16640,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 465
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1468E
// 首条位置: {"module":2468,"inlinee":83598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":50008,"parent_record_offset":49980,"ancestor_record_offset":49980,"inlinee":17396,"source_line":238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":47044,"parent_record_offset":46816,"ancestor_record_offset":46816,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":47104,"parent_record_offset":46816,"ancestor_record_offset":46816,"inlinee":17225,"source_line":220,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":46840,"parent_record_offset":46816,"ancestor_record_offset":46816,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$24::deref_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":47164,"parent_record_offset":46816,"ancestor_record_offset":46816,"inlinee":50904,"source_line":138,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":48012,"parent_record_offset":47988,"ancestor_record_offset":46816,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":47480,"parent_record_offset":47456,"ancestor_record_offset":46816,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":47564,"parent_record_offset":47540,"ancestor_record_offset":46816,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":47504,"parent_record_offset":47480,"ancestor_record_offset":46816,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":47676,"parent_record_offset":47652,"ancestor_record_offset":46816,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":50096,"parent_record_offset":50072,"ancestor_record_offset":49980,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":47924,"parent_record_offset":47900,"ancestor_record_offset":46816,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":47816,"parent_record_offset":47792,"ancestor_record_offset":46816,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":47988,"parent_record_offset":47964,"ancestor_record_offset":46816,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":47276,"parent_record_offset":47248,"ancestor_record_offset":46816,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":47248,"parent_record_offset":47220,"ancestor_record_offset":46816,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":47964,"parent_record_offset":47744,"ancestor_record_offset":46816,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":47308,"parent_record_offset":47276,"ancestor_record_offset":46816,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47876,"parent_record_offset":47852,"ancestor_record_offset":46816,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47852,"parent_record_offset":47744,"ancestor_record_offset":46816,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47792,"parent_record_offset":47768,"ancestor_record_offset":46816,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":48172,"parent_record_offset":47384,"ancestor_record_offset":46816,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47768,"parent_record_offset":47744,"ancestor_record_offset":46816,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47360,"parent_record_offset":47336,"ancestor_record_offset":46816,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47336,"parent_record_offset":47276,"ancestor_record_offset":46816,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47384,"parent_record_offset":47360,"ancestor_record_offset":46816,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":47624,"parent_record_offset":47596,"ancestor_record_offset":46816,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":47652,"parent_record_offset":47624,"ancestor_record_offset":46816,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":47712,"parent_record_offset":47596,"ancestor_record_offset":46816,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":47596,"parent_record_offset":47384,"ancestor_record_offset":46816,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47428,"parent_record_offset":47384,"ancestor_record_offset":46816,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":47744,"parent_record_offset":47384,"ancestor_record_offset":46816,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":47456,"parent_record_offset":47384,"ancestor_record_offset":46816,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":47540,"parent_record_offset":47384,"ancestor_record_offset":46816,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":48080,"parent_record_offset":47384,"ancestor_record_offset":46816,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":47220,"parent_record_offset":47192,"ancestor_record_offset":46816,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":46952,"parent_record_offset":46892,"ancestor_record_offset":46816,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":46892,"parent_record_offset":46840,"ancestor_record_offset":46816,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":47072,"parent_record_offset":47044,"ancestor_record_offset":46816,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":47192,"parent_record_offset":47164,"ancestor_record_offset":46816,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2468,"record_offset":50044,"parent_record_offset":50008,"ancestor_record_offset":49980,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":46864,"parent_record_offset":46840,"ancestor_record_offset":46816,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":48036,"parent_record_offset":48012,"ancestor_record_offset":46816,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":46924,"parent_record_offset":46892,"ancestor_record_offset":46816,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":46976,"parent_record_offset":46952,"ancestor_record_offset":46816,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":47008,"parent_record_offset":46892,"ancestor_record_offset":46816,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::delete_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 509
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2428,"record_offset":8432,"rva":22995360,"va":5391704480}
// 编译实例 RVAs: 0x15ee1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22086939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391704480,"raw_hex":"e8e5faaefe","size":5,"soft_fail":false,"target_va":5369617648,"va":5391704582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18550345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391704480,"raw_hex":"e8b7f1e4fe","size":5,"soft_fail":false,"target_va":5373154400,"va":5391704740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161d010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t191792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391704480,"raw_hex":"e830ed0200","size":5,"soft_fail":false,"target_va":5391896592,"va":5391704795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20395847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391704480,"raw_hex":"e8b9c8c8fe","size":5,"soft_fail":false,"target_va":5371308800,"va":5391704642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18545219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391704480,"raw_hex":"e8bd05e5fe","size":5,"soft_fail":false,"target_va":5373159392,"va":5391704606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2428,"record_offset":8556,"parent_record_offset":8432,"ancestor_record_offset":8432,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":8612,"parent_record_offset":8432,"ancestor_record_offset":8432,"inlinee":50927,"source_line":360,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2428,"record_offset":8768,"parent_record_offset":8736,"ancestor_record_offset":8432,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2428,"record_offset":8736,"parent_record_offset":8700,"ancestor_record_offset":8432,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2428,"record_offset":8700,"parent_record_offset":8636,"ancestor_record_offset":8432,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2428,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":8432,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2428,"record_offset":8800,"parent_record_offset":8700,"ancestor_record_offset":8432,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":8432,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2428,"record_offset":8672,"parent_record_offset":8636,"ancestor_record_offset":8432,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::delete_button::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 526
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2468,"record_offset":107016,"rva":23200320,"va":5391909440}
// 编译实例 RVAs: 0x1620240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22295970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391909440,"raw_hex":"e85ecaabfe","size":5,"soft_fail":false,"target_va":5369613520,"va":5391909485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5a660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12344566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391909440,"raw_hex":"e80aa343ff","size":5,"soft_fail":false,"target_va":5379565152,"va":5391909713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5b230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12341520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391909440,"raw_hex":"e8f0ae43ff","size":5,"soft_fail":false,"target_va":5379568176,"va":5391909691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11068550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391909440,"raw_hex":"e886e4a800","size":5,"soft_fail":false,"target_va":5402978912,"va":5391910357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11068554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391909440,"raw_hex":"e88ae4a800","size":5,"soft_fail":false,"target_va":5402978880,"va":5391910321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11069030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391909440,"raw_hex":"e866e6a800","size":5,"soft_fail":false,"target_va":5402979344,"va":5391910309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":110580,"parent_record_offset":107016,"ancestor_record_offset":107016,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::hover
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":107164,"parent_record_offset":107016,"ancestor_record_offset":107016,"inlinee":17401,"source_line":285,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::inactive_opacity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":108316,"parent_record_offset":107016,"ancestor_record_offset":107016,"inlinee":17264,"source_line":655,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":108376,"parent_record_offset":107016,"ancestor_record_offset":107016,"inlinee":17211,"source_line":660,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$40::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":109476,"parent_record_offset":107016,"ancestor_record_offset":107016,"inlinee":83593,"source_line":132,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":108012,"parent_record_offset":107988,"ancestor_record_offset":107016,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":107480,"parent_record_offset":107456,"ancestor_record_offset":107016,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":107564,"parent_record_offset":107540,"ancestor_record_offset":107016,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":107504,"parent_record_offset":107480,"ancestor_record_offset":107016,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":107676,"parent_record_offset":107652,"ancestor_record_offset":107016,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":108284,"parent_record_offset":107164,"ancestor_record_offset":107016,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":107924,"parent_record_offset":107900,"ancestor_record_offset":107016,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":107816,"parent_record_offset":107792,"ancestor_record_offset":107016,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2468,"record_offset":107192,"parent_record_offset":107164,"ancestor_record_offset":107016,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":107988,"parent_record_offset":107964,"ancestor_record_offset":107016,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":107248,"parent_record_offset":107220,"ancestor_record_offset":107016,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":107276,"parent_record_offset":107248,"ancestor_record_offset":107016,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":108488,"parent_record_offset":108464,"ancestor_record_offset":107016,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":108464,"parent_record_offset":108440,"ancestor_record_offset":107016,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":107964,"parent_record_offset":107744,"ancestor_record_offset":107016,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":107308,"parent_record_offset":107276,"ancestor_record_offset":107016,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107876,"parent_record_offset":107852,"ancestor_record_offset":107016,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107852,"parent_record_offset":107744,"ancestor_record_offset":107016,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107792,"parent_record_offset":107768,"ancestor_record_offset":107016,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":108172,"parent_record_offset":107384,"ancestor_record_offset":107016,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107768,"parent_record_offset":107744,"ancestor_record_offset":107016,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107360,"parent_record_offset":107336,"ancestor_record_offset":107016,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107336,"parent_record_offset":107276,"ancestor_record_offset":107016,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":108548,"parent_record_offset":108488,"ancestor_record_offset":107016,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107384,"parent_record_offset":107360,"ancestor_record_offset":107016,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":107624,"parent_record_offset":107596,"ancestor_record_offset":107016,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":107652,"parent_record_offset":107624,"ancestor_record_offset":107016,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":107712,"parent_record_offset":107596,"ancestor_record_offset":107016,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":107596,"parent_record_offset":107384,"ancestor_record_offset":107016,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107428,"parent_record_offset":107384,"ancestor_record_offset":107016,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":107744,"parent_record_offset":107384,"ancestor_record_offset":107016,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":107456,"parent_record_offset":107384,"ancestor_record_offset":107016,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":107540,"parent_record_offset":107384,"ancestor_record_offset":107016,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":108080,"parent_record_offset":107384,"ancestor_record_offset":107016,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":107220,"parent_record_offset":107164,"ancestor_record_offset":107016,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":108440,"parent_record_offset":108408,"ancestor_record_offset":107016,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":109532,"parent_record_offset":109504,"ancestor_record_offset":107016,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":108228,"parent_record_offset":107164,"ancestor_record_offset":107016,"inlinee":17189,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":108344,"parent_record_offset":108316,"ancestor_record_offset":107016,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":108408,"parent_record_offset":108376,"ancestor_record_offset":107016,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":109504,"parent_record_offset":109476,"ancestor_record_offset":107016,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2468,"record_offset":108252,"parent_record_offset":108228,"ancestor_record_offset":107016,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":108036,"parent_record_offset":108012,"ancestor_record_offset":107016,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
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
// 完整结构及原始类型记录: load.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::load::ThumbnailScrollContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::NodeContext::content", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::load::ThumbnailScrollView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::event::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::clicked_item
// 信息 ID: c65de91e110a733b63ff8701
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2428, inline_record=3532; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2506, inline_record=1528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::gap
// 信息 ID: 936d4067599c80f799b0068d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2506, inline_record=932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::horizontal
// 信息 ID: 0b5e95a32324e314ef6d6899
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2506, inline_record=992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::hovered_item
// 信息 ID: e5a206946f5159d0ae45ca27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2428, inline_record=3476; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2428, inline_record=4228; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2506, inline_record=1112; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2506, inline_record=1500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::hovered_item_pos
// 信息 ID: 2900548ab758a5707c300711
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2428, inline_record=4200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::initial_focused_item
// 信息 ID: c48b4ff73dcd92dbb0b5b09d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2428, inline_record=3236; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2506, inline_record=808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::lock
// 信息 ID: 9303e036ef5577e6a2c484b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2428, inline_record=3360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::min_scroll_pos
// 信息 ID: 3ed9e3be25b4f661c488139e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2428, inline_record=3296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::run_count
// 信息 ID: b6b4d35da5280638ba798668
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2506, inline_record=1052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::tile_height
// 信息 ID: 04cd618a70437e11a0e64a58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2506, inline_record=872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::delete_button
// 信息 ID: 5180dcfc4801394b363f8c9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui @ 0x15edd03 -> 0x15ee1a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::delete_button::closure$0
// 信息 ID: dabcc02e0ad6ef00b74e3868
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::delete_button", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::delete_button::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::delete_button::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::delete_button::closure_env$0,ref$<str$> > @ 0x161d245 -> 0x1620240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$0::thumbnail_scroll::closure$0
// 信息 ID: 95929a42ffa8be914fb62e44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::impl$0::thumbnail_scroll", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::impl$0::thumbnail_scroll<ref$<str$> >; module=1018, inline_record=116184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$0::thumbnail_scroll<ref$<str$> >
// 信息 ID: 05d22250ef9b7199d594a19f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] toolbar::menu::load::impl$0::thumbnail_scroll -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui @ 0x15ed85c -> 0xa5b750 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::lut_gallery::ui @ 0x1667dd8 -> 0xa5b750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$1::create_node_content
// 信息 ID: ef72871817cedb5c14b0cf95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::load::ThumbnailScrollContent>; module=1018, inline_record=94348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$4::on_input
// 信息 ID: f2a5e170e5bc89ffc09038b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$4::prepare
// 信息 ID: d9b6b3de55d4ceb35a148b1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$4::render
// 信息 ID: 9090cd5ce76b4d7af294fad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$8::default
// 信息 ID: 0483b7e39748806e37881e5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::load::ThumbnailScrollContent>; module=1018, inline_record=94396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui
// 信息 ID: cbcdd7e3a7ef8d41c773187d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e9a8aad5007157d4114e2e00: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> @ 0xa9352e -> 0x15ed720 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > @ 0x15e2a56 -> 0x15ed720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$1
// 信息 ID: 1574ea4ec2a27239929d8d82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=16640; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=18380; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=19804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$2
// 信息 ID: 52c3a543d22753dd32992740
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=46816; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=48556; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=49980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
