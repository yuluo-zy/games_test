// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_stone_railings_assemble.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5e8c3bcd0a55b61c7fd134afedfd6844defbb9d5aaa583aeeece53a78d92a866
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_stone_railings_assemble.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 465, 1600
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":465,"record_offset":272,"rva":9106336,"va":5377815456}
// 编译实例 RVAs: 0x8af3a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2137997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e87360dfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377816680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-2139105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e81f5cdfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377817804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9094476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e8b43a75ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377816663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8583955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e8ed047dff","size":5,"soft_fail":false,"target_va":5369240608,"va":5377824558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4242882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e83e42bfff","size":5,"soft_fail":false,"target_va":5373574768,"va":5377817645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t305309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e89da80400","size":5,"soft_fail":false,"target_va":5378126944,"va":5377821630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409519b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t659584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e880100a00","size":5,"soft_fail":false,"target_va":5378480560,"va":5377820971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t681480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e808660a00","size":5,"soft_fail":false,"target_va":5378502608,"va":5377821123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5507874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e8220b5400","size":5,"soft_fail":false,"target_va":5383325024,"va":5377817145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141498980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12487909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e8e58cbe00","size":5,"soft_fail":false,"target_va":5390305664,"va":5377817750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a48a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12536262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e8c649bf00","size":5,"soft_fail":false,"target_va":5390354592,"va":5377818325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414b71d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12609238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e8d666c000","size":5,"soft_fail":false,"target_va":5390430672,"va":5377821429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38534473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e849fd4b02","size":5,"soft_fail":false,"target_va":5416352064,"va":5377817586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38796664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e878fd4f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5377824125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38799587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e8e3085002","size":5,"soft_fail":false,"target_va":5416621600,"va":5377822008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38797323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e80b005002","size":5,"soft_fail":false,"target_va":5416621188,"va":5377823860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t38797880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e838025002","size":5,"soft_fail":false,"target_va":5416621762,"va":5377823877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t660263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e827130a00","size":5,"soft_fail":false,"target_va":5378481680,"va":5377821412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=15
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5566075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e87bee5400","size":5,"soft_fail":false,"target_va":5383383152,"va":5377817072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_for_side
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t24705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e881600000","size":5,"soft_fail":false,"target_va":5377846544,"va":5377821834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5455888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e810405300","size":5,"soft_fail":false,"target_va":5383274080,"va":5377818187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_mat3_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5455726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377815456,"raw_hex":"e86e3f5300","size":5,"soft_fail":false,"target_va":5383278560,"va":5377822829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":3304,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":1656,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":1376,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":1256,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":465,"record_offset":10344,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":465,"record_offset":11956,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":465,"record_offset":8400,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":465,"record_offset":5720,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":465,"record_offset":2336,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":465,"record_offset":5748,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":465,"record_offset":1884,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":465,"record_offset":1172,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":465,"record_offset":3416,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":465,"record_offset":4140,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":465,"record_offset":2280,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":465,"record_offset":2196,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":465,"record_offset":4232,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":465,"record_offset":4044,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":8836,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":465,"record_offset":2512,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":465,"record_offset":2140,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":465,"record_offset":3332,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":465,"record_offset":8428,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27520,"source_line":237,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":5540,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":6140,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":4356,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":2364,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":6996,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":8036,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":6468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":6676,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":5892,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":2392,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":2456,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":7148,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":4268,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$53::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":2428,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26816,"source_line":1757,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":465,"record_offset":12328,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":465,"record_offset":6384,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::floor_offsets
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":465,"record_offset":636,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27386,"source_line":20,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":465,"record_offset":3276,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":1524,"parent_record_offset":1492,"ancestor_record_offset":272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":2760,"parent_record_offset":2668,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":272,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":1492,"parent_record_offset":1460,"ancestor_record_offset":272,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":272,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":1728,"parent_record_offset":1684,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":1684,"parent_record_offset":1656,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":1404,"parent_record_offset":1376,"ancestor_record_offset":272,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":3440,"parent_record_offset":3416,"ancestor_record_offset":272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":465,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":272,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":465,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":272,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":465,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":272,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":465,"record_offset":10372,"parent_record_offset":10344,"ancestor_record_offset":272,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":465,"record_offset":10444,"parent_record_offset":10400,"ancestor_record_offset":272,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":465,"record_offset":740,"parent_record_offset":684,"ancestor_record_offset":272,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":465,"record_offset":780,"parent_record_offset":740,"ancestor_record_offset":272,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":465,"record_offset":8556,"parent_record_offset":8520,"ancestor_record_offset":272,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":465,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":272,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":465,"record_offset":8592,"parent_record_offset":8520,"ancestor_record_offset":272,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":465,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":272,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":465,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":272,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":465,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":272,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":465,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":272,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":465,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":272,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":465,"record_offset":1920,"parent_record_offset":1884,"ancestor_record_offset":272,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":465,"record_offset":2012,"parent_record_offset":1976,"ancestor_record_offset":272,"inlinee":16983,"source_line":107,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":465,"record_offset":1948,"parent_record_offset":1920,"ancestor_record_offset":272,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":465,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":272,"inlinee":10719,"source_line":267,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":465,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":272,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":465,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":272,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":465,"record_offset":3464,"parent_record_offset":3440,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":465,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":272,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":465,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":272,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":465,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":272,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":465,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":272,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":465,"record_offset":2728,"parent_record_offset":2696,"ancestor_record_offset":272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":465,"record_offset":8684,"parent_record_offset":8452,"ancestor_record_offset":272,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":465,"record_offset":8764,"parent_record_offset":8736,"ancestor_record_offset":272,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":6076,"parent_record_offset":5916,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":7176,"parent_record_offset":7148,"ancestor_record_offset":272,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":465,"record_offset":10400,"parent_record_offset":10372,"ancestor_record_offset":272,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":465,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":272,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":465,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":465,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":272,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":465,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":465,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":272,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":465,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":465,"record_offset":3712,"parent_record_offset":3652,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":465,"record_offset":3684,"parent_record_offset":3652,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":272,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":1848,"parent_record_offset":1684,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":465,"record_offset":1292,"parent_record_offset":1256,"ancestor_record_offset":272,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":465,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":465,"record_offset":1328,"parent_record_offset":1292,"ancestor_record_offset":272,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":465,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":465,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":272,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":465,"record_offset":3652,"parent_record_offset":3616,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":465,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":272,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":465,"record_offset":8496,"parent_record_offset":8452,"ancestor_record_offset":272,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":465,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":272,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":272,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":272,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":272,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":272,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":272,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":5980,"parent_record_offset":5948,"ancestor_record_offset":272,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":5948,"parent_record_offset":5916,"ancestor_record_offset":272,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":5916,"parent_record_offset":5892,"ancestor_record_offset":272,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":6700,"parent_record_offset":6676,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":6784,"parent_record_offset":6676,"ancestor_record_offset":272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":272,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":465,"record_offset":3744,"parent_record_offset":3616,"ancestor_record_offset":272,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":465,"record_offset":3616,"parent_record_offset":3584,"ancestor_record_offset":272,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":465,"record_offset":4880,"parent_record_offset":4848,"ancestor_record_offset":272,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":465,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":272,"inlinee":27388,"source_line":21,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::floor_offsets::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs
//     evidence={"module":465,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":272,"inlinee":27390,"source_line":23,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_stone_railings_assemble.rs
//     evidence={"module":465,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":272,"inlinee":27518,"source_line":105,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_stone_railings_assemble.rs
//     evidence={"module":465,"record_offset":2544,"parent_record_offset":2512,"ancestor_record_offset":272,"inlinee":27519,"source_line":135,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_stone_railings_assemble.rs
//     evidence={"module":465,"record_offset":4080,"parent_record_offset":4044,"ancestor_record_offset":272,"inlinee":27521,"source_line":185,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B7E
// 首条位置: {"module":465,"inlinee":27518}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":465,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":2220,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B7F
// 首条位置: {"module":465,"inlinee":27519}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":2544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":2544,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":2544,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":2544,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":2760,"parent_record_offset":2668,"ancestor_record_offset":2544,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":2544,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":2544,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":2544,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":465,"record_offset":2728,"parent_record_offset":2696,"ancestor_record_offset":2544,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":2544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13B42
// 首条位置: {"module":1600,"inlinee":80706}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":8764,"parent_record_offset":8308,"ancestor_record_offset":8308,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1600,"record_offset":8364,"parent_record_offset":8308,"ancestor_record_offset":8308,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1600,"record_offset":8396,"parent_record_offset":8308,"ancestor_record_offset":8308,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::fract
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1600,"record_offset":8452,"parent_record_offset":8308,"ancestor_record_offset":8308,"inlinee":26640,"source_line":178,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1600,"record_offset":9124,"parent_record_offset":8308,"ancestor_record_offset":8308,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1600,"record_offset":8976,"parent_record_offset":8308,"ancestor_record_offset":8308,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1600,"record_offset":8536,"parent_record_offset":8308,"ancestor_record_offset":8308,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1600,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":8308,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1600,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":8308,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1600,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":8308,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":8828,"parent_record_offset":8792,"ancestor_record_offset":8308,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":8792,"parent_record_offset":8764,"ancestor_record_offset":8308,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":9224,"parent_record_offset":9160,"ancestor_record_offset":8308,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1600,"record_offset":9056,"parent_record_offset":9032,"ancestor_record_offset":8308,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1600,"record_offset":9080,"parent_record_offset":9056,"ancestor_record_offset":8308,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1600,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":8308,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::fract
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1600,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":8308,"inlinee":26641,"source_line":1873,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::trunc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1600,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":8308,"inlinee":21635,"source_line":1844,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":9728,"parent_record_offset":9696,"ancestor_record_offset":8308,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":9572,"parent_record_offset":9532,"ancestor_record_offset":8308,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1600,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":8308,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1600,"record_offset":9008,"parent_record_offset":8976,"ancestor_record_offset":8308,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1600,"record_offset":8940,"parent_record_offset":8792,"ancestor_record_offset":8308,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1600,"record_offset":8604,"parent_record_offset":8536,"ancestor_record_offset":8308,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1600,"record_offset":8632,"parent_record_offset":8604,"ancestor_record_offset":8308,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":9500,"parent_record_offset":9468,"ancestor_record_offset":8308,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":9532,"parent_record_offset":9500,"ancestor_record_offset":8308,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":9696,"parent_record_offset":9500,"ancestor_record_offset":8308,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1600,"record_offset":9160,"parent_record_offset":9124,"ancestor_record_offset":8308,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1600,"record_offset":8576,"parent_record_offset":8536,"ancestor_record_offset":8308,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1600,"record_offset":8668,"parent_record_offset":8536,"ancestor_record_offset":8308,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1600,"record_offset":9468,"parent_record_offset":9160,"ancestor_record_offset":8308,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B81
// 首条位置: {"module":465,"inlinee":27521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":465,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":4080,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railing_tips
// 记录形式: compiled_procedure; instances=1
// 源码行: 455
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":465,"record_offset":15560,"rva":9116848,"va":5377825968}
// 编译实例 RVAs: 0x8b1cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2147831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e8093adfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377826530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408a9d20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e81f7fffff","size":5,"soft_fail":false,"target_va":5377793312,"va":5377826300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409519b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t653144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e858f70900","size":5,"soft_fail":false,"target_va":5378480560,"va":5377827411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a6850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12536275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e8d349bf00","size":5,"soft_fail":false,"target_va":5390362704,"va":5377826424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414b71d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12602836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e8d44dc000","size":5,"soft_fail":false,"target_va":5390430672,"va":5377827831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t653861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e825fa0900","size":5,"soft_fail":false,"target_va":5378481680,"va":5377827814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railing_tips_at
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e84c410000","size":5,"soft_fail":false,"target_va":5377843216,"va":5377826495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5446874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e8da1c5300","size":5,"soft_fail":false,"target_va":5383274080,"va":5377827201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_mat3_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5451716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377825968,"raw_hex":"e8c42f5300","size":5,"soft_fail":false,"target_va":5383278560,"va":5377826839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":16088,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":465,"record_offset":16144,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":465,"record_offset":16508,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":465,"record_offset":17300,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":465,"record_offset":16536,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":15948,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":465,"record_offset":17328,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":27520,"source_line":237,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":16992,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":16272,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":16452,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":465,"record_offset":16620,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_railing_shared::railing_tip_placements
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_railing_shared.rs
//     evidence={"module":465,"record_offset":15812,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":27492,"source_line":33,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":17912,"parent_record_offset":17888,"ancestor_record_offset":15560,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":15560,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":15560,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":465,"record_offset":17888,"parent_record_offset":17800,"ancestor_record_offset":15560,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":16044,"parent_record_offset":16020,"ancestor_record_offset":15560,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":16020,"parent_record_offset":15996,"ancestor_record_offset":15560,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":465,"record_offset":15996,"parent_record_offset":15972,"ancestor_record_offset":15560,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":465,"record_offset":15884,"parent_record_offset":15860,"ancestor_record_offset":15560,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":465,"record_offset":18040,"parent_record_offset":18016,"ancestor_record_offset":15560,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":465,"record_offset":18088,"parent_record_offset":18064,"ancestor_record_offset":15560,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":465,"record_offset":17448,"parent_record_offset":17416,"ancestor_record_offset":15560,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":465,"record_offset":17508,"parent_record_offset":17484,"ancestor_record_offset":15560,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":465,"record_offset":17484,"parent_record_offset":17416,"ancestor_record_offset":15560,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":465,"record_offset":16584,"parent_record_offset":16560,"ancestor_record_offset":15560,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":465,"record_offset":16784,"parent_record_offset":16760,"ancestor_record_offset":15560,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":465,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":15560,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":465,"record_offset":16560,"parent_record_offset":16536,"ancestor_record_offset":15560,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":465,"record_offset":15908,"parent_record_offset":15884,"ancestor_record_offset":15560,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":465,"record_offset":15860,"parent_record_offset":15812,"ancestor_record_offset":15560,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":465,"record_offset":16856,"parent_record_offset":16832,"ancestor_record_offset":15560,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":465,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":15560,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":465,"record_offset":17856,"parent_record_offset":17828,"ancestor_record_offset":15560,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":465,"record_offset":17576,"parent_record_offset":17352,"ancestor_record_offset":15560,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":465,"record_offset":17656,"parent_record_offset":17628,"ancestor_record_offset":15560,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":16476,"parent_record_offset":16452,"ancestor_record_offset":15560,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":16300,"parent_record_offset":16272,"ancestor_record_offset":15560,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":465,"record_offset":16808,"parent_record_offset":16784,"ancestor_record_offset":15560,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":15972,"parent_record_offset":15948,"ancestor_record_offset":15560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":465,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":15560,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":465,"record_offset":16728,"parent_record_offset":16660,"ancestor_record_offset":15560,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":465,"record_offset":16204,"parent_record_offset":16144,"ancestor_record_offset":15560,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":465,"record_offset":16176,"parent_record_offset":16144,"ancestor_record_offset":15560,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":465,"record_offset":16232,"parent_record_offset":16204,"ancestor_record_offset":15560,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":465,"record_offset":16660,"parent_record_offset":16620,"ancestor_record_offset":15560,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":465,"record_offset":17352,"parent_record_offset":17328,"ancestor_record_offset":15560,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":465,"record_offset":17392,"parent_record_offset":17352,"ancestor_record_offset":15560,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":465,"record_offset":17416,"parent_record_offset":17392,"ancestor_record_offset":15560,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":17016,"parent_record_offset":16992,"ancestor_record_offset":15560,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":17100,"parent_record_offset":16992,"ancestor_record_offset":15560,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":465,"record_offset":16760,"parent_record_offset":16620,"ancestor_record_offset":15560,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::add_corner_stone_railing_piece
// 记录形式: compiled_procedure; instances=1
// 源码行: 555
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":465,"record_offset":19192,"rva":9119568,"va":5377828688}
// 编译实例 RVAs: 0x8b2750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409519b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t651214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377828688,"raw_hex":"e8ceef0900","size":5,"soft_fail":false,"target_va":5378480560,"va":5377829341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t673247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377828688,"raw_hex":"e8df450a00","size":5,"soft_fail":false,"target_va":5378502608,"va":5377829356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5495989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377828688,"raw_hex":"e8b5dc5300","size":5,"soft_fail":false,"target_va":5383325024,"va":5377829030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414b71d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_stone_railings_assemble.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12601098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377828688,"raw_hex":"e80a47c000","size":5,"soft_fail":false,"target_va":5390430672,"va":5377829569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38791817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377828688,"raw_hex":"e889ea4f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377829940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t652117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377828688,"raw_hex":"e855f30900","size":5,"soft_fail":false,"target_va":5378481680,"va":5377829558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":465,"record_offset":20592,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":19432,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":19492,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":19464,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":20188,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":19920,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":19824,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":20100,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":465,"record_offset":20500,"parent_record_offset":19192,"ancestor_record_offset":19192,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":465,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":19192,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":20128,"parent_record_offset":20100,"ancestor_record_offset":19192,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":19676,"parent_record_offset":19516,"ancestor_record_offset":19192,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":465,"record_offset":20532,"parent_record_offset":20500,"ancestor_record_offset":19192,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":19580,"parent_record_offset":19548,"ancestor_record_offset":19192,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":19548,"parent_record_offset":19516,"ancestor_record_offset":19192,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":19516,"parent_record_offset":19492,"ancestor_record_offset":19192,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":465,"record_offset":19764,"parent_record_offset":19740,"ancestor_record_offset":19192,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00465.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":465,"record_offset":19612,"parent_record_offset":19580,"ancestor_record_offset":19192,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00465.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_stone_railings_assemble.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_stone_railings_assemble.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_stone_railings_assemble.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::add_corner_stone_railing_piece
// 信息 ID: 15d56aed293d918e0e91be70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36b9a6 -> 0x8b2750 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37de06 -> 0x8b2750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railing_tips
// 信息 ID: dcdcc82b49adbed0358fcf22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36ba6c -> 0x8b1cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37dec5 -> 0x8b1cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings
// 信息 ID: 6af58307c553cf5245c8df90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36aa13 -> 0x8af3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36b0d2 -> 0x8af3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37d7e1 -> 0x8af3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37ca8a -> 0x8af3a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$0
// 信息 ID: d8260a4c2710961c705090a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings; module=465, inline_record=2220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$1
// 信息 ID: c98dcb1c8878c1113873d729
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings; module=465, inline_record=2544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$2
// 信息 ID: 0223ba6c99b8fae247f9fd35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<f32>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<f32>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<f32>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure_env$2> > > >; module=1600, inline_record=8308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings::closure$3
// 信息 ID: 13cc6cc658f143d95b7b1781
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_stone_railings_assemble::assemble_stone_railings; module=465, inline_record=4080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
