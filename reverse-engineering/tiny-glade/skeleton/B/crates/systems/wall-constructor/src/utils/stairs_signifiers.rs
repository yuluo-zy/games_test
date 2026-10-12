// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/utils/stairs_signifiers.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f66092890d74ee016799067db05796802b24d5bdd35286d89e29d4bcb843da06
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_signifiers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1437, 1452
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1437,"record_offset":272,"rva":19021072,"va":5387730192}
// 编译实例 RVAs: 0x1223d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12648889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e847fe3eff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387730372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12649358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e872fc3eff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387730857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10274552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8083963ff","size":5,"soft_fail":false,"target_va":5377460112,"va":5387734659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9992479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8e18667ff","size":5,"soft_fail":false,"target_va":5377742176,"va":5387734650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c37760
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6213345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e81f31a1ff","size":5,"soft_fail":false,"target_va":5381519200,"va":5387732540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5799281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e88f82a7ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387733804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5798892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e81484a7ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387733447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f86cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2741688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8482ad6ff","size":5,"soft_fail":false,"target_va":5384989904,"va":5387731587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-178295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e88947fdff","size":5,"soft_fail":false,"target_va":5387553808,"va":5387732098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9ea0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-173672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e89859fdff","size":5,"soft_fail":false,"target_va":5387558560,"va":5387732227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fa450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-170538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8d665fdff","size":5,"soft_fail":false,"target_va":5387560016,"va":5387730549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412055b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-125404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e82416feff","size":5,"soft_fail":false,"target_va":5387605424,"va":5387730823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141246fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t142971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e87b2e0200","size":5,"soft_fail":false,"target_va":5387874224,"va":5387731248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24115023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e84ff76f01","size":5,"soft_fail":false,"target_va":5411847776,"va":5387732748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23805219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8233d6b01","size":5,"soft_fail":false,"target_va":5411540992,"va":5387735768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-6955184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e850df95ff","size":5,"soft_fail":false,"target_va":5380775408,"va":5387730587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23806894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8ae436b01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387735165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t23806628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8a4426b01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387735663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23806763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e82b436b01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387735136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23807731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8f3466b01","size":5,"soft_fail":false,"target_va":5411543104,"va":5387735368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_overflow_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23807874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e882476b01","size":5,"soft_fail":false,"target_va":5411543200,"va":5387735321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23807770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e81a476b01","size":5,"soft_fail":false,"target_va":5411543120,"va":5387735345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7314243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8bd6490ff","size":5,"soft_fail":false,"target_va":5380417056,"va":5387731294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7313310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8626890ff","size":5,"soft_fail":false,"target_va":5380417024,"va":5387730329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5810284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e89457a7ff","size":5,"soft_fail":false,"target_va":5381920256,"va":5387730535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5851033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e867b8a6ff","size":5,"soft_fail":false,"target_va":5381881936,"va":5387732964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14384354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8e27cdb00","size":5,"soft_fail":false,"target_va":5402116816,"va":5387732457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14384417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8217ddb00","size":5,"soft_fail":false,"target_va":5402116944,"va":5387732522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5847876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387730192,"raw_hex":"e8bcc4a6ff","size":5,"soft_fail":false,"target_va":5381885904,"va":5387733775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1437,"record_offset":568,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1437,"record_offset":720,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":3404,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":1984,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":804,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":2380,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":2012,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1437,"record_offset":2264,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1437,"record_offset":10440,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1437,"record_offset":6552,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1437,"record_offset":11080,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1437,"record_offset":3032,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1437,"record_offset":2944,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1437,"record_offset":3672,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1437,"record_offset":988,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1437,"record_offset":1664,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1437,"record_offset":3060,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1437,"record_offset":3760,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1437,"record_offset":5872,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7576,"source_line":378,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1437,"record_offset":1748,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1437,"record_offset":1580,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1437,"record_offset":5708,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1437,"record_offset":5900,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1437,"record_offset":3460,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70929,"source_line":429,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1437,"record_offset":9252,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1437,"record_offset":9440,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":9600,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":9468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":9816,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":9164,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1437,"record_offset":6496,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1437,"record_offset":10676,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1437,"record_offset":4240,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1437,"record_offset":1260,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1437,"record_offset":6072,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1437,"record_offset":4868,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1437,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":272,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1437,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":272,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1437,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1437,"record_offset":600,"parent_record_offset":568,"ancestor_record_offset":272,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1437,"record_offset":1912,"parent_record_offset":1820,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1437,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1437,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1437,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1437,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1437,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1437,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1437,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":272,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":844,"parent_record_offset":804,"ancestor_record_offset":272,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":272,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":4324,"parent_record_offset":4268,"ancestor_record_offset":272,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1437,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":272,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1437,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":272,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1437,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":272,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1437,"record_offset":10556,"parent_record_offset":10464,"ancestor_record_offset":272,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1437,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":272,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1437,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":272,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1437,"record_offset":1072,"parent_record_offset":1012,"ancestor_record_offset":272,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1437,"record_offset":10988,"parent_record_offset":10700,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1437,"record_offset":10788,"parent_record_offset":10736,"ancestor_record_offset":272,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1437,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":272,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1437,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":272,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1437,"record_offset":6224,"parent_record_offset":6072,"ancestor_record_offset":272,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1437,"record_offset":6252,"parent_record_offset":6072,"ancestor_record_offset":272,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1437,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":272,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1437,"record_offset":7212,"parent_record_offset":7188,"ancestor_record_offset":272,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1437,"record_offset":6732,"parent_record_offset":6696,"ancestor_record_offset":272,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1437,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1437,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":272,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1437,"record_offset":7236,"parent_record_offset":7212,"ancestor_record_offset":272,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1437,"record_offset":2568,"parent_record_offset":2524,"ancestor_record_offset":272,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1437,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":272,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1437,"record_offset":1880,"parent_record_offset":1848,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1437,"record_offset":4792,"parent_record_offset":4764,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1437,"record_offset":4648,"parent_record_offset":4616,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1437,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1437,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":272,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1437,"record_offset":10500,"parent_record_offset":10464,"ancestor_record_offset":272,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1437,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":272,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1437,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":272,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1437,"record_offset":6112,"parent_record_offset":6072,"ancestor_record_offset":272,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1437,"record_offset":2496,"parent_record_offset":2452,"ancestor_record_offset":272,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1437,"record_offset":1044,"parent_record_offset":1012,"ancestor_record_offset":272,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1437,"record_offset":6912,"parent_record_offset":6732,"ancestor_record_offset":272,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1437,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":272,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1437,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1437,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1437,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":272,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1437,"record_offset":2320,"parent_record_offset":2264,"ancestor_record_offset":272,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1437,"record_offset":2292,"parent_record_offset":2264,"ancestor_record_offset":272,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1437,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":272,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1437,"record_offset":676,"parent_record_offset":648,"ancestor_record_offset":272,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1437,"record_offset":956,"parent_record_offset":804,"ancestor_record_offset":272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1437,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":272,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1437,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1437,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":272,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1437,"record_offset":2596,"parent_record_offset":2524,"ancestor_record_offset":272,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1437,"record_offset":2416,"parent_record_offset":2380,"ancestor_record_offset":272,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1437,"record_offset":2452,"parent_record_offset":2416,"ancestor_record_offset":272,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1437,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":272,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1437,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":272,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1437,"record_offset":2524,"parent_record_offset":2452,"ancestor_record_offset":272,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1437,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1437,"record_offset":5808,"parent_record_offset":5784,"ancestor_record_offset":272,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1437,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1437,"record_offset":5784,"parent_record_offset":5708,"ancestor_record_offset":272,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1437,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":272,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1437,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":272,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1437,"record_offset":11012,"parent_record_offset":10988,"ancestor_record_offset":272,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1437,"record_offset":10736,"parent_record_offset":10700,"ancestor_record_offset":272,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1437,"record_offset":9336,"parent_record_offset":9304,"ancestor_record_offset":272,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1437,"record_offset":9304,"parent_record_offset":9276,"ancestor_record_offset":272,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1437,"record_offset":9276,"parent_record_offset":9252,"ancestor_record_offset":272,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1437,"record_offset":8248,"parent_record_offset":8220,"ancestor_record_offset":272,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1437,"record_offset":8276,"parent_record_offset":8248,"ancestor_record_offset":272,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1437,"record_offset":8360,"parent_record_offset":8248,"ancestor_record_offset":272,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":9744,"parent_record_offset":9624,"ancestor_record_offset":272,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":272,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":4764,"parent_record_offset":4592,"ancestor_record_offset":272,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1437,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1437,"record_offset":7140,"parent_record_offset":6696,"ancestor_record_offset":272,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1437,"record_offset":6696,"parent_record_offset":6664,"ancestor_record_offset":272,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1437,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":272,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1437,"record_offset":9368,"parent_record_offset":9336,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/stairs_signifiers.rs
//     evidence={"module":1437,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":272,"inlinee":70950,"source_line":206,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1437,"record_offset":4268,"parent_record_offset":4240,"ancestor_record_offset":272,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1437,"record_offset":4296,"parent_record_offset":4268,"ancestor_record_offset":272,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1437,"record_offset":4432,"parent_record_offset":4268,"ancestor_record_offset":272,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1437,"record_offset":8220,"parent_record_offset":7940,"ancestor_record_offset":272,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1437,"record_offset":4568,"parent_record_offset":4268,"ancestor_record_offset":272,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11526
// 首条位置: {"module":1437,"inlinee":70950}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1437,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":6960,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1437,"record_offset":7092,"parent_record_offset":6984,"ancestor_record_offset":6960,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1437,"record_offset":7036,"parent_record_offset":6984,"ancestor_record_offset":6960,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01437.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1437,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":6960,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01437.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_signifiers.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_signifiers.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_signifiers.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new
// 信息 ID: bc351d32b230121aeb98505e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x758fb6 -> 0x1223d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new::closure$1
// 信息 ID: ecd523add6916544ed027d0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new; module=1437, inline_record=6960; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new; module=1437, inline_record=7492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
