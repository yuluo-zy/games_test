// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/mei/layout.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3e8b9f4db6d098b06638fc834cec7d21184f9341036c8721ee6acb364ee8053c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: layout.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 31, 335, 341, 342, 1181, 1224, 2778, 2820
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":2368,"rva":16649744,"va":5385358864}
// 编译实例 RVAs: 0xfe0e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10682904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385358864,"raw_hex":"e8e8fd5cff","size":5,"soft_fail":false,"target_va":5374676336,"va":5385359235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26182197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385358864,"raw_hex":"e835828f01","size":5,"soft_fail":false,"target_va":5411541808,"va":5385359606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1224,"record_offset":4580,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1224,"record_offset":3220,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1224,"record_offset":2776,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":30977,"source_line":174,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":2952,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1224,"record_offset":6652,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1224,"record_offset":6564,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":2748,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":20148,"source_line":386,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points_or_zero
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6188,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":20171,"source_line":412,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6424,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":20153,"source_line":116,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":5996,"parent_record_offset":2368,"ancestor_record_offset":2368,"inlinee":20151,"source_line":402,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1224,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":2368,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1224,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":2368,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1224,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":2368,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1224,"record_offset":6680,"parent_record_offset":6652,"ancestor_record_offset":2368,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1224,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":2368,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1224,"record_offset":4300,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1224,"record_offset":2996,"parent_record_offset":2952,"ancestor_record_offset":2368,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1224,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2368,"inlinee":62294,"source_line":143,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1224,"record_offset":3972,"parent_record_offset":3944,"ancestor_record_offset":2368,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1224,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":2368,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1224,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":2368,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1224,"record_offset":3100,"parent_record_offset":2996,"ancestor_record_offset":2368,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1224,"record_offset":3344,"parent_record_offset":3248,"ancestor_record_offset":2368,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1224,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":2368,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1224,"record_offset":3376,"parent_record_offset":3220,"ancestor_record_offset":2368,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1224,"record_offset":2904,"parent_record_offset":2880,"ancestor_record_offset":2368,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1224,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":2368,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1224,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2368,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1224,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":2368,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1224,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":2368,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1224,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":2368,"inlinee":9857,"source_line":141,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1224,"record_offset":3036,"parent_record_offset":2996,"ancestor_record_offset":2368,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1224,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":2368,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1224,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":2368,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1224,"record_offset":3916,"parent_record_offset":3884,"ancestor_record_offset":2368,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1224,"record_offset":3856,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1224,"record_offset":3716,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1224,"record_offset":4328,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1224,"record_offset":3800,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1224,"record_offset":4480,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1224,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":2368,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1224,"record_offset":3536,"parent_record_offset":3504,"ancestor_record_offset":2368,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1224,"record_offset":3568,"parent_record_offset":3536,"ancestor_record_offset":2368,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1224,"record_offset":4184,"parent_record_offset":4016,"ancestor_record_offset":2368,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1224,"record_offset":4520,"parent_record_offset":3568,"ancestor_record_offset":2368,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":4124,"parent_record_offset":4040,"ancestor_record_offset":2368,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":2368,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":4420,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":2368,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2368,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":3604,"parent_record_offset":3568,"ancestor_record_offset":2368,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":3660,"parent_record_offset":3632,"ancestor_record_offset":2368,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":4452,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1224,"record_offset":4016,"parent_record_offset":3660,"ancestor_record_offset":2368,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1224,"record_offset":3504,"parent_record_offset":3472,"ancestor_record_offset":2368,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1224,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2368,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$17::perform_layout::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1224,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2368,"inlinee":61811,"source_line":880,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":2368,"inlinee":20148,"source_line":386,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6448,"parent_record_offset":6424,"ancestor_record_offset":2368,"inlinee":20143,"source_line":85,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6504,"parent_record_offset":6424,"ancestor_record_offset":2368,"inlinee":20142,"source_line":81,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6228,"parent_record_offset":6188,"ancestor_record_offset":2368,"inlinee":20151,"source_line":402,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":12524,"rva":16655008,"va":5385364128}
// 编译实例 RVAs: 0xfe22a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26242916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364128,"raw_hex":"e8646f9001","size":5,"soft_fail":false,"target_va":5411607536,"va":5385364615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e3f880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15062454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364128,"raw_hex":"e8b6d5e500","size":5,"soft_fail":false,"target_va":5400426624,"va":5385364165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e76060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15285594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364128,"raw_hex":"e85a3de900","size":5,"soft_fail":false,"target_va":5400649824,"va":5385364225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e76160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15285577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364128,"raw_hex":"e8493de900","size":5,"soft_fail":false,"target_va":5400650080,"va":5385364498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364128,"raw_hex":"e868e7ffff","size":5,"soft_fail":false,"target_va":5385358160,"va":5385364451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-5892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364128,"raw_hex":"e8fce8ffff","size":5,"soft_fail":false,"target_va":5385358368,"va":5385364255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":13064,"parent_record_offset":12524,"ancestor_record_offset":12524,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12692,"parent_record_offset":12524,"ancestor_record_offset":12524,"inlinee":62296,"source_line":732,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1224,"record_offset":12888,"parent_record_offset":12856,"ancestor_record_offset":12524,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":12940,"parent_record_offset":12692,"ancestor_record_offset":12524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":12856,"parent_record_offset":12692,"ancestor_record_offset":12524,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":12792,"parent_record_offset":12692,"ancestor_record_offset":12524,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1224,"record_offset":12736,"parent_record_offset":12692,"ancestor_record_offset":12524,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1224,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":12524,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1224,"record_offset":12828,"parent_record_offset":12692,"ancestor_record_offset":12524,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12908,"parent_record_offset":12888,"ancestor_record_offset":12524,"inlinee":62299,"source_line":710,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12968,"parent_record_offset":12692,"ancestor_record_offset":12524,"inlinee":62296,"source_line":732,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$26::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":13136,"parent_record_offset":13112,"ancestor_record_offset":12524,"inlinee":20175,"source_line":721,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":11852,"rva":16654480,"va":5385363600}
// 编译实例 RVAs: 0xfe2090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26243598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385363600,"raw_hex":"e80e729001","size":5,"soft_fail":false,"target_va":5411607536,"va":5385363933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e76060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15286150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385363600,"raw_hex":"e8863fe900","size":5,"soft_fail":false,"target_va":5400649824,"va":5385363669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e76160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15286279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385363600,"raw_hex":"e80740e900","size":5,"soft_fail":false,"target_va":5400650080,"va":5385363796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e769d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15288601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385363600,"raw_hex":"e81949e900","size":5,"soft_fail":false,"target_va":5400652240,"va":5385363634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385363600,"raw_hex":"e826eaffff","size":5,"soft_fail":false,"target_va":5385358160,"va":5385363749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385363600,"raw_hex":"e8b4eaffff","size":5,"soft_fail":false,"target_va":5385358368,"va":5385363815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":12428,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12068,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":62296,"source_line":732,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1224,"record_offset":12252,"parent_record_offset":12224,"ancestor_record_offset":11852,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":12304,"parent_record_offset":12068,"ancestor_record_offset":11852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":12224,"parent_record_offset":12068,"ancestor_record_offset":11852,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":12168,"parent_record_offset":12068,"ancestor_record_offset":11852,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1224,"record_offset":12112,"parent_record_offset":12068,"ancestor_record_offset":11852,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1224,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":11852,"inlinee":62297,"source_line":2061,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1224,"record_offset":12196,"parent_record_offset":12068,"ancestor_record_offset":11852,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12272,"parent_record_offset":12252,"ancestor_record_offset":11852,"inlinee":62299,"source_line":710,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12332,"parent_record_offset":12068,"ancestor_record_offset":11852,"inlinee":62296,"source_line":732,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$26::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12484,"parent_record_offset":12460,"ancestor_record_offset":11852,"inlinee":20175,"source_line":721,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":13176,"rva":16655712,"va":5385364832}
// 编译实例 RVAs: 0xfe2560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26242126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364832,"raw_hex":"e84e6c9001","size":5,"soft_fail":false,"target_va":5411607536,"va":5385365405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe2090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364832,"raw_hex":"e89dfaffff","size":5,"soft_fail":false,"target_va":5385363600,"va":5385364974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e3f880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15061723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364832,"raw_hex":"e8dbd2e500","size":5,"soft_fail":false,"target_va":5400426624,"va":5385364896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e761f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15285067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364832,"raw_hex":"e84b3be900","size":5,"soft_fail":false,"target_va":5400650224,"va":5385365152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364832,"raw_hex":"e8bee4ffff","size":5,"soft_fail":false,"target_va":5385358160,"va":5385365133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-6905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385364832,"raw_hex":"e807e5ffff","size":5,"soft_fail":false,"target_va":5385358368,"va":5385365268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":13680,"parent_record_offset":13176,"ancestor_record_offset":13176,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":13392,"parent_record_offset":13176,"ancestor_record_offset":13176,"inlinee":62296,"source_line":732,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1224,"record_offset":13720,"parent_record_offset":13680,"ancestor_record_offset":13176,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":13556,"parent_record_offset":13392,"ancestor_record_offset":13176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":13520,"parent_record_offset":13392,"ancestor_record_offset":13176,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":13484,"parent_record_offset":13392,"ancestor_record_offset":13176,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1224,"record_offset":13428,"parent_record_offset":13392,"ancestor_record_offset":13176,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1224,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":13176,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$190::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1224,"record_offset":13584,"parent_record_offset":13392,"ancestor_record_offset":13176,"inlinee":6498,"source_line":1398,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$27::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":13176,"inlinee":62301,"source_line":746,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::width
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EAE
// 首条位置: {"module":341,"inlinee":20142}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1224,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":6504,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::height
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EAF
// 首条位置: {"module":341,"inlinee":20143}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1224,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::size_vec2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EB9
// 首条位置: {"module":1224,"inlinee":20153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6448,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":20143,"source_line":85,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6504,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":20142,"source_line":81,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1224,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":6424,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$11::from_points
// 记录形式: inlinee_declaration; instances=1
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF354
// 首条位置: {"module":1224,"inlinee":62292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$13::from_points
// 记录形式: inlinee_declaration; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF352
// 首条位置: {"module":1224,"inlinee":62290}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Axis::is_horizontal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4E1C
// 首条位置: {"module":335,"inlinee":19996}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 227, 237, 707, 714, 732
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF358
// 首条位置: {"module":1224,"inlinee":62296}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":12304,"parent_record_offset":12068,"ancestor_record_offset":12068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":12224,"parent_record_offset":12068,"ancestor_record_offset":12068,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":12168,"parent_record_offset":12068,"ancestor_record_offset":12068,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1224,"record_offset":12112,"parent_record_offset":12068,"ancestor_record_offset":12068,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$190::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1224,"record_offset":13584,"parent_record_offset":13392,"ancestor_record_offset":13392,"inlinee":6498,"source_line":1398,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1224,"record_offset":12196,"parent_record_offset":12068,"ancestor_record_offset":12068,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12332,"parent_record_offset":12068,"ancestor_record_offset":12068,"inlinee":62296,"source_line":732,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1224,"record_offset":12252,"parent_record_offset":12224,"ancestor_record_offset":12068,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1224,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":12068,"inlinee":62297,"source_line":2061,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1224,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":12692,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":12272,"parent_record_offset":12252,"ancestor_record_offset":12068,"inlinee":62299,"source_line":710,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 227, 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":13796,"rva":16656608,"va":5385365728}
// 编译实例 RVAs: 0xfe28e0, 0xfe29e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26115187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385365728,"raw_hex":"e8737c8e01","size":5,"soft_fail":false,"target_va":5411481152,"va":5385365960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1224,"record_offset":13972,"parent_record_offset":13796,"ancestor_record_offset":13796,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1224,"record_offset":13996,"parent_record_offset":13972,"ancestor_record_offset":13796,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1224,"record_offset":14020,"parent_record_offset":13996,"ancestor_record_offset":13796,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1224,"record_offset":14044,"parent_record_offset":14020,"ancestor_record_offset":13796,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 227, 237, 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186DD
// 首条位置: {"module":2778,"inlinee":100061}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":9324,"parent_record_offset":9300,"ancestor_record_offset":9300,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":2820,"record_offset":564,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2820,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":536,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2820,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":536,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2820,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":536,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2820,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":536,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2820,"record_offset":672,"parent_record_offset":564,"ancestor_record_offset":536,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2820,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":536,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2820,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":536,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2820,"record_offset":1028,"parent_record_offset":564,"ancestor_record_offset":536,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2820,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":536,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":9360,"parent_record_offset":9324,"ancestor_record_offset":9300,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":9424,"parent_record_offset":9324,"ancestor_record_offset":9300,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2820,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":536,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2820,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":536,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2820,"record_offset":888,"parent_record_offset":868,"ancestor_record_offset":536,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":9396,"parent_record_offset":9324,"ancestor_record_offset":9300,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2820,"record_offset":620,"parent_record_offset":564,"ancestor_record_offset":536,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":2820,"record_offset":868,"parent_record_offset":564,"ancestor_record_offset":536,"inlinee":62307,"source_line":714,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":2820,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":536,"inlinee":62288,"source_line":714,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 227, 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF364
// 首条位置: {"module":2778,"inlinee":62308}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":58836,"parent_record_offset":58780,"ancestor_record_offset":58780,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":58808,"parent_record_offset":58780,"ancestor_record_offset":58780,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$16::default
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 278
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EBC
// 首条位置: {"module":341,"record_offset":256,"rva":6875216,"va":5375584336}
// 编译实例 RVAs: 0x68e850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::horizontal
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":372,"rva":6875264,"va":5375584384}
// 编译实例 RVAs: 0x68e880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LayoutStyle::horizontal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":341,"record_offset":488,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":20146,"source_line":301,"archive":"_pdb/B/modules/00341.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::horizontal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EB2
// 首条位置: {"module":341,"inlinee":20146}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::vertical
// 记录形式: inlinee_declaration; instances=1
// 源码行: 306
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EB1
// 首条位置: {"module":341,"inlinee":20145}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::gap
// 记录形式: compiled_procedure; instances=1
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":516,"rva":6875312,"va":5375584432}
// 编译实例 RVAs: 0x68e8b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::padding
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EC0
// 首条位置: {"module":342,"inlinee":20160}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::expand
// 记录形式: compiled_procedure; instances=1
// 源码行: 321
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":632,"rva":6875376,"va":5375584496}
// 编译实例 RVAs: 0x68e8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::space_between
// 记录形式: compiled_procedure; instances=1
// 源码行: 326
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":752,"rva":6875440,"va":5375584560}
// 编译实例 RVAs: 0x68e930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::align_center
// 记录形式: compiled_procedure; instances=1
// 源码行: 336
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":876,"rva":6875488,"va":5375584608}
// 编译实例 RVAs: 0x68e960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::xalign_center
// 记录形式: compiled_procedure; instances=1
// 源码行: 353
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":1000,"rva":6875536,"va":5375584656}
// 编译实例 RVAs: 0x68e990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::size
// 记录形式: compiled_procedure; instances=1
// 源码行: 365
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":1124,"rva":6875584,"va":5375584704}
// 编译实例 RVAs: 0x68e9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::ComputedLayout::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 380
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4E18
// 首条位置: {"module":335,"inlinee":19992}
// 编译实例 RVAs: 0x68ea00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":335,"record_offset":84168,"parent_record_offset":84144,"ancestor_record_offset":84144,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":335,"record_offset":84224,"parent_record_offset":84144,"ancestor_record_offset":84144,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":335,"record_offset":84248,"parent_record_offset":84224,"ancestor_record_offset":84144,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":335,"record_offset":84192,"parent_record_offset":84168,"ancestor_record_offset":84144,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentage::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 386
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EB4
// 首条位置: {"module":341,"inlinee":20148}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentage::points
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 393
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EB5
// 首条位置: {"module":341,"inlinee":20149}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Size::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 402
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4EB7
// 首条位置: {"module":341,"inlinee":20151}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":5996,"inlinee":20148,"source_line":386,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Size::points
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 406
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4ECA
// 首条位置: {"module":1224,"inlinee":20170}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":8128,"parent_record_offset":8104,"ancestor_record_offset":8104,"inlinee":20149,"source_line":393,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 412
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4ECB
// 首条位置: {"module":1224,"inlinee":20171}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6228,"parent_record_offset":6188,"ancestor_record_offset":6188,"inlinee":20151,"source_line":402,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":6256,"parent_record_offset":6228,"ancestor_record_offset":6188,"inlinee":20148,"source_line":386,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::points
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4ECC
// 首条位置: {"module":1224,"inlinee":20172}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":7500,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":20149,"source_line":393,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen
// 记录形式: compiled_procedure; instances=1
// 源码行: 476
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":341,"record_offset":1528,"rva":6875680,"va":5375584800}
// 编译实例 RVAs: 0x68ea20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe12a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9775185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375584800,"raw_hex":"e851289500","size":5,"soft_fail":false,"target_va":5385360032,"va":5375584842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe19a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9776992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375584800,"raw_hex":"e8602f9500","size":5,"soft_fail":false,"target_va":5385361824,"va":5375584827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>
// 记录形式: compiled_procedure; instances=1
// 源码行: 487
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":6832,"rva":16650912,"va":5385360032}
// 编译实例 RVAs: 0xfe12a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10278725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385360032,"raw_hex":"e8bb2863ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5385360208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10280225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385360032,"raw_hex":"e8df2263ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385361724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9782279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385360032,"raw_hex":"e8f9bb6aff","size":5,"soft_fail":false,"target_va":5375579088,"va":5385361362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26179378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385360032,"raw_hex":"e832778f01","size":5,"soft_fail":false,"target_va":5411540963,"va":5385361580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1224,"record_offset":8492,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1224,"record_offset":7052,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1224,"record_offset":8384,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":7800,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":9048,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1224,"record_offset":7884,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1224,"record_offset":7992,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1224,"record_offset":8216,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1224,"record_offset":7716,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":7772,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":20149,"source_line":393,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":7448,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":20172,"source_line":427,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":7632,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":20153,"source_line":116,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":8104,"parent_record_offset":6832,"ancestor_record_offset":6832,"inlinee":20170,"source_line":406,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":6832,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":6832,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":6832,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":7344,"parent_record_offset":7168,"ancestor_record_offset":6832,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":9212,"parent_record_offset":9120,"ancestor_record_offset":6832,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":6832,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":6832,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":6832,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":6832,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":6832,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":6832,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":7168,"parent_record_offset":7136,"ancestor_record_offset":6832,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":7136,"parent_record_offset":7108,"ancestor_record_offset":6832,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":9096,"parent_record_offset":9072,"ancestor_record_offset":6832,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":7204,"parent_record_offset":7168,"ancestor_record_offset":6832,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1224,"record_offset":8544,"parent_record_offset":8492,"ancestor_record_offset":6832,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1224,"record_offset":7080,"parent_record_offset":7052,"ancestor_record_offset":6832,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1224,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":6832,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1224,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":6832,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1224,"record_offset":8016,"parent_record_offset":7992,"ancestor_record_offset":6832,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":6832,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":6832,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":9180,"parent_record_offset":9148,"ancestor_record_offset":6832,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1224,"record_offset":7680,"parent_record_offset":7656,"ancestor_record_offset":6832,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1224,"record_offset":7740,"parent_record_offset":7716,"ancestor_record_offset":6832,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":6832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1224,"record_offset":7940,"parent_record_offset":7884,"ancestor_record_offset":6832,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":8656,"parent_record_offset":8492,"ancestor_record_offset":6832,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":7500,"parent_record_offset":7448,"ancestor_record_offset":6832,"inlinee":20149,"source_line":393,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":6832,"inlinee":20142,"source_line":81,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>
// 记录形式: compiled_procedure; instances=1
// 源码行: 487
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":9344,"rva":16652704,"va":5385361824}
// 编译实例 RVAs: 0xfe19a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10280517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385361824,"raw_hex":"e8bb2163ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5385362000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10282001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385361824,"raw_hex":"e8ef1b63ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385363500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9784118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385361824,"raw_hex":"e8cab46aff","size":5,"soft_fail":false,"target_va":5375579088,"va":5385363201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26177603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385361824,"raw_hex":"e843708f01","size":5,"soft_fail":false,"target_va":5411540963,"va":5385363355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1224,"record_offset":11000,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1224,"record_offset":9564,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1224,"record_offset":10892,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":10312,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":11556,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1224,"record_offset":10396,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1224,"record_offset":10500,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1224,"record_offset":10724,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1224,"record_offset":10228,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":10284,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":20149,"source_line":393,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":9960,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":20172,"source_line":427,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":10144,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":20153,"source_line":116,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":10612,"parent_record_offset":9344,"ancestor_record_offset":9344,"inlinee":20170,"source_line":406,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":9344,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":9344,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":9344,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":9856,"parent_record_offset":9680,"ancestor_record_offset":9344,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":11720,"parent_record_offset":11628,"ancestor_record_offset":9344,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":9344,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":9620,"parent_record_offset":9592,"ancestor_record_offset":9344,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":11656,"parent_record_offset":11628,"ancestor_record_offset":9344,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":11628,"parent_record_offset":11604,"ancestor_record_offset":9344,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":11124,"parent_record_offset":11100,"ancestor_record_offset":9344,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":11100,"parent_record_offset":11076,"ancestor_record_offset":9344,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":9680,"parent_record_offset":9648,"ancestor_record_offset":9344,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":9648,"parent_record_offset":9620,"ancestor_record_offset":9344,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":9344,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":9716,"parent_record_offset":9680,"ancestor_record_offset":9344,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1224,"record_offset":11052,"parent_record_offset":11000,"ancestor_record_offset":9344,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1224,"record_offset":9592,"parent_record_offset":9564,"ancestor_record_offset":9344,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1224,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":9344,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1224,"record_offset":9764,"parent_record_offset":9740,"ancestor_record_offset":9344,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1224,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":9344,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":9788,"parent_record_offset":9764,"ancestor_record_offset":9344,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":9812,"parent_record_offset":9788,"ancestor_record_offset":9344,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1224,"record_offset":11688,"parent_record_offset":11656,"ancestor_record_offset":9344,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1224,"record_offset":10192,"parent_record_offset":10168,"ancestor_record_offset":9344,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1224,"record_offset":10252,"parent_record_offset":10228,"ancestor_record_offset":9344,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":9344,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1224,"record_offset":10452,"parent_record_offset":10396,"ancestor_record_offset":9344,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":11164,"parent_record_offset":11000,"ancestor_record_offset":9344,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":10012,"parent_record_offset":9960,"ancestor_record_offset":9344,"inlinee":20149,"source_line":393,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1224,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":9344,"inlinee":20143,"source_line":85,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 710
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF35B
// 首条位置: {"module":1224,"inlinee":62299}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4ED1
// 首条位置: {"module":1224,"inlinee":20177}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":15228,"rva":16657072,"va":5385366192}
// 编译实例 RVAs: 0xfe2ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26114799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385366192,"raw_hex":"e8ef7a8e01","size":5,"soft_fail":false,"target_va":5411481152,"va":5385366348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10284920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385366192,"raw_hex":"e8881063ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385366419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::__private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2712892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385366192,"raw_hex":"e8c49ad6ff","size":5,"soft_fail":false,"target_va":5382653424,"va":5385366311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":15408,"parent_record_offset":15228,"ancestor_record_offset":15228,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":15576,"parent_record_offset":15552,"ancestor_record_offset":15228,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1224,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":15228,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":15528,"parent_record_offset":15504,"ancestor_record_offset":15228,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1224,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":15228,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1224,"record_offset":15432,"parent_record_offset":15408,"ancestor_record_offset":15228,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF363
// 首条位置: {"module":2820,"inlinee":62307}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2820,"record_offset":888,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2820,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":868,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2820,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":868,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2820,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":868,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02820.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF350
// 首条位置: {"module":1224,"inlinee":62288}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1224,"record_offset":15860,"rva":16657392,"va":5385366512}
// 编译实例 RVAs: 0xfe2bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26113624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385366512,"raw_hex":"e858768e01","size":5,"soft_fail":false,"target_va":5411480384,"va":5385366755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e75ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15282843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385366512,"raw_hex":"e89b32e900","size":5,"soft_fail":false,"target_va":5400649376,"va":5385366528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e76060
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15283251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385366512,"raw_hex":"e83334e900","size":5,"soft_fail":false,"target_va":5400649824,"va":5385366568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":16364,"parent_record_offset":15860,"ancestor_record_offset":15860,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":16136,"parent_record_offset":15860,"ancestor_record_offset":15860,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1224,"record_offset":16400,"parent_record_offset":15860,"ancestor_record_offset":15860,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1224,"record_offset":16108,"parent_record_offset":15860,"ancestor_record_offset":15860,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1224,"record_offset":16172,"parent_record_offset":15860,"ancestor_record_offset":15860,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$26::newtype_variant_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1224,"record_offset":16200,"parent_record_offset":16172,"ancestor_record_offset":15860,"inlinee":62303,"source_line":2142,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1224,"record_offset":16300,"parent_record_offset":16264,"ancestor_record_offset":15860,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1224,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":15860,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01224.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$26::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 721
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4ECF
// 首条位置: {"module":1224,"inlinee":20175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$27::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 746
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF35D
// 首条位置: {"module":1224,"inlinee":62301}
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
// 完整结构及原始类型记录: layout.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Align
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Align>", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Align"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Align> >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Align>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Axis
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::ComputedLayout
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["toolbar::mei::layout::ComputedLayout", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LayoutStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 17; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >", "constructor": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleXY"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleYX"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LayoutTemp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<toolbar::mei::layout::LayoutTemp,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["toolbar::mei::layout::LayoutTemp", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["country_core::systems::ui::GlobalId", "toolbar::mei::layout::LayoutTemp", "tuple$<>", "toolbar::mei::core::impl$17::perform_layout::closure_env$1", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<country_core::systems::ui::GlobalId>,country_core::systems::ui::GlobalId,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > > > >", "constructor": "core::iter::adapters::copied::impl$1::fold", "arguments": ["core::slice::iter::Iter<country_core::systems::ui::GlobalId>", "country_core::systems::ui::GlobalId", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<country_core::systems::ui::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$17::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$20::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$1> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LengthPercentage
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "enum2$<toolbar::mei::layout::LengthPercentage>", "constructor": "enum2$", "arguments": ["toolbar::mei::layout::LengthPercentage"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >", "constructor": "toolbar::mei::layout::_::impl$0::deserialize", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>", "ref_mut$<ron::de::Deserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "toolbar::mei::layout::Rect", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "serde::de::impls::impl$190::deserialize::TupleVisitor", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "serde::__private::de::content::impl$21::deserialize_tuple", "arguments": ["enum2$<ron::error::Error>", "serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "serde::de::impl$1::fmt", "arguments": ["serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LengthPercentageDeser::Naked
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Rect
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "toolbar::mei::layout::Rect", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Size
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::SwizzleXY
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleXY"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::SwizzleYX
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleYX"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::XAlign
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::XAlign>", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::XAlign"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::XAlign> >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::XAlign>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: layout.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: layout.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Axis::is_horizontal
// 信息 ID: a4cdd8f5f0593fc335ac1f65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::mei_post_update; module=335, inline_record=86308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::ComputedLayout::center
// 信息 ID: b81aa45279eb203a0c3d1f43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x505b68 -> 0x68ea00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::core::mei_post_update; module=335, inline_record=84144; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::mei_post_update; module=335, inline_record=92112; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::iter_subtree<tuple$<>,toolbar::mei::core::update_nav::closure_env$2>; module=1181, inline_record=176008; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::align_center
// 信息 ID: 82f3577534fa14ac698cab52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$52::pill_button<ref$<str$> > @ 0x32e836 -> 0x68e960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >
// 信息 ID: 262d988428426c459a871d3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["country_core::systems::ui::GlobalId"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<country_core::systems::ui::GlobalId>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >", "toolbar::mei::core::impl$17::perform_layout::closure_env$0"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children -> ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x689040 -> 0xfe0e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::expand
// 信息 ID: a4bb6fb8608024180463d99e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$26::tab_switcher<ref$<str$> > @ 0x32def2 -> 0x68e8f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x334377 -> 0x68e8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::gap
// 信息 ID: a709ec38bf0482b3d69dd11e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x150fbb -> 0x68e8b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x15109e -> 0x68e8b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151181 -> 0x68e8b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151259 -> 0x68e8b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::horizontal
// 信息 ID: 45c0441850bfab19f96fa5d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::horizontal; module=341, inline_record=488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::padding
// 信息 ID: 5d02601768a29a9b0edce9f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=342, inline_record=9692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen
// 信息 ID: e3cc53524c8379a89c722d9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x689871 -> 0x68ea20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>
// 信息 ID: 9abca81c001b6c5d6a6192f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::layout::LayoutStyle::place_childen_impl -> ["toolbar::mei::layout::SwizzleXY"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::layout::LayoutStyle::place_childen @ 0x68ea4a -> 0xfe12a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>
// 信息 ID: c6b01b58abee9044e2194e4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::layout::LayoutStyle::place_childen_impl -> ["toolbar::mei::layout::SwizzleYX"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::layout::LayoutStyle::place_childen @ 0x68ea3b -> 0xfe19a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::size
// 信息 ID: 2a201c11f602c255e0ee8c38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$52::pill_button<ref$<str$> > @ 0x32e85b -> 0x68e9c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::space_between
// 信息 ID: c82870d685c1187fb1b77447
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> > @ 0x32eb47 -> 0x68e930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::vertical
// 信息 ID: ccfada19be242722a7f03a62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::xalign_center
// 信息 ID: d191d296ae19b277c4898e85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$52::pill_button<ref$<str$> > @ 0x32e841 -> 0x68e990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentage::points
// 信息 ID: 95b1674036827a741bdee4f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=7500; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=7532; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=7564; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=7596; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=7772; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=8128; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=8184; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=8716; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentage::points_or_zero
// 信息 ID: 667971833cc9dad7f8447623
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=2748; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6024; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6056; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6120; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6152; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6256; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6288; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6352; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
// 信息 ID: 9b00054d6cad09848d36ae51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1224, inline_record=12272; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1224, inline_record=12908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::height
// 信息 ID: bbcb7b56bd7bc46661e1b76b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6448; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1224, inline_record=10168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::points
// 信息 ID: 4a17feb707f6bbb3d323278b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=7448; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1224, inline_record=9960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::points_or_zero
// 信息 ID: 1bddc17f964c77f62ac2cd6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::size_vec2
// 信息 ID: 59b399cc4f4ebf261c39bc5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6424; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=7632; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1224, inline_record=10144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::width
// 信息 ID: 5e2d585666f4815062638d6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6504; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=7656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Size::points
// 信息 ID: 21560b91755984d4d27d87da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=8104; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=8160; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=8688; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1224, inline_record=8784; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1224, inline_record=10612; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1224, inline_record=10668; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1224, inline_record=11196; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1224, inline_record=11292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Size::points_or_zero
// 信息 ID: 7e7443b9c180d713454d7c56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=5996; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6092; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6228; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::ui::GlobalId> >,toolbar::mei::core::impl$17::perform_layout::closure_env$0> >; module=1224, inline_record=6324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize
// 信息 ID: 3a319724cf5ea19821e9a2fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1224, inline_record=12068; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1224, inline_record=12332; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1224, inline_record=12692; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1224, inline_record=12968; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >; module=1224, inline_record=13392; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Align> >; module=2778, inline_record=19648; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::XAlign> >; module=2778, inline_record=23252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 4168a4d9934b4bc909e3b082
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<toolbar::mei::layout::_::impl$0::deserialize::__FieldVisitor>; module=1224, inline_record=424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 信息 ID: 1ebecc66c3585382fc37b9c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 613f11779085392b817823f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2820, inline_record=868; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2820, inline_record=1220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 051c1c114078486bbd58d81b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> > @ 0x1e3a5f6 -> 0xfe28e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> > @ 0x1e3a776 -> 0xfe29e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: fccdd130d8017ae4505fa626
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2820, inline_record=644; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2820, inline_record=1004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 122d8a49bbb9222c39b9930d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2778, inline_record=9300; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2778, inline_record=9868; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2820, inline_record=536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 2d5bed0ae11ad137e0ae2371
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<toolbar::mei::layout::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=58780; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<toolbar::mei::layout::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=61300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 7093455f3e9559555deaa95a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::__private::de::content::EnumRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<enum2$<ron::error::Error>,toolbar::mei::layout::_::impl$0::deserialize::__Visitor> @ 0x1e76193 -> 0xfe2bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >
// 信息 ID: 65c714a28d2e5d6869015005
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["toolbar::mei::layout::LengthPercentage"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["enum2$<toolbar::mei::layout::LengthPercentage>", "ref_mut$<ron::de::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > > @ 0x1e3d171 -> 0xfe2560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 信息 ID: f8eb637d9d4efe5a8f067ae3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["ref_mut$<ron::de::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::tuple_variant<toolbar::mei::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1e3aff9 -> 0xfe22a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$9::tuple_variant<toolbar::mei::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x1e3b0a7 -> 0xfe22a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x1e3ceb6 -> 0xfe22a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 939c96a925f0e7c5ab5fdaa7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::__private::de::content::ContentRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> > @ 0xfe25ee -> 0xfe2090 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x1e76245 -> 0xfe2090 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x1e7627a -> 0xfe2090 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x1e762b0 -> 0xfe2090 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x1e762ea -> 0xfe2090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::horizontal
// 信息 ID: c8024cdb38de1ef9d41eaf8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x150f9a -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151085 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151168 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x151248 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_impl<ref$<str$> > @ 0x32eb37 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x32ed35 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f3c0 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x32f970 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<country_core::render::render_config::ImageQualityPreset> > @ 0x32ff20 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x330510 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x330ac2 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> > @ 0x330f90 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x331580 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x331b70 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x332190 -> 0x68e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x332660 -> 0x68e880 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$11::from_points
// 信息 ID: e0549269bda4f821847573a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$13::from_points
// 信息 ID: ec9ba7f2ca252c319341b8d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$16::default
// 信息 ID: d09ad520f01ab649ce410e1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$26::tab_switcher<ref$<str$> > @ 0x32dedf -> 0x68e850 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$52::pill_button<ref$<str$> > @ 0x32e827 -> 0x68e850 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x334364 -> 0x68e850 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=342, inline_record=2072; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$26::from
// 信息 ID: d895edb064956c4af71e0fc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1224, inline_record=12484; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1224, inline_record=13136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$27::from
// 信息 ID: df766ce9c0d2f1dc6e3cb0d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >; module=1224, inline_record=13744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
