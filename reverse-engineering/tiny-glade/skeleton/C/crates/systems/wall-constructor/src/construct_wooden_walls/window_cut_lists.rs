// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_wooden_walls/window_cut_lists.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 502769cc0ec193a9a6ae15e5fa53c421fc36b32ca3ce2f343b011fdec958c1c4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: window_cut_lists.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1455, 1482, 1494, 3156
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists::new
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13799
// 首条位置: {"module":1494,"inlinee":79769}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1494,"record_offset":1860,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1494,"record_offset":2012,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1494,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1816,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1494,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1816,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1494,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":1816,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/window_cut_lists.rs
//     evidence={"module":1494,"record_offset":1892,"parent_record_offset":1860,"ancestor_record_offset":1816,"inlinee":79771,"source_line":49,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1379B
// 首条位置: {"module":1494,"inlinee":79771}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1494,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":1892,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1494,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1892,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1494,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1892,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13615
// 首条位置: {"module":1455,"inlinee":79381}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_cutout_aabb
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1455,"record_offset":174968,"parent_record_offset":174940,"ancestor_record_offset":174940,"inlinee":79383,"source_line":1483,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1455,"record_offset":175028,"parent_record_offset":174968,"ancestor_record_offset":174940,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1455,"record_offset":175056,"parent_record_offset":174968,"ancestor_record_offset":174940,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists::list_for_piece
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3156,"record_offset":280,"rva":38679776,"va":5407388896}
// 编译实例 RVAs: 0x24e34e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: window_cut_lists.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8963877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407388896,"raw_hex":"e825c78800","size":5,"soft_fail":false,"target_va":5416353040,"va":5407389158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_buffers::pack_window_cut_list
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25633374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407388896,"raw_hex":"e8a2dd78fe","size":5,"soft_fail":false,"target_va":5381755920,"va":5407389289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24087290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407388896,"raw_hex":"e8067590fe","size":5,"soft_fail":false,"target_va":5383301744,"va":5407389029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3156,"record_offset":1408,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3156,"record_offset":564,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3156,"record_offset":1380,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3156,"record_offset":1324,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3156,"record_offset":480,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3156,"record_offset":1112,"parent_record_offset":1080,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3156,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":280,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3156,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3156,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3156,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":280,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3156,"record_offset":1080,"parent_record_offset":612,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3156,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":280,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3156,"record_offset":1228,"parent_record_offset":612,"ancestor_record_offset":280,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3156,"record_offset":1200,"parent_record_offset":612,"ancestor_record_offset":280,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3156,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":280,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3156,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":280,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3156,"record_offset":852,"parent_record_offset":768,"ancestor_record_offset":280,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":3156,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":280,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3156,"record_offset":656,"parent_record_offset":612,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3156,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":280,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3156,"record_offset":884,"parent_record_offset":852,"ancestor_record_offset":280,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3156,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":280,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3156,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3156,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":280,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3156,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3156,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3156,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3156,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3156,"record_offset":984,"parent_record_offset":952,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/window_cut_lists.rs
//     evidence={"module":3156,"record_offset":952,"parent_record_offset":932,"ancestor_record_offset":280,"inlinee":79327,"source_line":76,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/window_cut_lists.rs
//     evidence={"module":3156,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":280,"inlinee":79326,"source_line":96,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135DF
// 首条位置: {"module":1455,"inlinee":79327}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3156,"record_offset":984,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3156,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":952,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135DE
// 首条位置: {"module":1455,"inlinee":79326}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/window_cut_lists.rs
//     evidence={"module":3156,"record_offset":952,"parent_record_offset":932,"ancestor_record_offset":932,"inlinee":79327,"source_line":76,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3156,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":932,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3156,"record_offset":984,"parent_record_offset":952,"ancestor_record_offset":932,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03156.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists::into_parts
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137AC
// 首条位置: {"module":1494,"inlinee":79788}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1494,"record_offset":11736,"parent_record_offset":11708,"ancestor_record_offset":11708,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1494,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":11708,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1494,"record_offset":11944,"parent_record_offset":11920,"ancestor_record_offset":11708,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1494,"record_offset":11968,"parent_record_offset":11944,"ancestor_record_offset":11708,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1494,"record_offset":11896,"parent_record_offset":11808,"ancestor_record_offset":11708,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1494,"record_offset":11836,"parent_record_offset":11808,"ancestor_record_offset":11708,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1494,"record_offset":11808,"parent_record_offset":11784,"ancestor_record_offset":11708,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1494,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":11708,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1494,"record_offset":11864,"parent_record_offset":11836,"ancestor_record_offset":11708,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01494.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1494,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":11708,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01494.jsonl.gz"}
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
// 完整结构及原始类型记录: window_cut_lists.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: window_cut_lists.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: window_cut_lists.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists::into_parts
// 信息 ID: 6e4c07c3c82574fb860d0e3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=11708; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=27888; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=35968; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=44716; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=54568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists::list_for_piece
// 信息 ID: c6c7eebb9a4aaa9ededed117
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x14598ac -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x14592a3 -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145ad6e -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x1458099 -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x1457b75 -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145bf8f -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145bc3f -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145d31d -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::emit_curved_horizontal_wall_board_piece<system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145f905 -> 0x24e34e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists::new
// 信息 ID: 79305b435da2cb21e14f0f2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=1816; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=14048; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=29612; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=37724; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=47092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece::closure$0
// 信息 ID: 79e0c070523ebc4fd0829057
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists::list_for_piece; module=3156, inline_record=952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece::closure$1
// 信息 ID: 958a39d48265afce0e449a2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::list_for_piece", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::window_cut_lists::WindowCutLists::list_for_piece; module=3156, inline_record=932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0
// 信息 ID: 75dbf4b97e5c5e86c17cc4b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=1892; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=14124; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=29688; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=37800; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11>; module=1494, inline_record=47168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure$0
// 信息 ID: 320c8d215112a8d4bcd8f611
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >,country_core::resources::walls::window_cuts::impl$0::records::closure_env$0>,system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::window_cuts::WindowCutRecord> >,country_core::resources::walls::window_cuts::impl$0::records::closure_env$0>,system_wall_constructor::construct_wooden_walls::window_cut_lists::impl$0::new::closure$0::closure_env$0> >; module=1455, inline_record=174940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
