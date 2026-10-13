// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 347ca81f88d86579ed550c6dc2dd49f0689f2e208ae120d549180128bb036027
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_railings_assemble.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 448, 1546, 1559
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":264,"rva":8362208,"va":5377071328}
// 编译实例 RVAs: 0x7f98e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=4
//     target_aliases=14 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7014343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e839f894ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5377081826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1990743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8a99fe1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377072226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=23
//     target_aliases=1 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1990439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8d9a0e1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377071938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1996594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8ce88e1ff","size":5,"soft_fail":false,"target_va":5375081536,"va":5377078125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t382962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8f2d70500","size":5,"soft_fail":false,"target_va":5377460048,"va":5377077081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408587c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t382935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8d7d70500","size":5,"soft_fail":false,"target_va":5377460160,"va":5377077220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4861798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8662f4a00","size":5,"soft_fail":false,"target_va":5381934528,"va":5377072725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141268e90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10939833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8b9eda600","size":5,"soft_fail":false,"target_va":5388013200,"va":5377073362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269710
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10936852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e814e2a600","size":5,"soft_fail":false,"target_va":5388015376,"va":5377078519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10943598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e86efca600","size":5,"soft_fail":false,"target_va":5388016864,"va":5377073261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14126a110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10939423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e81feca600","size":5,"soft_fail":false,"target_va":5388017936,"va":5377078508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128cdd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11083096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8581da900","size":5,"soft_fail":false,"target_va":5388160464,"va":5377077363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412938f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11113760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e82095a900","size":5,"soft_fail":false,"target_va":5388187888,"va":5377074123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34768886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8f6871202","size":5,"soft_fail":false,"target_va":5411847168,"va":5377078277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3821893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e845513a00","size":5,"soft_fail":false,"target_va":5380895104,"va":5377073206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34459623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8e7cf0d02","size":5,"soft_fail":false,"target_va":5411540963,"va":5377081335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34460728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e838d40d02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377081043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34461223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e827d60d02","size":5,"soft_fail":false,"target_va":5411542422,"va":5377081194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=23
//     evidence={"assembly":"\tcallq\t34461210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e81ad60d02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377081081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_cols
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t665005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8ad250a00","size":5,"soft_fail":false,"target_va":5377742080,"va":5377077070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=15
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4937069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e86d554b00","size":5,"soft_fail":false,"target_va":5382009728,"va":5377072654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::stairs_railings_assemble::construct_wooden_railing
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e85a290000","size":5,"soft_fail":false,"target_va":5377089424,"va":5377078833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4808880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8b0604900","size":5,"soft_fail":false,"target_va":5381882704,"va":5377073819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_mat3_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4805819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377071328,"raw_hex":"e8bb544900","size":5,"soft_fail":false,"target_va":5381885648,"va":5377079824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":2996,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":1460,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":1208,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":11332,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":12036,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":1112,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":12964,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":448,"record_offset":14664,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":16204,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":448,"record_offset":9076,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":448,"record_offset":6152,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":448,"record_offset":2112,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":448,"record_offset":6180,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":448,"record_offset":1656,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":448,"record_offset":1028,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":3024,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":4704,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":448,"record_offset":2056,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":19714,"source_line":35,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":596,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":4796,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":4608,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":772,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":448,"record_offset":2316,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":448,"record_offset":1912,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":448,"record_offset":3944,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":9104,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":25450,"source_line":237,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":5968,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":6656,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":4920,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4745,"source_line":516,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":2140,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":7456,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":6956,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":7136,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":6436,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":2168,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":2260,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":7608,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":4832,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$53::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":2204,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":24869,"source_line":1501,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":448,"record_offset":16452,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":448,"record_offset":6236,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":448,"record_offset":2968,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":448,"record_offset":12640,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":264,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":11540,"parent_record_offset":11516,"ancestor_record_offset":264,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":1356,"parent_record_offset":1324,"ancestor_record_offset":264,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":11492,"parent_record_offset":11460,"ancestor_record_offset":264,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":952,"parent_record_offset":868,"ancestor_record_offset":264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":1536,"parent_record_offset":1504,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":1264,"parent_record_offset":1236,"ancestor_record_offset":264,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":11396,"parent_record_offset":11364,"ancestor_record_offset":264,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":1324,"parent_record_offset":1292,"ancestor_record_offset":264,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":1292,"parent_record_offset":1264,"ancestor_record_offset":264,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":11428,"parent_record_offset":11396,"ancestor_record_offset":264,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":3336,"parent_record_offset":3300,"ancestor_record_offset":264,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":12120,"parent_record_offset":12092,"ancestor_record_offset":264,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":12092,"parent_record_offset":12064,"ancestor_record_offset":264,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":12064,"parent_record_offset":12036,"ancestor_record_offset":264,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":1504,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":264,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":3176,"parent_record_offset":3132,"ancestor_record_offset":264,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":264,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":3060,"parent_record_offset":3024,"ancestor_record_offset":264,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":448,"record_offset":3580,"parent_record_offset":3132,"ancestor_record_offset":264,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":448,"record_offset":11364,"parent_record_offset":11332,"ancestor_record_offset":264,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":448,"record_offset":3096,"parent_record_offset":3060,"ancestor_record_offset":264,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":448,"record_offset":3132,"parent_record_offset":3096,"ancestor_record_offset":264,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":448,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":264,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":448,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":264,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":448,"record_offset":14736,"parent_record_offset":14692,"ancestor_record_offset":264,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":9232,"parent_record_offset":9196,"ancestor_record_offset":264,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":9292,"parent_record_offset":9268,"ancestor_record_offset":264,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":9268,"parent_record_offset":9196,"ancestor_record_offset":264,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":264,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":448,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":448,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":264,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":448,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":264,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":448,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":264,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":448,"record_offset":1692,"parent_record_offset":1656,"ancestor_record_offset":264,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":448,"record_offset":1784,"parent_record_offset":1748,"ancestor_record_offset":264,"inlinee":24402,"source_line":108,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":448,"record_offset":1720,"parent_record_offset":1692,"ancestor_record_offset":264,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":448,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":264,"inlinee":11459,"source_line":269,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":448,"record_offset":1748,"parent_record_offset":1720,"ancestor_record_offset":264,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":448,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":264,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":3628,"parent_record_offset":3604,"ancestor_record_offset":264,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":264,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":264,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":448,"record_offset":14760,"parent_record_offset":14736,"ancestor_record_offset":264,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":448,"record_offset":9316,"parent_record_offset":9292,"ancestor_record_offset":264,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":264,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":264,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":9388,"parent_record_offset":9128,"ancestor_record_offset":264,"inlinee":20286,"source_line":2065,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":9468,"parent_record_offset":9440,"ancestor_record_offset":264,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":264,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":6260,"parent_record_offset":6236,"ancestor_record_offset":264,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":7636,"parent_record_offset":7608,"ancestor_record_offset":264,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":264,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":448,"record_offset":14692,"parent_record_offset":14664,"ancestor_record_offset":264,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":264,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":4668,"parent_record_offset":4644,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":448,"record_offset":12376,"parent_record_offset":12120,"ancestor_record_offset":264,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":448,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":264,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":448,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":448,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":448,"record_offset":4128,"parent_record_offset":4096,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":14808,"parent_record_offset":14784,"ancestor_record_offset":264,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":1624,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":264,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":12992,"parent_record_offset":12964,"ancestor_record_offset":264,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":264,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":1168,"parent_record_offset":1140,"ancestor_record_offset":264,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":13020,"parent_record_offset":12992,"ancestor_record_offset":264,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":448,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":264,"inlinee":4540,"source_line":372,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":4096,"parent_record_offset":4060,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":4156,"parent_record_offset":4096,"ancestor_record_offset":264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":264,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":9172,"parent_record_offset":9128,"ancestor_record_offset":264,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":264,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":448,"record_offset":5416,"parent_record_offset":5384,"ancestor_record_offset":264,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":264,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":264,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":264,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":264,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":5992,"parent_record_offset":5968,"ancestor_record_offset":264,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":6524,"parent_record_offset":6492,"ancestor_record_offset":264,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":6492,"parent_record_offset":6460,"ancestor_record_offset":264,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":264,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":264,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":7244,"parent_record_offset":7136,"ancestor_record_offset":264,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":6980,"parent_record_offset":6956,"ancestor_record_offset":264,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":448,"record_offset":4244,"parent_record_offset":4060,"ancestor_record_offset":264,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":448,"record_offset":4060,"parent_record_offset":4028,"ancestor_record_offset":264,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":448,"record_offset":5440,"parent_record_offset":5416,"ancestor_record_offset":264,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs
//     evidence={"module":448,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":264,"inlinee":25446,"source_line":61,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs
//     evidence={"module":448,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":264,"inlinee":25447,"source_line":64,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs
//     evidence={"module":448,"record_offset":1996,"parent_record_offset":1968,"ancestor_record_offset":264,"inlinee":25448,"source_line":103,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs
//     evidence={"module":448,"record_offset":2348,"parent_record_offset":2316,"ancestor_record_offset":264,"inlinee":25449,"source_line":133,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs
//     evidence={"module":448,"record_offset":4644,"parent_record_offset":4608,"ancestor_record_offset":264,"inlinee":25451,"source_line":183,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":448,"record_offset":12692,"parent_record_offset":12640,"ancestor_record_offset":264,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":448,"record_offset":12784,"parent_record_offset":12640,"ancestor_record_offset":264,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6366
// 首条位置: {"module":448,"inlinee":25446}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":448,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":624,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6367
// 首条位置: {"module":448,"inlinee":25447}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":448,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6368
// 首条位置: {"module":448,"inlinee":25448}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":448,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1996,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6369
// 首条位置: {"module":448,"inlinee":25449}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":2348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":2348,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":2564,"parent_record_offset":2472,"ancestor_record_offset":2348,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":2348,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":2348,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":2348,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":2532,"parent_record_offset":2500,"ancestor_record_offset":2348,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":2348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11883
// 首条位置: {"module":1546,"inlinee":71811}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1546,"record_offset":22492,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1546,"record_offset":22148,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1546,"record_offset":22180,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::fract
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1546,"record_offset":22208,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":12960,"source_line":172,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1546,"record_offset":22884,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1546,"record_offset":22672,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1546,"record_offset":22264,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1546,"record_offset":22552,"parent_record_offset":22528,"ancestor_record_offset":22092,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1546,"record_offset":22600,"parent_record_offset":22576,"ancestor_record_offset":22092,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1546,"record_offset":22576,"parent_record_offset":22552,"ancestor_record_offset":22092,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1546,"record_offset":22528,"parent_record_offset":22492,"ancestor_record_offset":22092,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1546,"record_offset":22984,"parent_record_offset":22920,"ancestor_record_offset":22092,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1546,"record_offset":22760,"parent_record_offset":22736,"ancestor_record_offset":22092,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1546,"record_offset":22784,"parent_record_offset":22760,"ancestor_record_offset":22092,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1546,"record_offset":22824,"parent_record_offset":22672,"ancestor_record_offset":22092,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1546,"record_offset":22852,"parent_record_offset":22672,"ancestor_record_offset":22092,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1546,"record_offset":23488,"parent_record_offset":23456,"ancestor_record_offset":22092,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1546,"record_offset":23332,"parent_record_offset":23292,"ancestor_record_offset":22092,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1546,"record_offset":22736,"parent_record_offset":22712,"ancestor_record_offset":22092,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1546,"record_offset":22712,"parent_record_offset":22672,"ancestor_record_offset":22092,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1546,"record_offset":22640,"parent_record_offset":22492,"ancestor_record_offset":22092,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1546,"record_offset":22332,"parent_record_offset":22264,"ancestor_record_offset":22092,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1546,"record_offset":22360,"parent_record_offset":22332,"ancestor_record_offset":22092,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1546,"record_offset":23260,"parent_record_offset":23228,"ancestor_record_offset":22092,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1546,"record_offset":23292,"parent_record_offset":23260,"ancestor_record_offset":22092,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1546,"record_offset":23456,"parent_record_offset":23260,"ancestor_record_offset":22092,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::trunc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1546,"record_offset":22232,"parent_record_offset":22208,"ancestor_record_offset":22092,"inlinee":12961,"source_line":149,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1546,"record_offset":22920,"parent_record_offset":22884,"ancestor_record_offset":22092,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1546,"record_offset":22304,"parent_record_offset":22264,"ancestor_record_offset":22092,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1546,"record_offset":22396,"parent_record_offset":22264,"ancestor_record_offset":22092,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1546,"record_offset":23228,"parent_record_offset":22920,"ancestor_record_offset":22092,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x636B
// 首条位置: {"module":448,"inlinee":25451}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":4668,"parent_record_offset":4644,"ancestor_record_offset":4644,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::railings_tips
// 记录形式: compiled_procedure; instances=1
// 源码行: 485
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":21088,"rva":8375424,"va":5377084544}
// 编译实例 RVAs: 0x7fcc80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7020039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8f9e194ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5377087522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2005128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e87867e1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377086611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2005085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8a367e1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377086584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2005514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8f665e1ff","size":5,"soft_fail":false,"target_va":5375081536,"va":5377087045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t373747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8f3b30500","size":5,"soft_fail":false,"target_va":5377460048,"va":5377086296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4849479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e847ff4900","size":5,"soft_fail":false,"target_va":5381934528,"va":5377085044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141268e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10926324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8f4b8a600","size":5,"soft_fail":false,"target_va":5388013200,"va":5377086871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10930085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8a5c7a600","size":5,"soft_fail":false,"target_va":5388016864,"va":5377086774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128cdd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11073927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e887f9a800","size":5,"soft_fail":false,"target_va":5388160464,"va":5377086532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34453609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e869b80d02","size":5,"soft_fail":false,"target_va":5411540992,"va":5377087378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34453560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e838b80d02","size":5,"soft_fail":false,"target_va":5411540963,"va":5377087398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34454972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8bcbd0d02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377087319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_cols
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t655790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8ae010a00","size":5,"soft_fail":false,"target_va":5377742080,"va":5377086285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::stairs_railings_assemble::construct_wooden_railing
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8fd080000","size":5,"soft_fail":false,"target_va":5377089424,"va":5377087118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8d70d0000","size":5,"soft_fail":false,"target_va":5377088368,"va":5377084820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4796885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e8d5314900","size":5,"soft_fail":false,"target_va":5381882704,"va":5377085814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_mat3_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4800123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377084544,"raw_hex":"e87b3e4900","size":5,"soft_fail":false,"target_va":5381885648,"va":5377085520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":25072,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":25592,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":26572,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":22480,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":448,"record_offset":24384,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":448,"record_offset":21320,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":448,"record_offset":22508,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":24840,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":448,"record_offset":25676,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":448,"record_offset":26256,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":24412,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":25450,"source_line":237,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":21624,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":21596,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":22212,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":23096,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":21408,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":22028,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":22120,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":448,"record_offset":22596,"parent_record_offset":21088,"ancestor_record_offset":21088,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":25128,"parent_record_offset":25104,"ancestor_record_offset":21088,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":25152,"parent_record_offset":25128,"ancestor_record_offset":21088,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":26780,"parent_record_offset":26756,"ancestor_record_offset":21088,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":25024,"parent_record_offset":25000,"ancestor_record_offset":21088,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":25104,"parent_record_offset":25072,"ancestor_record_offset":21088,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":26732,"parent_record_offset":26700,"ancestor_record_offset":21088,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":25000,"parent_record_offset":24912,"ancestor_record_offset":21088,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26636,"parent_record_offset":26604,"ancestor_record_offset":21088,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":24940,"parent_record_offset":24912,"ancestor_record_offset":21088,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":24912,"parent_record_offset":24888,"ancestor_record_offset":21088,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26700,"parent_record_offset":26668,"ancestor_record_offset":21088,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26668,"parent_record_offset":26636,"ancestor_record_offset":21088,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":24888,"parent_record_offset":24864,"ancestor_record_offset":21088,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":25616,"parent_record_offset":25592,"ancestor_record_offset":21088,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":25640,"parent_record_offset":25616,"ancestor_record_offset":21088,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":448,"record_offset":26604,"parent_record_offset":26572,"ancestor_record_offset":21088,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":24532,"parent_record_offset":24500,"ancestor_record_offset":21088,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":24592,"parent_record_offset":24568,"ancestor_record_offset":21088,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":24568,"parent_record_offset":24500,"ancestor_record_offset":21088,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":22556,"parent_record_offset":22532,"ancestor_record_offset":21088,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":448,"record_offset":22824,"parent_record_offset":22800,"ancestor_record_offset":21088,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":448,"record_offset":22872,"parent_record_offset":22848,"ancestor_record_offset":21088,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":448,"record_offset":22532,"parent_record_offset":22508,"ancestor_record_offset":21088,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":22896,"parent_record_offset":22872,"ancestor_record_offset":21088,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":448,"record_offset":24616,"parent_record_offset":24592,"ancestor_record_offset":21088,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":24968,"parent_record_offset":24940,"ancestor_record_offset":21088,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":24688,"parent_record_offset":24436,"ancestor_record_offset":21088,"inlinee":20286,"source_line":2065,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":24768,"parent_record_offset":24740,"ancestor_record_offset":21088,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":22060,"parent_record_offset":22028,"ancestor_record_offset":21088,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":21440,"parent_record_offset":21408,"ancestor_record_offset":21088,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":22152,"parent_record_offset":22120,"ancestor_record_offset":21088,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":22848,"parent_record_offset":22824,"ancestor_record_offset":21088,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":24864,"parent_record_offset":24840,"ancestor_record_offset":21088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":448,"record_offset":22760,"parent_record_offset":22736,"ancestor_record_offset":21088,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":448,"record_offset":21372,"parent_record_offset":21344,"ancestor_record_offset":21088,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":25180,"parent_record_offset":25152,"ancestor_record_offset":21088,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":24640,"parent_record_offset":24616,"ancestor_record_offset":21088,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":22736,"parent_record_offset":22712,"ancestor_record_offset":21088,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":21344,"parent_record_offset":21320,"ancestor_record_offset":21088,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":22712,"parent_record_offset":22636,"ancestor_record_offset":21088,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":24436,"parent_record_offset":24412,"ancestor_record_offset":21088,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":24476,"parent_record_offset":24436,"ancestor_record_offset":21088,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":24500,"parent_record_offset":24476,"ancestor_record_offset":21088,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":21712,"parent_record_offset":21680,"ancestor_record_offset":21088,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":21680,"parent_record_offset":21648,"ancestor_record_offset":21088,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":21648,"parent_record_offset":21624,"ancestor_record_offset":21088,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":22420,"parent_record_offset":22396,"ancestor_record_offset":21088,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":22324,"parent_record_offset":22236,"ancestor_record_offset":21088,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":23128,"parent_record_offset":23096,"ancestor_record_offset":21088,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":23284,"parent_record_offset":23096,"ancestor_record_offset":21088,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":22236,"parent_record_offset":22212,"ancestor_record_offset":21088,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":448,"record_offset":22800,"parent_record_offset":22596,"ancestor_record_offset":21088,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":448,"record_offset":21744,"parent_record_offset":21712,"ancestor_record_offset":21088,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs
//     evidence={"module":448,"record_offset":25708,"parent_record_offset":25676,"ancestor_record_offset":21088,"inlinee":25453,"source_line":616,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs
//     evidence={"module":448,"record_offset":26284,"parent_record_offset":26256,"ancestor_record_offset":21088,"inlinee":25454,"source_line":619,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 508
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":28000,"rva":8379248,"va":5377088368}
// 编译实例 RVAs: 0x7fdb70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4845783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377088368,"raw_hex":"e8d7f04900","size":5,"soft_fail":false,"target_va":5381934528,"va":5377088740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125d6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10877594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377088368,"raw_hex":"e89afaa500","size":5,"soft_fail":false,"target_va":5387966112,"va":5377088513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125de50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10879533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377088368,"raw_hex":"e82d02a600","size":5,"soft_fail":false,"target_va":5387968080,"va":5377088542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34452916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377088368,"raw_hex":"e8b4b50d02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377089375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::graph
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1051806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377088368,"raw_hex":"e89e0c1000","size":5,"soft_fail":false,"target_va":5378140336,"va":5377088525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2222247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377088368,"raw_hex":"e8a7e82100","size":5,"soft_fail":false,"target_va":5379310864,"va":5377088612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast_simple_exclude_wall_id
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t847317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377088368,"raw_hex":"e8d5ed0c00","size":5,"soft_fail":false,"target_va":5377936464,"va":5377089142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":448,"record_offset":28868,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":28352,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":6954,"source_line":221,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":29316,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":28896,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":28496,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":28676,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":29108,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":28768,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::edge_weight
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":448,"record_offset":28208,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":4650,"source_line":525,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":448,"record_offset":28264,"parent_record_offset":28000,"ancestor_record_offset":28000,"inlinee":4972,"source_line":454,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":28316,"parent_record_offset":28292,"ancestor_record_offset":28000,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":28292,"parent_record_offset":28264,"ancestor_record_offset":28000,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":28376,"parent_record_offset":28352,"ancestor_record_offset":28000,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":28708,"parent_record_offset":28676,"ancestor_record_offset":28000,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":28524,"parent_record_offset":28496,"ancestor_record_offset":28000,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":28800,"parent_record_offset":28768,"ancestor_record_offset":28000,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":29140,"parent_record_offset":29108,"ancestor_record_offset":28000,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":29340,"parent_record_offset":29316,"ancestor_record_offset":28000,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":448,"record_offset":28452,"parent_record_offset":28424,"ancestor_record_offset":28000,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":28424,"parent_record_offset":28400,"ancestor_record_offset":28000,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::edge_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":448,"record_offset":28232,"parent_record_offset":28208,"ancestor_record_offset":28000,"inlinee":4651,"source_line":234,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":448,"record_offset":28400,"parent_record_offset":28376,"ancestor_record_offset":28000,"inlinee":10600,"source_line":719,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 616
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x636D
// 首条位置: {"module":448,"inlinee":25453}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":25748,"parent_record_offset":25708,"ancestor_record_offset":25708,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":25960,"parent_record_offset":25936,"ancestor_record_offset":25708,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":25936,"parent_record_offset":25844,"ancestor_record_offset":25708,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":25872,"parent_record_offset":25844,"ancestor_record_offset":25708,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":25844,"parent_record_offset":25820,"ancestor_record_offset":25708,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":25820,"parent_record_offset":25796,"ancestor_record_offset":25708,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":25904,"parent_record_offset":25872,"ancestor_record_offset":25708,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":25772,"parent_record_offset":25748,"ancestor_record_offset":25708,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 619
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x636E
// 首条位置: {"module":448,"inlinee":25454}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":26312,"parent_record_offset":26284,"ancestor_record_offset":26284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":26516,"parent_record_offset":26492,"ancestor_record_offset":26284,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":26492,"parent_record_offset":26396,"ancestor_record_offset":26284,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26428,"parent_record_offset":26396,"ancestor_record_offset":26284,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26396,"parent_record_offset":26368,"ancestor_record_offset":26284,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26368,"parent_record_offset":26340,"ancestor_record_offset":26284,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":26460,"parent_record_offset":26428,"ancestor_record_offset":26284,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":26340,"parent_record_offset":26312,"ancestor_record_offset":26284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::construct_wooden_railing
// 记录形式: compiled_procedure; instances=1
// 源码行: 670
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":29488,"rva":8380304,"va":5377089424}
// 编译实例 RVAs: 0x7fdf90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4844413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377089424,"raw_hex":"e87deb4900","size":5,"soft_fail":false,"target_va":5381934528,"va":5377090110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128d0e0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11071502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377089424,"raw_hex":"e80ef0a800","size":5,"soft_fail":false,"target_va":5388161248,"va":5377089741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34449806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377089424,"raw_hex":"e88ea90d02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377092485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_cols
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t651675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377089424,"raw_hex":"e89bf10900","size":5,"soft_fail":false,"target_va":5377742080,"va":5377090400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4919931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377089424,"raw_hex":"e87b124b00","size":5,"soft_fail":false,"target_va":5382009728,"va":5377089792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":448,"record_offset":29720,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":30384,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":30356,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":30504,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":448,"record_offset":30284,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":31312,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":25309,"source_line":237,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":30588,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":31108,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":33364,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":33132,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":32896,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":30052,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":30228,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$37::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":29848,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":24706,"source_line":1361,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":30532,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$56::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":29964,"parent_record_offset":29488,"ancestor_record_offset":29488,"inlinee":25456,"source_line":1525,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":31432,"parent_record_offset":31400,"ancestor_record_offset":29488,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":31492,"parent_record_offset":31468,"ancestor_record_offset":29488,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":31468,"parent_record_offset":31400,"ancestor_record_offset":29488,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":448,"record_offset":31984,"parent_record_offset":31960,"ancestor_record_offset":29488,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":448,"record_offset":32032,"parent_record_offset":32008,"ancestor_record_offset":29488,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":29488,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":448,"record_offset":31516,"parent_record_offset":31492,"ancestor_record_offset":29488,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":31588,"parent_record_offset":31336,"ancestor_record_offset":29488,"inlinee":20286,"source_line":2065,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":31668,"parent_record_offset":31640,"ancestor_record_offset":29488,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":29900,"parent_record_offset":29872,"ancestor_record_offset":29488,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":30080,"parent_record_offset":30052,"ancestor_record_offset":29488,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":29992,"parent_record_offset":29964,"ancestor_record_offset":29488,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":448,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":29488,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":448,"record_offset":31920,"parent_record_offset":31896,"ancestor_record_offset":29488,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":448,"record_offset":29800,"parent_record_offset":29752,"ancestor_record_offset":29488,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":29488,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":448,"record_offset":30316,"parent_record_offset":30284,"ancestor_record_offset":29488,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":31896,"parent_record_offset":31872,"ancestor_record_offset":29488,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":29752,"parent_record_offset":29720,"ancestor_record_offset":29488,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":448,"record_offset":31872,"parent_record_offset":31804,"ancestor_record_offset":29488,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":31336,"parent_record_offset":31312,"ancestor_record_offset":29488,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":31376,"parent_record_offset":31336,"ancestor_record_offset":29488,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":448,"record_offset":31400,"parent_record_offset":31376,"ancestor_record_offset":29488,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":30672,"parent_record_offset":30640,"ancestor_record_offset":29488,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":30640,"parent_record_offset":30612,"ancestor_record_offset":29488,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":30612,"parent_record_offset":30588,"ancestor_record_offset":29488,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":32996,"parent_record_offset":32960,"ancestor_record_offset":29488,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":32960,"parent_record_offset":32924,"ancestor_record_offset":29488,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":32924,"parent_record_offset":32896,"ancestor_record_offset":29488,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":29872,"parent_record_offset":29848,"ancestor_record_offset":29488,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":33160,"parent_record_offset":33132,"ancestor_record_offset":29488,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":448,"record_offset":31960,"parent_record_offset":31768,"ancestor_record_offset":29488,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":448,"record_offset":31768,"parent_record_offset":31740,"ancestor_record_offset":29488,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":448,"record_offset":30704,"parent_record_offset":30672,"ancestor_record_offset":29488,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::stairs_railings_assemble::add_corner_stone_railing_piece
// 记录形式: compiled_procedure; instances=1
// 源码行: 788
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":36812,"rva":8383408,"va":5377092528}
// 编译实例 RVAs: 0x7febb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140858750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t366707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092528,"raw_hex":"e873980500","size":5,"soft_fail":false,"target_va":5377460048,"va":5377093336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408587c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t366804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092528,"raw_hex":"e8d4980500","size":5,"soft_fail":false,"target_va":5377460160,"va":5377093351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4841665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092528,"raw_hex":"e8c1e04900","size":5,"soft_fail":false,"target_va":5381934528,"va":5377092858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128cdd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11067009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092528,"raw_hex":"e881dea800","size":5,"soft_fail":false,"target_va":5388160464,"va":5377093450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34448425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092528,"raw_hex":"e829a40d02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377093866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_cols
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t648746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092528,"raw_hex":"e82ae60900","size":5,"soft_fail":false,"target_va":5377742080,"va":5377093329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":448,"record_offset":38268,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":37020,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":37080,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":37052,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":37756,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":37480,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":37384,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":13651,"source_line":1297,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":37664,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":448,"record_offset":37972,"parent_record_offset":36812,"ancestor_record_offset":36812,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":37696,"parent_record_offset":37664,"ancestor_record_offset":36812,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":37236,"parent_record_offset":37104,"ancestor_record_offset":36812,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":448,"record_offset":38004,"parent_record_offset":37972,"ancestor_record_offset":36812,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":37168,"parent_record_offset":37136,"ancestor_record_offset":36812,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":37136,"parent_record_offset":37104,"ancestor_record_offset":36812,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":37104,"parent_record_offset":37080,"ancestor_record_offset":36812,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":448,"record_offset":37324,"parent_record_offset":37300,"ancestor_record_offset":36812,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":448,"record_offset":37200,"parent_record_offset":37168,"ancestor_record_offset":36812,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00448.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_railings_assemble.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_railings_assemble.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_railings_assemble.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::add_corner_stone_railing_piece
// 信息 ID: 421cc29855442f14fa460550
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xb73d -> 0x7febb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1ab57 -> 0x7febb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
// 信息 ID: 271778d159f41b4a2dd8f3aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xb2de -> 0x7f98e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xdcd9 -> 0x7f98e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1a6fa -> 0x7f98e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1d106 -> 0x7f98e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$0
// 信息 ID: 6a1ad7fc60432270efe02f65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings; module=448, inline_record=624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$1
// 信息 ID: e08772cb71b8a0ead6a3a941
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings; module=448, inline_record=712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$2
// 信息 ID: acf4b89a5bd0b1b47c717a0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings; module=448, inline_record=1996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$3
// 信息 ID: bec255fa3b3a05416346a87c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings; module=448, inline_record=2348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$4
// 信息 ID: 646d133f1848a227d09e9643
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env$4 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<f32>,system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env$4> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env$4 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<f32>,system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env$4> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<f32>,system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env$4> > > >; module=1546, inline_record=22092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure$5
// 信息 ID: 01c7a2eeedec571dda4d7800
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings; module=448, inline_record=4644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::construct_wooden_railing
// 信息 ID: 323d9eb23d5ae9ae778c2b2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings @ 0x7fb631 -> 0x7fdf90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips @ 0x7fd68e -> 0x7fdf90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::railings_tips
// 信息 ID: 47358f640854e74737303067
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xb81f -> 0x7fcc80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1ac39 -> 0x7fcc80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$0
// 信息 ID: d3ab80dc036dd87561601065
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::railings_tips", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips @ 0x7fcd94 -> 0x7fdb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips @ 0x7fcdb5 -> 0x7fdb70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$1
// 信息 ID: 2018c13d06e8a5af732213d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::railings_tips", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips; module=448, inline_record=25708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::closure$2
// 信息 ID: 2de7d88eca8a12d09c040f06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::stairs_railings_assemble::railings_tips", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_railings_assemble::railings_tips; module=448, inline_record=26284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
