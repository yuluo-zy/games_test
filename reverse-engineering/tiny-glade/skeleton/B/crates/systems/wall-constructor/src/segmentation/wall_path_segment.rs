// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1bf3f6aaf0ac9da2e1b45445737aba98b216afec712c4e530e397fab7c711fbd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_path_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 396, 402, 1413, 1449
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FEA
// 首条位置: {"module":396,"inlinee":24554}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":396,"record_offset":13396,"parent_record_offset":13344,"ancestor_record_offset":13344,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1449,"record_offset":660,"rva":19070864,"va":5387779984}
// 编译实例 RVAs: 0x122ff90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d3750
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12962355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8cd353aff","size":5,"soft_fail":false,"target_va":5374818128,"va":5387780478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12698967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8a93a3eff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387780466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5893735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e89911a6ff","size":5,"soft_fail":false,"target_va":5381889712,"va":5387783442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5893527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e86912a6ff","size":5,"soft_fail":false,"target_va":5381889888,"va":5387783410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd7e0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-212409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e847c2fcff","size":5,"soft_fail":false,"target_va":5387573216,"va":5387785620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412002f0
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-197115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e805fefcff","size":5,"soft_fail":false,"target_va":5387584240,"va":5387781350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141203d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-183853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8d331fdff","size":5,"soft_fail":false,"target_va":5387599152,"va":5387783000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412052e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-178209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8df47fdff","size":5,"soft_fail":false,"target_va":5387604704,"va":5387782908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141205830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-174851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8fd54fdff","size":5,"soft_fail":false,"target_va":5387606064,"va":5387780910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141208d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-163489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e85f81fdff","size":5,"soft_fail":false,"target_va":5387619600,"va":5387783084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141208e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-162578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8ee84fdff","size":5,"soft_fail":false,"target_va":5387619856,"va":5387782429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7004949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8eb1c95ff","size":5,"soft_fail":false,"target_va":5380775408,"va":5387780352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23757745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8b1836a01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387784314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=20
//     evidence={"assembly":"\tcallq\t23757791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8df836a01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387784500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23758739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e893876a01","size":5,"soft_fail":false,"target_va":5411543584,"va":5387784840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23758837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8f5876a01","size":5,"soft_fail":false,"target_va":5411543456,"va":5387784614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23757621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e835836a01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387784278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopIdx::index
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-197274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e866fdfcff","size":5,"soft_fail":false,"target_va":5387584128,"va":5387781397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-199503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e8b1f4fcff","size":5,"soft_fail":false,"target_va":5387584352,"va":5387783850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::subslice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-199055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387779984,"raw_hex":"e871f6fcff","size":5,"soft_fail":false,"target_va":5387584784,"va":5387783834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1449,"record_offset":916,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1449,"record_offset":1264,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1449,"record_offset":6656,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1449,"record_offset":6848,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1449,"record_offset":4372,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1449,"record_offset":8344,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1449,"record_offset":1084,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1449,"record_offset":4400,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1449,"record_offset":2640,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1449,"record_offset":2612,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1449,"record_offset":2528,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1449,"record_offset":6544,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":28998,"source_line":3362,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1449,"record_offset":4288,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1449,"record_offset":6320,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":19714,"source_line":35,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1449,"record_offset":1472,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1449,"record_offset":1000,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1449,"record_offset":4204,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1449,"record_offset":1992,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
//     evidence={"module":1449,"record_offset":6944,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":70998,"source_line":126,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1449,"record_offset":944,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":24417,"source_line":175,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] utils::slice_iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1449,"record_offset":2472,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":70994,"source_line":674,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1449,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1449,"record_offset":1636,"parent_record_offset":1544,"ancestor_record_offset":660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1449,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1449,"record_offset":1572,"parent_record_offset":1544,"ancestor_record_offset":660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1449,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1449,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1449,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1449,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1449,"record_offset":1300,"parent_record_offset":1264,"ancestor_record_offset":660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1449,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":660,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1449,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":660,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1449,"record_offset":7332,"parent_record_offset":7308,"ancestor_record_offset":660,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1449,"record_offset":3096,"parent_record_offset":3064,"ancestor_record_offset":660,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1449,"record_offset":7372,"parent_record_offset":7220,"ancestor_record_offset":660,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1449,"record_offset":7400,"parent_record_offset":7220,"ancestor_record_offset":660,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1449,"record_offset":2748,"parent_record_offset":2696,"ancestor_record_offset":660,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1449,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":660,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1449,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":660,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::Copied::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1449,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":660,"inlinee":29000,"source_line":24,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1449,"record_offset":2696,"parent_record_offset":2640,"ancestor_record_offset":660,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1449,"record_offset":4812,"parent_record_offset":4592,"ancestor_record_offset":660,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1449,"record_offset":4312,"parent_record_offset":4288,"ancestor_record_offset":660,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1449,"record_offset":4444,"parent_record_offset":4400,"ancestor_record_offset":660,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1449,"record_offset":4336,"parent_record_offset":4312,"ancestor_record_offset":660,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1449,"record_offset":4488,"parent_record_offset":4444,"ancestor_record_offset":660,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1449,"record_offset":3128,"parent_record_offset":3064,"ancestor_record_offset":660,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1449,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":660,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1449,"record_offset":3036,"parent_record_offset":3000,"ancestor_record_offset":660,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1449,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":660,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1449,"record_offset":3000,"parent_record_offset":2960,"ancestor_record_offset":660,"inlinee":14173,"source_line":296,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1449,"record_offset":2960,"parent_record_offset":2696,"ancestor_record_offset":660,"inlinee":9207,"source_line":347,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1449,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":660,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1449,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1449,"record_offset":2876,"parent_record_offset":2696,"ancestor_record_offset":660,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1449,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":660,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1449,"record_offset":1604,"parent_record_offset":1572,"ancestor_record_offset":660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1449,"record_offset":7052,"parent_record_offset":7008,"ancestor_record_offset":660,"inlinee":24425,"source_line":3058,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1449,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":660,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1449,"record_offset":7284,"parent_record_offset":7260,"ancestor_record_offset":660,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1449,"record_offset":7260,"parent_record_offset":7220,"ancestor_record_offset":660,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1449,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":660,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1449,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":660,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1449,"record_offset":5592,"parent_record_offset":4592,"ancestor_record_offset":660,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1449,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1449,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":660,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1449,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":660,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1449,"record_offset":1144,"parent_record_offset":1108,"ancestor_record_offset":660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1449,"record_offset":1412,"parent_record_offset":1264,"ancestor_record_offset":660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1449,"record_offset":6684,"parent_record_offset":6656,"ancestor_record_offset":660,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1449,"record_offset":6876,"parent_record_offset":6848,"ancestor_record_offset":660,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1449,"record_offset":6712,"parent_record_offset":6684,"ancestor_record_offset":660,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1449,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":660,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1449,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":660,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1449,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":660,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1449,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":660,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1449,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":660,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1449,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1449,"record_offset":1172,"parent_record_offset":1108,"ancestor_record_offset":660,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1449,"record_offset":4648,"parent_record_offset":4592,"ancestor_record_offset":660,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1449,"record_offset":4724,"parent_record_offset":4648,"ancestor_record_offset":660,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1449,"record_offset":2176,"parent_record_offset":1992,"ancestor_record_offset":660,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1449,"record_offset":7192,"parent_record_offset":6944,"ancestor_record_offset":660,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1449,"record_offset":7220,"parent_record_offset":6944,"ancestor_record_offset":660,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$9::iterator_slice_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1449,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":660,"inlinee":70996,"source_line":661,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$6::next
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1449,"record_offset":2776,"parent_record_offset":2696,"ancestor_record_offset":660,"inlinee":70773,"source_line":212,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$6::next::closure$0
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1449,"record_offset":2836,"parent_record_offset":2804,"ancestor_record_offset":660,"inlinee":70775,"source_line":213,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1449,"record_offset":7008,"parent_record_offset":6944,"ancestor_record_offset":660,"inlinee":24424,"source_line":226,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11556
// 首条位置: {"module":1449,"inlinee":70998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1449,"record_offset":7192,"parent_record_offset":6944,"ancestor_record_offset":6944,"inlinee":14137,"source_line":25,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1449,"record_offset":7220,"parent_record_offset":6944,"ancestor_record_offset":6944,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1449,"record_offset":7008,"parent_record_offset":6944,"ancestor_record_offset":6944,"inlinee":24424,"source_line":226,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1449,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":6944,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1449,"record_offset":7332,"parent_record_offset":7308,"ancestor_record_offset":6944,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1449,"record_offset":7372,"parent_record_offset":7220,"ancestor_record_offset":6944,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1449,"record_offset":7400,"parent_record_offset":7220,"ancestor_record_offset":6944,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1449,"record_offset":7052,"parent_record_offset":7008,"ancestor_record_offset":6944,"inlinee":24425,"source_line":3058,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1449,"record_offset":7284,"parent_record_offset":7260,"ancestor_record_offset":6944,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01449.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1449,"record_offset":7260,"parent_record_offset":7220,"ancestor_record_offset":6944,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01449.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_path_segment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_path_segment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_path_segment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
// 信息 ID: d041e4b047847ea65f3afb63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=396, inline_record=13344; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=402, inline_record=11160; 不是独立调用边。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2>; module=1413, inline_record=744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl
// 信息 ID: 5edca07565fad258bd6c58c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760288 -> 0x122ff90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76e39f -> 0x122ff90 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2> @ 0x11ffa3d -> 0x122ff90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl::closure$0
// 信息 ID: 411681c201f2b71aeb983df7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1449, inline_record=6944; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1449, inline_record=7532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
