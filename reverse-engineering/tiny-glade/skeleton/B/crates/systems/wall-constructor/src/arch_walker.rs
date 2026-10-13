// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/arch_walker.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 093ebc0b0b18bc34bd034563571fce4ef5200f340474b22820e80e935ed15dc4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: arch_walker.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 183, 388, 396, 410, 1450, 1467
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F47
// 首条位置: {"module":388,"record_offset":272,"rva":7652048,"va":5376361168}
// 编译实例 RVAs: 0x74c2d0
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
// 首条位置: {"module":1450,"record_offset":8776,"rva":19082160,"va":5387791280}
// 编译实例 RVAs: 0x1232bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5902769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387791280,"raw_hex":"e84feea5ff","size":5,"soft_fail":false,"target_va":5381889712,"va":5387792476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5902572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387791280,"raw_hex":"e814efa5ff","size":5,"soft_fail":false,"target_va":5381889888,"va":5387792455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5856947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387791280,"raw_hex":"e84da1a6ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387791502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24056030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387791280,"raw_hex":"e8de106f01","size":5,"soft_fail":false,"target_va":5411847904,"va":5387791869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23748837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387791280,"raw_hex":"e8e5606a01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387793222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23750871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387791280,"raw_hex":"e8d7686a01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387791420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23748701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387791280,"raw_hex":"e85d606a01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387793198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_arches::construct_arches::closure$3
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387791280,"raw_hex":"e8c3e9ffff","size":5,"soft_fail":false,"target_va":5387786992,"va":5387792680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":9788,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1450,"record_offset":9192,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":9052,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1450,"record_offset":10284,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":9660,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":9572,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1450,"record_offset":9760,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1450,"record_offset":10368,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":14172,"source_line":122,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1450,"record_offset":9024,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":14168,"source_line":163,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":9148,"parent_record_offset":9124,"ancestor_record_offset":8776,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":10732,"parent_record_offset":10708,"ancestor_record_offset":8776,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":10756,"parent_record_offset":10732,"ancestor_record_offset":8776,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":10236,"parent_record_offset":10212,"ancestor_record_offset":8776,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":8776,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":10796,"parent_record_offset":10640,"ancestor_record_offset":8776,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":10584,"parent_record_offset":10368,"ancestor_record_offset":8776,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1450,"record_offset":10824,"parent_record_offset":10640,"ancestor_record_offset":8776,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1450,"record_offset":9228,"parent_record_offset":9192,"ancestor_record_offset":8776,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1450,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":8776,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":8776,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":9432,"parent_record_offset":9344,"ancestor_record_offset":8776,"inlinee":9538,"source_line":27,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":9260,"parent_record_offset":9192,"ancestor_record_offset":8776,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":9320,"parent_record_offset":9192,"ancestor_record_offset":8776,"inlinee":9568,"source_line":95,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":8776,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":8776,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":9344,"parent_record_offset":9320,"ancestor_record_offset":8776,"inlinee":9537,"source_line":341,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1450,"record_offset":10212,"parent_record_offset":10188,"ancestor_record_offset":8776,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1450,"record_offset":10188,"parent_record_offset":10072,"ancestor_record_offset":8776,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":8776,"inlinee":14173,"source_line":296,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":10048,"parent_record_offset":9984,"ancestor_record_offset":8776,"inlinee":9207,"source_line":347,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":8776,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":9600,"parent_record_offset":9572,"ancestor_record_offset":8776,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1450,"record_offset":10708,"parent_record_offset":10684,"ancestor_record_offset":8776,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1450,"record_offset":10684,"parent_record_offset":10640,"ancestor_record_offset":8776,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":9688,"parent_record_offset":9660,"ancestor_record_offset":8776,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":10492,"parent_record_offset":10368,"ancestor_record_offset":8776,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":10436,"parent_record_offset":10368,"ancestor_record_offset":8776,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1450,"record_offset":9724,"parent_record_offset":9688,"ancestor_record_offset":8776,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1450,"record_offset":10612,"parent_record_offset":10368,"ancestor_record_offset":8776,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1450,"record_offset":10640,"parent_record_offset":10368,"ancestor_record_offset":8776,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01450.jsonl.gz"}
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
// 首条位置: {"module":1450,"record_offset":10892,"rva":19084160,"va":5387793280}
// 编译实例 RVAs: 0x1233380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18527362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e87e4be5fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387795021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10052738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e87e9b66ff","size":5,"soft_fail":false,"target_va":5377742176,"va":5387794909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5904824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e848e6a5ff","size":5,"soft_fail":false,"target_va":5381889712,"va":5387794531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5904627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e80de7a5ff","size":5,"soft_fail":false,"target_va":5381889888,"va":5387794510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5858947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e87d99a6ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387793502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-161371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e8a589fdff","size":5,"soft_fail":false,"target_va":5387633712,"va":5387795078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24054048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e820096f01","size":5,"soft_fail":false,"target_va":5411847904,"va":5387793851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23746066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e812566a01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387795993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23748871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e807616a01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387793420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23745930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387793280,"raw_hex":"e88a556a01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387795969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":12012,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1450,"record_offset":11416,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":11276,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1450,"record_offset":12508,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":11884,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":11796,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1450,"record_offset":11984,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1450,"record_offset":12592,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":14172,"source_line":122,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1450,"record_offset":11248,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":14168,"source_line":163,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1450,"record_offset":14976,"parent_record_offset":10892,"ancestor_record_offset":10892,"inlinee":71006,"source_line":578,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":11372,"parent_record_offset":11348,"ancestor_record_offset":10892,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":12948,"parent_record_offset":12924,"ancestor_record_offset":10892,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":12972,"parent_record_offset":12948,"ancestor_record_offset":10892,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":12460,"parent_record_offset":12436,"ancestor_record_offset":10892,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":11348,"parent_record_offset":11324,"ancestor_record_offset":10892,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":13476,"parent_record_offset":13072,"ancestor_record_offset":10892,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":14060,"parent_record_offset":14004,"ancestor_record_offset":10892,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":10892,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":13616,"parent_record_offset":13560,"ancestor_record_offset":10892,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":13012,"parent_record_offset":12860,"ancestor_record_offset":10892,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":12804,"parent_record_offset":12592,"ancestor_record_offset":10892,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":13384,"parent_record_offset":13352,"ancestor_record_offset":10892,"inlinee":14178,"source_line":829,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1450,"record_offset":13040,"parent_record_offset":12860,"ancestor_record_offset":10892,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1450,"record_offset":11452,"parent_record_offset":11416,"ancestor_record_offset":10892,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1450,"record_offset":12324,"parent_record_offset":12296,"ancestor_record_offset":10892,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":10892,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":11656,"parent_record_offset":11568,"ancestor_record_offset":10892,"inlinee":9538,"source_line":27,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":11484,"parent_record_offset":11416,"ancestor_record_offset":10892,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":11544,"parent_record_offset":11416,"ancestor_record_offset":10892,"inlinee":9568,"source_line":95,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":11324,"parent_record_offset":11300,"ancestor_record_offset":10892,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":11508,"parent_record_offset":11484,"ancestor_record_offset":10892,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":10892,"inlinee":9537,"source_line":341,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1450,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":10892,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1450,"record_offset":12412,"parent_record_offset":12296,"ancestor_record_offset":10892,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":10892,"inlinee":14173,"source_line":296,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":12272,"parent_record_offset":12208,"ancestor_record_offset":10892,"inlinee":9207,"source_line":347,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":12532,"parent_record_offset":12508,"ancestor_record_offset":10892,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":13412,"parent_record_offset":13352,"ancestor_record_offset":10892,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":11824,"parent_record_offset":11796,"ancestor_record_offset":10892,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1450,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":10892,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1450,"record_offset":12900,"parent_record_offset":12860,"ancestor_record_offset":10892,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1450,"record_offset":13748,"parent_record_offset":13072,"ancestor_record_offset":10892,"inlinee":24469,"source_line":129,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1450,"record_offset":13860,"parent_record_offset":13072,"ancestor_record_offset":10892,"inlinee":24474,"source_line":538,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":10892,"inlinee":24471,"source_line":152,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":10892,"inlinee":24477,"source_line":646,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":10892,"inlinee":24480,"source_line":636,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13796,"parent_record_offset":13772,"ancestor_record_offset":10892,"inlinee":24472,"source_line":70,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13896,"parent_record_offset":13860,"ancestor_record_offset":10892,"inlinee":24476,"source_line":781,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13952,"parent_record_offset":13920,"ancestor_record_offset":10892,"inlinee":24479,"source_line":796,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13560,"parent_record_offset":13072,"ancestor_record_offset":10892,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13524,"parent_record_offset":13500,"ancestor_record_offset":10892,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":10892,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":14004,"parent_record_offset":13976,"ancestor_record_offset":10892,"inlinee":24482,"source_line":1199,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":10892,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":11912,"parent_record_offset":11884,"ancestor_record_offset":10892,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":12712,"parent_record_offset":12592,"ancestor_record_offset":10892,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":13352,"parent_record_offset":13328,"ancestor_record_offset":10892,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":13328,"parent_record_offset":13072,"ancestor_record_offset":10892,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":12656,"parent_record_offset":12592,"ancestor_record_offset":10892,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1450,"record_offset":14240,"parent_record_offset":14212,"ancestor_record_offset":10892,"inlinee":24486,"source_line":172,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1450,"record_offset":11948,"parent_record_offset":11912,"ancestor_record_offset":10892,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1450,"record_offset":14908,"parent_record_offset":13072,"ancestor_record_offset":10892,"inlinee":71007,"source_line":92,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1450,"record_offset":13072,"parent_record_offset":12592,"ancestor_record_offset":10892,"inlinee":71006,"source_line":578,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1450,"record_offset":12832,"parent_record_offset":12592,"ancestor_record_offset":10892,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1450,"record_offset":12860,"parent_record_offset":12592,"ancestor_record_offset":10892,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7>
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":2412,"rva":19078816,"va":5387787936}
// 编译实例 RVAs: 0x1231ea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140858750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10329811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e82d6162ff","size":5,"soft_fail":false,"target_va":5377460048,"va":5387789854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408587c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10329722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e8866162ff","size":5,"soft_fail":false,"target_va":5377460160,"va":5387789877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5899457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e83ffba5ff","size":5,"soft_fail":false,"target_va":5381889712,"va":5387789164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5899260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e804fca5ff","size":5,"soft_fail":false,"target_va":5381889888,"va":5387789143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5853605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e85baea6ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387788160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c120
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-157049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e8879afdff","size":5,"soft_fail":false,"target_va":5387632928,"va":5387789972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24059375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e8ef1d6f01","size":5,"soft_fail":false,"target_va":5411847904,"va":5387788524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23750842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e8ba686a01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387791217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23754215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e8e7756a01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387788076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23750706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e832686a01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387791193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_mat3_translation
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10047812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387787936,"raw_hex":"e8bcae66ff","size":5,"soft_fail":false,"target_va":5377742032,"va":5387789839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":3468,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1450,"record_offset":2872,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":2732,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1450,"record_offset":3964,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":3340,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":3252,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1450,"record_offset":3440,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1450,"record_offset":4048,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":14172,"source_line":122,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1450,"record_offset":2704,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":14168,"source_line":163,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/brackets.rs
//     evidence={"module":1450,"record_offset":6660,"parent_record_offset":2412,"ancestor_record_offset":2412,"inlinee":71004,"source_line":208,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2412,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":2412,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":2412,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":2412,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1450,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":2412,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":6596,"parent_record_offset":4536,"ancestor_record_offset":2412,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":4476,"parent_record_offset":4320,"ancestor_record_offset":2412,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":4264,"parent_record_offset":4048,"ancestor_record_offset":2412,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":5076,"parent_record_offset":5040,"ancestor_record_offset":2412,"inlinee":14178,"source_line":829,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1450,"record_offset":4504,"parent_record_offset":4320,"ancestor_record_offset":2412,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1450,"record_offset":2908,"parent_record_offset":2872,"ancestor_record_offset":2412,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1450,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":2412,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":2412,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":3112,"parent_record_offset":3024,"ancestor_record_offset":2412,"inlinee":9538,"source_line":27,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":2940,"parent_record_offset":2872,"ancestor_record_offset":2412,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":3000,"parent_record_offset":2872,"ancestor_record_offset":2412,"inlinee":9568,"source_line":95,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":2412,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":2412,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1450,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":2412,"inlinee":9537,"source_line":341,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1450,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":2412,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1450,"record_offset":3868,"parent_record_offset":3752,"ancestor_record_offset":2412,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":2412,"inlinee":14173,"source_line":296,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1450,"record_offset":3728,"parent_record_offset":3664,"ancestor_record_offset":2412,"inlinee":9207,"source_line":347,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":2412,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":5104,"parent_record_offset":5040,"ancestor_record_offset":2412,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":4860,"parent_record_offset":4828,"ancestor_record_offset":2412,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":2412,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1450,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":2412,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1450,"record_offset":4364,"parent_record_offset":4320,"ancestor_record_offset":2412,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":6292,"parent_record_offset":5244,"ancestor_record_offset":2412,"inlinee":14186,"source_line":86,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_mat3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":5244,"parent_record_offset":5208,"ancestor_record_offset":2412,"inlinee":14182,"source_line":571,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":5372,"parent_record_offset":5320,"ancestor_record_offset":2412,"inlinee":14185,"source_line":554,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":5208,"parent_record_offset":4536,"ancestor_record_offset":2412,"inlinee":14180,"source_line":706,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$9::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":5320,"parent_record_offset":5244,"ancestor_record_offset":2412,"inlinee":14184,"source_line":721,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":5168,"parent_record_offset":4536,"ancestor_record_offset":2412,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":2412,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":4172,"parent_record_offset":4048,"ancestor_record_offset":2412,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":2412,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":5012,"parent_record_offset":4536,"ancestor_record_offset":2412,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":5428,"parent_record_offset":5372,"ancestor_record_offset":2412,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":4736,"parent_record_offset":4536,"ancestor_record_offset":2412,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":4116,"parent_record_offset":4048,"ancestor_record_offset":2412,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":4828,"parent_record_offset":4536,"ancestor_record_offset":2412,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1450,"record_offset":3404,"parent_record_offset":3368,"ancestor_record_offset":2412,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/brackets.rs
//     evidence={"module":1450,"record_offset":4536,"parent_record_offset":4048,"ancestor_record_offset":2412,"inlinee":71004,"source_line":208,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1450,"record_offset":4292,"parent_record_offset":4048,"ancestor_record_offset":2412,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1450,"record_offset":4320,"parent_record_offset":4048,"ancestor_record_offset":2412,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":183,"record_offset":240,"rva":4762448,"va":5373471568}
// 编译实例 RVAs: 0x48ab50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8416935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373471568,"raw_hex":"e8a76e8000","size":5,"soft_fail":false,"target_va":5381889712,"va":5373472772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8417132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373471568,"raw_hex":"e86c6f8000","size":5,"soft_fail":false,"target_va":5381889888,"va":5373472751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8462765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373471568,"raw_hex":"e8ad218100","size":5,"soft_fail":false,"target_va":5381934560,"va":5373471790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: arch_walker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38375734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373471568,"raw_hex":"e836914902","size":5,"soft_fail":false,"target_va":5411847904,"va":5373472165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38068541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373471568,"raw_hex":"e83de14402","size":5,"soft_fail":false,"target_va":5411542064,"va":5373473518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38070583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373471568,"raw_hex":"e837e94402","size":5,"soft_fail":false,"target_va":5411542296,"va":5373471708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38068405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373471568,"raw_hex":"e8b5e04402","size":5,"soft_fail":false,"target_va":5411541904,"va":5373473494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373471568,"raw_hex":"e84b020000","size":5,"soft_fail":false,"target_va":5373473568,"va":5373472976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":183,"record_offset":1468,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":183,"record_offset":872,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":183,"record_offset":560,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":183,"record_offset":1964,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":14170,"source_line":35,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":1340,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":1252,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":183,"record_offset":1440,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":183,"record_offset":2048,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":14172,"source_line":122,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::curve_to_arch_ws
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":183,"record_offset":532,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":14168,"source_line":163,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":183,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":240,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":183,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":240,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":183,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":240,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":183,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":240,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":183,"record_offset":804,"parent_record_offset":608,"ancestor_record_offset":240,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":183,"record_offset":2476,"parent_record_offset":2320,"ancestor_record_offset":240,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":183,"record_offset":2264,"parent_record_offset":2048,"ancestor_record_offset":240,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":183,"record_offset":2504,"parent_record_offset":2320,"ancestor_record_offset":240,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":183,"record_offset":908,"parent_record_offset":872,"ancestor_record_offset":240,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":183,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":240,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":183,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":240,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":183,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":240,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":183,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":240,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::super_nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":183,"record_offset":1112,"parent_record_offset":1024,"ancestor_record_offset":240,"inlinee":9538,"source_line":27,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":183,"record_offset":940,"parent_record_offset":872,"ancestor_record_offset":240,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":183,"record_offset":1000,"parent_record_offset":872,"ancestor_record_offset":240,"inlinee":9568,"source_line":95,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":183,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":240,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":183,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":240,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":183,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":240,"inlinee":9537,"source_line":341,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":183,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":240,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":183,"record_offset":1868,"parent_record_offset":1752,"ancestor_record_offset":240,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":183,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":240,"inlinee":14173,"source_line":296,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":183,"record_offset":1728,"parent_record_offset":1664,"ancestor_record_offset":240,"inlinee":9207,"source_line":347,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":240,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":1280,"parent_record_offset":1252,"ancestor_record_offset":240,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":183,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":240,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":183,"record_offset":2364,"parent_record_offset":2320,"ancestor_record_offset":240,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":183,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":240,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":183,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":240,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":183,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":240,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":183,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":240,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":1368,"parent_record_offset":1340,"ancestor_record_offset":240,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":2172,"parent_record_offset":2048,"ancestor_record_offset":240,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":2116,"parent_record_offset":2048,"ancestor_record_offset":240,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":183,"record_offset":1404,"parent_record_offset":1368,"ancestor_record_offset":240,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":183,"record_offset":2292,"parent_record_offset":2048,"ancestor_record_offset":240,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":183,"record_offset":2320,"parent_record_offset":2048,"ancestor_record_offset":240,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x375C
// 首条位置: {"module":183,"inlinee":14172}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":183,"record_offset":2264,"parent_record_offset":2048,"ancestor_record_offset":2048,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":2172,"parent_record_offset":2048,"ancestor_record_offset":2048,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":2116,"parent_record_offset":2048,"ancestor_record_offset":2048,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs
//     evidence={"module":1450,"record_offset":13072,"parent_record_offset":12592,"ancestor_record_offset":12592,"inlinee":71006,"source_line":578,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/brackets.rs
//     evidence={"module":1450,"record_offset":4536,"parent_record_offset":4048,"ancestor_record_offset":4048,"inlinee":71004,"source_line":208,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":183,"record_offset":2292,"parent_record_offset":2048,"ancestor_record_offset":2048,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":183,"record_offset":2320,"parent_record_offset":2048,"ancestor_record_offset":2048,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":183,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":2048,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":183,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":2048,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":6596,"parent_record_offset":4536,"ancestor_record_offset":4048,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":14060,"parent_record_offset":14004,"ancestor_record_offset":12592,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":12592,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1450,"record_offset":13616,"parent_record_offset":13560,"ancestor_record_offset":12592,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":183,"record_offset":2476,"parent_record_offset":2320,"ancestor_record_offset":2048,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1450,"record_offset":5076,"parent_record_offset":5040,"ancestor_record_offset":4048,"inlinee":14178,"source_line":829,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":183,"record_offset":2504,"parent_record_offset":2320,"ancestor_record_offset":2048,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":4768,"parent_record_offset":4736,"ancestor_record_offset":4048,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":5104,"parent_record_offset":5040,"ancestor_record_offset":4048,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1450,"record_offset":4860,"parent_record_offset":4828,"ancestor_record_offset":4048,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":2048,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":183,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":2048,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":183,"record_offset":2364,"parent_record_offset":2320,"ancestor_record_offset":2048,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1450,"record_offset":13748,"parent_record_offset":13072,"ancestor_record_offset":12592,"inlinee":24469,"source_line":129,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1450,"record_offset":13860,"parent_record_offset":13072,"ancestor_record_offset":12592,"inlinee":24474,"source_line":538,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":6292,"parent_record_offset":5244,"ancestor_record_offset":4048,"inlinee":14186,"source_line":86,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":5244,"parent_record_offset":5208,"ancestor_record_offset":4048,"inlinee":14182,"source_line":571,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":5372,"parent_record_offset":5320,"ancestor_record_offset":4048,"inlinee":14185,"source_line":554,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":5208,"parent_record_offset":4536,"ancestor_record_offset":4048,"inlinee":14180,"source_line":706,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1450,"record_offset":5320,"parent_record_offset":5244,"ancestor_record_offset":4048,"inlinee":14184,"source_line":721,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":12592,"inlinee":24471,"source_line":152,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":12592,"inlinee":24477,"source_line":646,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":12592,"inlinee":24480,"source_line":636,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13796,"parent_record_offset":13772,"ancestor_record_offset":12592,"inlinee":24472,"source_line":70,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13896,"parent_record_offset":13860,"ancestor_record_offset":12592,"inlinee":24476,"source_line":781,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1450,"record_offset":13952,"parent_record_offset":13920,"ancestor_record_offset":12592,"inlinee":24479,"source_line":796,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13560,"parent_record_offset":13072,"ancestor_record_offset":12592,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13524,"parent_record_offset":13500,"ancestor_record_offset":12592,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":12592,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":14004,"parent_record_offset":13976,"ancestor_record_offset":12592,"inlinee":24482,"source_line":1199,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1450,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":12592,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":5168,"parent_record_offset":4536,"ancestor_record_offset":4048,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":2048,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":4920,"parent_record_offset":4536,"ancestor_record_offset":4048,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":4048,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":5012,"parent_record_offset":4536,"ancestor_record_offset":4048,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":5428,"parent_record_offset":5372,"ancestor_record_offset":4048,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":4736,"parent_record_offset":4536,"ancestor_record_offset":4048,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":4644,"parent_record_offset":4536,"ancestor_record_offset":4048,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1450,"record_offset":4828,"parent_record_offset":4536,"ancestor_record_offset":4048,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1450,"record_offset":14240,"parent_record_offset":14212,"ancestor_record_offset":12592,"inlinee":24486,"source_line":172,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":183,"record_offset":2228,"parent_record_offset":2196,"ancestor_record_offset":2048,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::construct_lintels::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/mod.rs
//     evidence={"module":1450,"record_offset":14908,"parent_record_offset":13072,"ancestor_record_offset":12592,"inlinee":71007,"source_line":92,"archive":"_pdb/B/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::arch_walker::curve_to_arch_ws
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3758
// 首条位置: {"module":183,"inlinee":14168}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7>", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_arches::construct_arches::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >", "constructor": "system_wall_constructor::arch_walker::ArchWalker::walk", "arguments": ["system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: arch_walker.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/B-arches/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_wall_constructor::arch_walker::ArchWalker", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/B-profile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_wall_constructor::arch_walker::ArchWalker", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/arch_walker.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: arch_walker.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::emit_segment_bricks
// 信息 ID: dd80a3f22cde6f8c18bd982f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >; module=183, inline_record=2048; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7>; module=1450, inline_record=4048; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>; module=1450, inline_record=10368; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1450, inline_record=12592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::new
// 信息 ID: c8f9b0326cf868e3e592b352
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 2 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true> @ 0x49877e -> 0x74c2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x49eb1c -> 0x74c2d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=396, inline_record=19044; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets; module=410, inline_record=15424; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=9884; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3>
// 信息 ID: faaf1cb2880e1725c75218f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_arches::construct_arches::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760beb -> 0x1232bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >
// 信息 ID: 2204131e8b7bcebb9aab0391
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0 -> ["system_wall_constructor::construct_lintels::construct_lintels::closure_env$2"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1243eec -> 0x1233380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x12440a3 -> 0x1233380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7>
// 信息 ID: 5a1fdf7ea5aeab1919196df9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets @ 0x784772 -> 0x1231ea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >
// 信息 ID: f16a4b606aada30872604e51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6 -> ["false"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::arch_walker::ArchWalker::walk -> ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true> @ 0x4988bb -> 0x48ab50 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x49ec60 -> 0x48ab50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::arch_walker::curve_to_arch_ws
// 信息 ID: e05bf0bc647e0abed71419a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >; module=183, inline_record=532; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >; module=183, inline_record=1224; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >; module=183, inline_record=1496; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >; module=183, inline_record=2020; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >; module=183, inline_record=2540; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7>; module=1450, inline_record=2704; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7>; module=1450, inline_record=3224; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7>; module=1450, inline_record=3496; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
