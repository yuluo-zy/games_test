// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/wall_color_slots.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 612e09d7363c8184247ceed4a25fed7f46e633a672787027be7ce87d55837ab2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_color_slots.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1131, 2711
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::default_filled<country_core::resources::color_id::GateColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":34428,"rva":17569296,"va":5386278416}
// 编译实例 RVAs: 0x10c1610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10599842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e85e425eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386278525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17556319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8a11cf4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386278506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba1820
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5373446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8fa01aeff","size":5,"soft_fail":false,"target_va":5380904992,"va":5386278433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e898fcce01","size":5,"soft_fail":false,"target_va":5416620794,"va":5386278493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e88ffece01","size":5,"soft_fail":false,"target_va":5416621536,"va":5386278732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":34652,"parent_record_offset":34428,"ancestor_record_offset":34428,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":34616,"parent_record_offset":34428,"ancestor_record_offset":34428,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":34992,"parent_record_offset":34968,"ancestor_record_offset":34428,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":35016,"parent_record_offset":34992,"ancestor_record_offset":34428,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":35040,"parent_record_offset":35016,"ancestor_record_offset":34428,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":34968,"parent_record_offset":34788,"ancestor_record_offset":34428,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":34732,"parent_record_offset":34708,"ancestor_record_offset":34428,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":34788,"parent_record_offset":34756,"ancestor_record_offset":34428,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":34756,"parent_record_offset":34732,"ancestor_record_offset":34428,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":34824,"parent_record_offset":34788,"ancestor_record_offset":34428,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":35096,"parent_record_offset":34676,"ancestor_record_offset":34428,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":34708,"parent_record_offset":34676,"ancestor_record_offset":34428,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1131,"record_offset":34676,"parent_record_offset":34652,"ancestor_record_offset":34428,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1131,"record_offset":34848,"parent_record_offset":34824,"ancestor_record_offset":34428,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1131,"record_offset":35204,"parent_record_offset":35172,"ancestor_record_offset":34428,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1131,"record_offset":34928,"parent_record_offset":34872,"ancestor_record_offset":34428,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":35172,"parent_record_offset":35140,"ancestor_record_offset":34428,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":35140,"parent_record_offset":35096,"ancestor_record_offset":34428,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":34872,"parent_record_offset":34848,"ancestor_record_offset":34428,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":34900,"parent_record_offset":34872,"ancestor_record_offset":34428,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1131,"record_offset":35276,"parent_record_offset":35096,"ancestor_record_offset":34428,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1131,"record_offset":35244,"parent_record_offset":35096,"ancestor_record_offset":34428,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::default_filled<country_core::resources::color_id::HalftimberColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":35348,"rva":17569296,"va":5386278416}
// 编译实例 RVAs: 0x10c1610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10599842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e85e425eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386278525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17556319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8a11cf4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386278506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba1820
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5373446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8fa01aeff","size":5,"soft_fail":false,"target_va":5380904992,"va":5386278433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e898fcce01","size":5,"soft_fail":false,"target_va":5416620794,"va":5386278493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e88ffece01","size":5,"soft_fail":false,"target_va":5416621536,"va":5386278732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":35576,"parent_record_offset":35348,"ancestor_record_offset":35348,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":35540,"parent_record_offset":35348,"ancestor_record_offset":35348,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":35916,"parent_record_offset":35892,"ancestor_record_offset":35348,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":35940,"parent_record_offset":35916,"ancestor_record_offset":35348,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":35964,"parent_record_offset":35940,"ancestor_record_offset":35348,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":35892,"parent_record_offset":35712,"ancestor_record_offset":35348,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":35656,"parent_record_offset":35632,"ancestor_record_offset":35348,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":35712,"parent_record_offset":35680,"ancestor_record_offset":35348,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":35680,"parent_record_offset":35656,"ancestor_record_offset":35348,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":35748,"parent_record_offset":35712,"ancestor_record_offset":35348,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":36020,"parent_record_offset":35600,"ancestor_record_offset":35348,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":35632,"parent_record_offset":35600,"ancestor_record_offset":35348,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1131,"record_offset":35600,"parent_record_offset":35576,"ancestor_record_offset":35348,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1131,"record_offset":35772,"parent_record_offset":35748,"ancestor_record_offset":35348,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1131,"record_offset":36128,"parent_record_offset":36096,"ancestor_record_offset":35348,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1131,"record_offset":35852,"parent_record_offset":35796,"ancestor_record_offset":35348,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":36096,"parent_record_offset":36064,"ancestor_record_offset":35348,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":36064,"parent_record_offset":36020,"ancestor_record_offset":35348,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":35796,"parent_record_offset":35772,"ancestor_record_offset":35348,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":35824,"parent_record_offset":35796,"ancestor_record_offset":35348,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1131,"record_offset":36200,"parent_record_offset":36020,"ancestor_record_offset":35348,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1131,"record_offset":36168,"parent_record_offset":36020,"ancestor_record_offset":35348,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::default_filled<country_core::resources::color_id::StoneFoundationColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":38116,"rva":17569632,"va":5386278752}
// 编译实例 RVAs: 0x10c1760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10600178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278752,"raw_hex":"e80e415eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386278861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17556655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278752,"raw_hex":"e8511bf4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386278842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba1830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5373766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278752,"raw_hex":"e8ba00aeff","size":5,"soft_fail":false,"target_va":5380905008,"va":5386278769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30341960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278752,"raw_hex":"e848fbce01","size":5,"soft_fail":false,"target_va":5416620794,"va":5386278829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278752,"raw_hex":"e83ffdce01","size":5,"soft_fail":false,"target_va":5416621536,"va":5386279068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":38352,"parent_record_offset":38116,"ancestor_record_offset":38116,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":38316,"parent_record_offset":38116,"ancestor_record_offset":38116,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":38692,"parent_record_offset":38668,"ancestor_record_offset":38116,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":38716,"parent_record_offset":38692,"ancestor_record_offset":38116,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":38740,"parent_record_offset":38716,"ancestor_record_offset":38116,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":38668,"parent_record_offset":38488,"ancestor_record_offset":38116,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":38432,"parent_record_offset":38408,"ancestor_record_offset":38116,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":38488,"parent_record_offset":38456,"ancestor_record_offset":38116,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":38456,"parent_record_offset":38432,"ancestor_record_offset":38116,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":38524,"parent_record_offset":38488,"ancestor_record_offset":38116,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":38796,"parent_record_offset":38376,"ancestor_record_offset":38116,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":38408,"parent_record_offset":38376,"ancestor_record_offset":38116,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1131,"record_offset":38376,"parent_record_offset":38352,"ancestor_record_offset":38116,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1131,"record_offset":38548,"parent_record_offset":38524,"ancestor_record_offset":38116,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1131,"record_offset":38904,"parent_record_offset":38872,"ancestor_record_offset":38116,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1131,"record_offset":38628,"parent_record_offset":38572,"ancestor_record_offset":38116,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":38872,"parent_record_offset":38840,"ancestor_record_offset":38116,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":38840,"parent_record_offset":38796,"ancestor_record_offset":38116,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":38572,"parent_record_offset":38548,"ancestor_record_offset":38116,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":38600,"parent_record_offset":38572,"ancestor_record_offset":38116,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1131,"record_offset":38976,"parent_record_offset":38796,"ancestor_record_offset":38116,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1131,"record_offset":38944,"parent_record_offset":38796,"ancestor_record_offset":38116,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::default_filled<country_core::resources::color_id::WallColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":36272,"rva":17569296,"va":5386278416}
// 编译实例 RVAs: 0x10c1610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10599842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e85e425eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386278525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17556319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8a11cf4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386278506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba1820
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5373446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8fa01aeff","size":5,"soft_fail":false,"target_va":5380904992,"va":5386278433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e898fcce01","size":5,"soft_fail":false,"target_va":5416620794,"va":5386278493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e88ffece01","size":5,"soft_fail":false,"target_va":5416621536,"va":5386278732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":36496,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":36460,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":36836,"parent_record_offset":36812,"ancestor_record_offset":36272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":36860,"parent_record_offset":36836,"ancestor_record_offset":36272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":36884,"parent_record_offset":36860,"ancestor_record_offset":36272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":36812,"parent_record_offset":36632,"ancestor_record_offset":36272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":36576,"parent_record_offset":36552,"ancestor_record_offset":36272,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":36632,"parent_record_offset":36600,"ancestor_record_offset":36272,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":36600,"parent_record_offset":36576,"ancestor_record_offset":36272,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":36668,"parent_record_offset":36632,"ancestor_record_offset":36272,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":36940,"parent_record_offset":36520,"ancestor_record_offset":36272,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":36552,"parent_record_offset":36520,"ancestor_record_offset":36272,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1131,"record_offset":36520,"parent_record_offset":36496,"ancestor_record_offset":36272,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1131,"record_offset":36692,"parent_record_offset":36668,"ancestor_record_offset":36272,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1131,"record_offset":37048,"parent_record_offset":37016,"ancestor_record_offset":36272,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1131,"record_offset":36772,"parent_record_offset":36716,"ancestor_record_offset":36272,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":37016,"parent_record_offset":36984,"ancestor_record_offset":36272,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":36984,"parent_record_offset":36940,"ancestor_record_offset":36272,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":36716,"parent_record_offset":36692,"ancestor_record_offset":36272,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":36744,"parent_record_offset":36716,"ancestor_record_offset":36272,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1131,"record_offset":37120,"parent_record_offset":36940,"ancestor_record_offset":36272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1131,"record_offset":37088,"parent_record_offset":36940,"ancestor_record_offset":36272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::default_filled<country_core::resources::color_id::WoodenWallColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":37192,"rva":17569296,"va":5386278416}
// 编译实例 RVAs: 0x10c1610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10599842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e85e425eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386278525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17556319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8a11cf4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386278506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba1820
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5373446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8fa01aeff","size":5,"soft_fail":false,"target_va":5380904992,"va":5386278433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e898fcce01","size":5,"soft_fail":false,"target_va":5416620794,"va":5386278493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e88ffece01","size":5,"soft_fail":false,"target_va":5416621536,"va":5386278732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":37420,"parent_record_offset":37192,"ancestor_record_offset":37192,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":37384,"parent_record_offset":37192,"ancestor_record_offset":37192,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":37760,"parent_record_offset":37736,"ancestor_record_offset":37192,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":37784,"parent_record_offset":37760,"ancestor_record_offset":37192,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":37808,"parent_record_offset":37784,"ancestor_record_offset":37192,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":37736,"parent_record_offset":37556,"ancestor_record_offset":37192,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":37500,"parent_record_offset":37476,"ancestor_record_offset":37192,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":37556,"parent_record_offset":37524,"ancestor_record_offset":37192,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":37524,"parent_record_offset":37500,"ancestor_record_offset":37192,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":37592,"parent_record_offset":37556,"ancestor_record_offset":37192,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":37864,"parent_record_offset":37444,"ancestor_record_offset":37192,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":37476,"parent_record_offset":37444,"ancestor_record_offset":37192,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1131,"record_offset":37444,"parent_record_offset":37420,"ancestor_record_offset":37192,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1131,"record_offset":37616,"parent_record_offset":37592,"ancestor_record_offset":37192,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1131,"record_offset":37972,"parent_record_offset":37940,"ancestor_record_offset":37192,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1131,"record_offset":37696,"parent_record_offset":37640,"ancestor_record_offset":37192,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":37940,"parent_record_offset":37908,"ancestor_record_offset":37192,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":37908,"parent_record_offset":37864,"ancestor_record_offset":37192,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":37640,"parent_record_offset":37616,"ancestor_record_offset":37192,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":37668,"parent_record_offset":37640,"ancestor_record_offset":37192,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1131,"record_offset":38044,"parent_record_offset":37864,"ancestor_record_offset":37192,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1131,"record_offset":38012,"parent_record_offset":37864,"ancestor_record_offset":37192,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::default_filled<country_core::resources::color_id_roof::RoofColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":33504,"rva":17569296,"va":5386278416}
// 编译实例 RVAs: 0x10c1610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10599842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e85e425eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386278525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17556319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8a11cf4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386278506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba1820
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5373446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e8fa01aeff","size":5,"soft_fail":false,"target_va":5380904992,"va":5386278433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e898fcce01","size":5,"soft_fail":false,"target_va":5416620794,"va":5386278493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386278416,"raw_hex":"e88ffece01","size":5,"soft_fail":false,"target_va":5416621536,"va":5386278732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":33732,"parent_record_offset":33504,"ancestor_record_offset":33504,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":33696,"parent_record_offset":33504,"ancestor_record_offset":33504,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":34072,"parent_record_offset":34048,"ancestor_record_offset":33504,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":34096,"parent_record_offset":34072,"ancestor_record_offset":33504,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":34120,"parent_record_offset":34096,"ancestor_record_offset":33504,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":34048,"parent_record_offset":33868,"ancestor_record_offset":33504,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":33812,"parent_record_offset":33788,"ancestor_record_offset":33504,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":33868,"parent_record_offset":33836,"ancestor_record_offset":33504,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":33836,"parent_record_offset":33812,"ancestor_record_offset":33504,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":33904,"parent_record_offset":33868,"ancestor_record_offset":33504,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":34176,"parent_record_offset":33756,"ancestor_record_offset":33504,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":33788,"parent_record_offset":33756,"ancestor_record_offset":33504,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1131,"record_offset":33756,"parent_record_offset":33732,"ancestor_record_offset":33504,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1131,"record_offset":33928,"parent_record_offset":33904,"ancestor_record_offset":33504,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1131,"record_offset":34284,"parent_record_offset":34252,"ancestor_record_offset":33504,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1131,"record_offset":34008,"parent_record_offset":33952,"ancestor_record_offset":33504,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":34252,"parent_record_offset":34220,"ancestor_record_offset":33504,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":34220,"parent_record_offset":34176,"ancestor_record_offset":33504,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":33952,"parent_record_offset":33928,"ancestor_record_offset":33504,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":33980,"parent_record_offset":33952,"ancestor_record_offset":33504,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1131,"record_offset":34356,"parent_record_offset":34176,"ancestor_record_offset":33504,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1131,"record_offset":34324,"parent_record_offset":34176,"ancestor_record_offset":33504,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::default_filled<country_core::resources::walls::wall_render_slots::SlotWallIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":39048,"rva":17569968,"va":5386279088}
// 编译实例 RVAs: 0x10c18b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10600512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279088,"raw_hex":"e8c03f5eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5386279195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17556989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279088,"raw_hex":"e8031af4fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5386279176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6e190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6633269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279088,"raw_hex":"e8cbc89aff","size":5,"soft_fail":false,"target_va":5379645840,"va":5386279104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30341625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279088,"raw_hex":"e8f9f9ce01","size":5,"soft_fail":false,"target_va":5416620794,"va":5386279164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30342144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279088,"raw_hex":"e800fcce01","size":5,"soft_fail":false,"target_va":5416621536,"va":5386279387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":39288,"parent_record_offset":39048,"ancestor_record_offset":39048,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":39252,"parent_record_offset":39048,"ancestor_record_offset":39048,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":39628,"parent_record_offset":39604,"ancestor_record_offset":39048,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":39652,"parent_record_offset":39628,"ancestor_record_offset":39048,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":39676,"parent_record_offset":39652,"ancestor_record_offset":39048,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1131,"record_offset":39604,"parent_record_offset":39424,"ancestor_record_offset":39048,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":39368,"parent_record_offset":39344,"ancestor_record_offset":39048,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":39424,"parent_record_offset":39392,"ancestor_record_offset":39048,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":39392,"parent_record_offset":39368,"ancestor_record_offset":39048,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1131,"record_offset":39460,"parent_record_offset":39424,"ancestor_record_offset":39048,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":39732,"parent_record_offset":39312,"ancestor_record_offset":39048,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1131,"record_offset":39344,"parent_record_offset":39312,"ancestor_record_offset":39048,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1131,"record_offset":39312,"parent_record_offset":39288,"ancestor_record_offset":39048,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1131,"record_offset":39484,"parent_record_offset":39460,"ancestor_record_offset":39048,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1131,"record_offset":39840,"parent_record_offset":39808,"ancestor_record_offset":39048,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1131,"record_offset":39564,"parent_record_offset":39508,"ancestor_record_offset":39048,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":39808,"parent_record_offset":39776,"ancestor_record_offset":39048,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1131,"record_offset":39776,"parent_record_offset":39732,"ancestor_record_offset":39048,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":39508,"parent_record_offset":39484,"ancestor_record_offset":39048,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1131,"record_offset":39536,"parent_record_offset":39508,"ancestor_record_offset":39048,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1131,"record_offset":39912,"parent_record_offset":39732,"ancestor_record_offset":39048,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1131,"record_offset":39880,"parent_record_offset":39732,"ancestor_record_offset":39048,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::ColorTables::colors_of
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2711,"record_offset":268,"rva":35133936,"va":5403843056}
// 编译实例 RVAs: 0x21819f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34301096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843056,"raw_hex":"e8589bf4fd","size":5,"soft_fail":false,"target_va":5369542000,"va":5403843091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbb800
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18637439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843056,"raw_hex":"e8819de3fe","size":5,"soft_fail":false,"target_va":5385205760,"va":5403843194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_id::WallColorTable::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22938584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843056,"raw_hex":"e828fca1fe","size":5,"soft_fail":false,"target_va":5380904528,"va":5403843107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2711,"record_offset":1348,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2711,"record_offset":412,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2711,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2711,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":268,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2711,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":268,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2711,"record_offset":1176,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2711,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2711,"record_offset":880,"parent_record_offset":852,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2711,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2711,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":2711,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":268,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":820,"parent_record_offset":792,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":852,"parent_record_offset":820,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":792,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2711,"record_offset":652,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2711,"record_offset":1204,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2711,"record_offset":736,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2711,"record_offset":596,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2711,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":268,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2711,"record_offset":436,"parent_record_offset":412,"ancestor_record_offset":268,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2711,"record_offset":1060,"parent_record_offset":952,"ancestor_record_offset":268,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2711,"record_offset":464,"parent_record_offset":436,"ancestor_record_offset":268,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":1000,"parent_record_offset":976,"ancestor_record_offset":268,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":1296,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":268,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":268,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":492,"parent_record_offset":436,"ancestor_record_offset":268,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":268,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":624,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":952,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::sync_wall_color_slots
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2711,"record_offset":1380,"rva":35134288,"va":5403843408}
// 编译实例 RVAs: 0x2181b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19234727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e85980dafe","size":5,"soft_fail":false,"target_va":5384609408,"va":5403844130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..collections..hash..map..HashMap$LT$K$C$V$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$$LP$K$C$V$RP$$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7559732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8345a7300","size":5,"soft_fail":false,"target_va":5411403456,"va":5403843719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=18
//     evidence={"assembly":"\tcallq\t-28166404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8fc3652fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403845103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ea970
//     owner=unresolved_namespace; count=3
//     target_aliases=33 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34173507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8bd8df6fd","size":5,"soft_fail":false,"target_va":5369670000,"va":5403843502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25717589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8ab9477fe","size":5,"soft_fail":false,"target_va":5378126944,"va":5403844528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a05e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24624590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8324288fe","size":5,"soft_fail":false,"target_va":5379218944,"va":5403843529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a4b310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24341396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e86c948cfe","size":5,"soft_fail":false,"target_va":5379502864,"va":5403844255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24128549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8dbd38ffe","size":5,"soft_fail":false,"target_va":5379716240,"va":5403844784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410a3db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17689205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e88b15f2fe","size":5,"soft_fail":false,"target_va":5386157488,"va":5403846688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c1610
//     owner=unresolved_namespace; count=5
//     target_aliases=6 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17565736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8d8f7f3fe","size":5,"soft_fail":false,"target_va":5386278416,"va":5403844147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c1760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17565459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8edf8f3fe","size":5,"soft_fail":false,"target_va":5386278752,"va":5403844206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c18b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17565152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e820faf3fe","size":5,"soft_fail":false,"target_va":5386279088,"va":5403844235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c19f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17566271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8c1f5f3fe","size":5,"soft_fail":false,"target_va":5386279408,"va":5403845674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c1ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17565795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e89df7f3fe","size":5,"soft_fail":false,"target_va":5386279840,"va":5403845630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c1d50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17565319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e879f9f3fe","size":5,"soft_fail":false,"target_va":5386280272,"va":5403845586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c1f00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17565019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8a5faf3fe","size":5,"soft_fail":false,"target_va":5386280704,"va":5403845718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c20b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17564565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e86bfcf3fe","size":5,"soft_fail":false,"target_va":5386281136,"va":5403845696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c2260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17564045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e873fef3fe","size":5,"soft_fail":false,"target_va":5386281568,"va":5403845608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c2410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17563657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8f7fff3fe","size":5,"soft_fail":false,"target_va":5386282000,"va":5403845652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421833a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8fa110000","size":5,"soft_fail":false,"target_va":5403849632,"va":5403845025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421837e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e865160000","size":5,"soft_fail":false,"target_va":5403850720,"va":5403844982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142183c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e84f1a0000","size":5,"soft_fail":false,"target_va":5403851808,"va":5403845068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142184060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8911f0000","size":5,"soft_fail":false,"target_va":5403852896,"va":5403844810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421844a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e850230000","size":5,"soft_fail":false,"target_va":5403853984,"va":5403844939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421848e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8bb270000","size":5,"soft_fail":false,"target_va":5403855072,"va":5403844896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142184d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8262c0000","size":5,"soft_fail":false,"target_va":5403856160,"va":5403844853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t12775908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8e4f1c200","size":5,"soft_fail":false,"target_va":5416621762,"va":5403845849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_render_slots::WallRenderSlots::gpu_capacity
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24198606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e832c28efe","size":5,"soft_fail":false,"target_va":5379644912,"va":5403843513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_render_slots::WallRenderSlots::set_gpu_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24200457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e8f7ba8efe","size":5,"soft_fail":false,"target_va":5379644928,"va":5403845380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_render_slots::WallRenderSlots::slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24200359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e859bb8efe","size":5,"soft_fail":false,"target_va":5379645184,"va":5403845538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_render_slots::WallRenderSlots::take_pending_upload
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24200353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e85fbb8efe","size":5,"soft_fail":false,"target_va":5379644976,"va":5403845324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::wall_color_slots::ColorTables::colors_of
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403843408,"raw_hex":"e824faffff","size":5,"soft_fail":false,"target_va":5403843056,"va":5403844551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2711,"record_offset":8516,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2711,"record_offset":8600,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2711,"record_offset":2256,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2711,"record_offset":1588,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2711,"record_offset":2136,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2711,"record_offset":2072,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2711,"record_offset":8112,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2711,"record_offset":1560,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] system_color::wall_color_slots::grown_capacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/wall_color_slots.rs
//     evidence={"module":2711,"record_offset":2164,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":108359,"source_line":131,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] system_color::wall_color_slots::rebuild
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/wall_color_slots.rs
//     evidence={"module":2711,"record_offset":2316,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":108360,"source_line":172,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] system_color::wall_color_slots::write_slot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/wall_color_slots.rs
//     evidence={"module":2711,"record_offset":8720,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":108361,"source_line":155,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2711,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":1380,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2711,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":1380,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2711,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":1380,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2711,"record_offset":4152,"parent_record_offset":4064,"ancestor_record_offset":1380,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":1380,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":4092,"parent_record_offset":4064,"ancestor_record_offset":1380,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":1380,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":1380,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1380,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":1380,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2711,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1380,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2711,"record_offset":1828,"parent_record_offset":1796,"ancestor_record_offset":1380,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2711,"record_offset":3320,"parent_record_offset":2316,"ancestor_record_offset":1380,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2711,"record_offset":1796,"parent_record_offset":1764,"ancestor_record_offset":1380,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2711,"record_offset":8796,"parent_record_offset":8772,"ancestor_record_offset":1380,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2711,"record_offset":8844,"parent_record_offset":8820,"ancestor_record_offset":1380,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2711,"record_offset":2284,"parent_record_offset":2256,"ancestor_record_offset":1380,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2711,"record_offset":1732,"parent_record_offset":1616,"ancestor_record_offset":1380,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2711,"record_offset":1764,"parent_record_offset":1732,"ancestor_record_offset":1380,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2711,"record_offset":1616,"parent_record_offset":1588,"ancestor_record_offset":1380,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2711,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":1380,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2711,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":1380,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2711,"record_offset":1648,"parent_record_offset":1616,"ancestor_record_offset":1380,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":2711,"record_offset":2032,"parent_record_offset":1732,"ancestor_record_offset":1380,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2711,"record_offset":2196,"parent_record_offset":2164,"ancestor_record_offset":1380,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2711,"record_offset":2224,"parent_record_offset":2164,"ancestor_record_offset":1380,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2711,"record_offset":2568,"parent_record_offset":2316,"ancestor_record_offset":1380,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2711,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":1380,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2711,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":1380,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2711,"record_offset":2764,"parent_record_offset":2316,"ancestor_record_offset":1380,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2711,"record_offset":2736,"parent_record_offset":2316,"ancestor_record_offset":1380,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":2711,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":1380,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":2711,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":1380,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2711,"record_offset":9412,"parent_record_offset":9384,"ancestor_record_offset":1380,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2711,"record_offset":9440,"parent_record_offset":9412,"ancestor_record_offset":1380,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2711,"record_offset":4120,"parent_record_offset":4092,"ancestor_record_offset":1380,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2711,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":1380,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2711,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":1380,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2711,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":1380,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2711,"record_offset":3260,"parent_record_offset":2764,"ancestor_record_offset":1380,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2711,"record_offset":3160,"parent_record_offset":2884,"ancestor_record_offset":1380,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2711,"record_offset":3992,"parent_record_offset":2316,"ancestor_record_offset":1380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2711,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":1380,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2711,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":1380,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2711,"record_offset":8660,"parent_record_offset":8600,"ancestor_record_offset":1380,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2711,"record_offset":8628,"parent_record_offset":8600,"ancestor_record_offset":1380,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2711,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":1380,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2711,"record_offset":3348,"parent_record_offset":3320,"ancestor_record_offset":1380,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2711,"record_offset":3376,"parent_record_offset":3348,"ancestor_record_offset":1380,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2711,"record_offset":1968,"parent_record_offset":1796,"ancestor_record_offset":1380,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2711,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":1380,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2711,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":1380,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":2952,"parent_record_offset":2924,"ancestor_record_offset":1380,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":1380,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":2924,"parent_record_offset":2884,"ancestor_record_offset":1380,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2711,"record_offset":3048,"parent_record_offset":2884,"ancestor_record_offset":1380,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2711,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":1380,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":3104,"parent_record_offset":2884,"ancestor_record_offset":1380,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":2884,"parent_record_offset":2848,"ancestor_record_offset":1380,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":9352,"parent_record_offset":9320,"ancestor_record_offset":1380,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":9264,"parent_record_offset":9240,"ancestor_record_offset":1380,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":9320,"parent_record_offset":9264,"ancestor_record_offset":1380,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":9292,"parent_record_offset":9264,"ancestor_record_offset":1380,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":9384,"parent_record_offset":9352,"ancestor_record_offset":1380,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":1380,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":1380,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2711,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":1380,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2711,"record_offset":2624,"parent_record_offset":2316,"ancestor_record_offset":1380,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2711,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":1380,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2711,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":1380,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2711,"record_offset":2484,"parent_record_offset":2316,"ancestor_record_offset":1380,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2711,"record_offset":2792,"parent_record_offset":2764,"ancestor_record_offset":1380,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::grown_capacity
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A747
// 首条位置: {"module":2711,"inlinee":108359}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2711,"record_offset":2196,"parent_record_offset":2164,"ancestor_record_offset":2164,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2711,"record_offset":2224,"parent_record_offset":2164,"ancestor_record_offset":2164,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::GateColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":42528,"rva":17572448,"va":5386281568}
// 编译实例 RVAs: 0x10c2260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2817264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281568,"raw_hex":"e81003d5ff","size":5,"soft_fail":false,"target_va":5383464432,"va":5386281691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8721471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281568,"raw_hex":"e8c1eb7aff","size":5,"soft_fail":false,"target_va":5377560512,"va":5386281978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8154730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281568,"raw_hex":"e8969183ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5386281669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6565423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281568,"raw_hex":"e8d1d19bff","size":5,"soft_fail":false,"target_va":5379716240,"va":5386281658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142185160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17575624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281568,"raw_hex":"e8c82e0c01","size":5,"soft_fail":false,"target_va":5403857248,"va":5386281619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30339789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281568,"raw_hex":"e8cdf2ce01","size":5,"soft_fail":false,"target_va":5416621600,"va":5386281806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30339690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281568,"raw_hex":"e86af2ce01","size":5,"soft_fail":false,"target_va":5416621632,"va":5386281937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2561887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281568,"raw_hex":"e8a1e8d8ff","size":5,"soft_fail":false,"target_va":5383719888,"va":5386281770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386281568,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5386281640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":42784,"parent_record_offset":42528,"ancestor_record_offset":42528,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":42992,"parent_record_offset":42528,"ancestor_record_offset":42528,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":42724,"parent_record_offset":42528,"ancestor_record_offset":42528,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":42820,"parent_record_offset":42528,"ancestor_record_offset":42528,"inlinee":67148,"source_line":206,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":42944,"parent_record_offset":42876,"ancestor_record_offset":42528,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":42748,"parent_record_offset":42724,"ancestor_record_offset":42528,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":42848,"parent_record_offset":42820,"ancestor_record_offset":42528,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":42876,"parent_record_offset":42848,"ancestor_record_offset":42528,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1131,"record_offset":42916,"parent_record_offset":42876,"ancestor_record_offset":42528,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::HalftimberColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":40496,"rva":17570720,"va":5386279840}
// 编译实例 RVAs: 0x10c1ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2815536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279840,"raw_hex":"e8d009d5ff","size":5,"soft_fail":false,"target_va":5383464432,"va":5386279963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8719743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279840,"raw_hex":"e881f27aff","size":5,"soft_fail":false,"target_va":5377560512,"va":5386280250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8153002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279840,"raw_hex":"e8569883ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5386279941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6563695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279840,"raw_hex":"e891d89bff","size":5,"soft_fail":false,"target_va":5379716240,"va":5386279930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142185160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17577352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279840,"raw_hex":"e888350c01","size":5,"soft_fail":false,"target_va":5403857248,"va":5386279891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30341517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279840,"raw_hex":"e88df9ce01","size":5,"soft_fail":false,"target_va":5416621600,"va":5386280078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30341418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279840,"raw_hex":"e82af9ce01","size":5,"soft_fail":false,"target_va":5416621632,"va":5386280209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2560159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279840,"raw_hex":"e861efd8ff","size":5,"soft_fail":false,"target_va":5383719888,"va":5386280042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386279840,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5386279912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":40760,"parent_record_offset":40496,"ancestor_record_offset":40496,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":40968,"parent_record_offset":40496,"ancestor_record_offset":40496,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":40700,"parent_record_offset":40496,"ancestor_record_offset":40496,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":40796,"parent_record_offset":40496,"ancestor_record_offset":40496,"inlinee":67148,"source_line":206,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":40920,"parent_record_offset":40852,"ancestor_record_offset":40496,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":40724,"parent_record_offset":40700,"ancestor_record_offset":40496,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":40824,"parent_record_offset":40796,"ancestor_record_offset":40496,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":40852,"parent_record_offset":40824,"ancestor_record_offset":40496,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1131,"record_offset":40892,"parent_record_offset":40852,"ancestor_record_offset":40496,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::StoneFoundationColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":39984,"rva":17570288,"va":5386279408}
// 编译实例 RVAs: 0x10c19f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2815104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279408,"raw_hex":"e8800bd5ff","size":5,"soft_fail":false,"target_va":5383464432,"va":5386279531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8719311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279408,"raw_hex":"e831f47aff","size":5,"soft_fail":false,"target_va":5377560512,"va":5386279818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8152570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279408,"raw_hex":"e8069a83ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5386279509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6563263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279408,"raw_hex":"e841da9bff","size":5,"soft_fail":false,"target_va":5379716240,"va":5386279498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142185160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17577784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279408,"raw_hex":"e838370c01","size":5,"soft_fail":false,"target_va":5403857248,"va":5386279459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30341949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279408,"raw_hex":"e83dfbce01","size":5,"soft_fail":false,"target_va":5416621600,"va":5386279646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30341850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279408,"raw_hex":"e8daface01","size":5,"soft_fail":false,"target_va":5416621632,"va":5386279777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2559727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386279408,"raw_hex":"e811f1d8ff","size":5,"soft_fail":false,"target_va":5383719888,"va":5386279610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386279408,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5386279480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":40252,"parent_record_offset":39984,"ancestor_record_offset":39984,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":40460,"parent_record_offset":39984,"ancestor_record_offset":39984,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":40192,"parent_record_offset":39984,"ancestor_record_offset":39984,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":40288,"parent_record_offset":39984,"ancestor_record_offset":39984,"inlinee":67148,"source_line":206,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":40412,"parent_record_offset":40344,"ancestor_record_offset":39984,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":40216,"parent_record_offset":40192,"ancestor_record_offset":39984,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":40316,"parent_record_offset":40288,"ancestor_record_offset":39984,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":40344,"parent_record_offset":40316,"ancestor_record_offset":39984,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1131,"record_offset":40384,"parent_record_offset":40344,"ancestor_record_offset":39984,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::WallColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":41004,"rva":17571152,"va":5386280272}
// 编译实例 RVAs: 0x10c1d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2815968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280272,"raw_hex":"e82008d5ff","size":5,"soft_fail":false,"target_va":5383464432,"va":5386280395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8720175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280272,"raw_hex":"e8d1f07aff","size":5,"soft_fail":false,"target_va":5377560512,"va":5386280682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8153434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280272,"raw_hex":"e8a69683ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5386280373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6564127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280272,"raw_hex":"e8e1d69bff","size":5,"soft_fail":false,"target_va":5379716240,"va":5386280362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142185160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17576920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280272,"raw_hex":"e8d8330c01","size":5,"soft_fail":false,"target_va":5403857248,"va":5386280323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30341085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280272,"raw_hex":"e8ddf7ce01","size":5,"soft_fail":false,"target_va":5416621600,"va":5386280510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30340986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280272,"raw_hex":"e87af7ce01","size":5,"soft_fail":false,"target_va":5416621632,"va":5386280641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2560591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280272,"raw_hex":"e8b1edd8ff","size":5,"soft_fail":false,"target_va":5383719888,"va":5386280474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386280272,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5386280344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":41260,"parent_record_offset":41004,"ancestor_record_offset":41004,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":41468,"parent_record_offset":41004,"ancestor_record_offset":41004,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":41200,"parent_record_offset":41004,"ancestor_record_offset":41004,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":41296,"parent_record_offset":41004,"ancestor_record_offset":41004,"inlinee":67148,"source_line":206,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":41420,"parent_record_offset":41352,"ancestor_record_offset":41004,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":41224,"parent_record_offset":41200,"ancestor_record_offset":41004,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":41324,"parent_record_offset":41296,"ancestor_record_offset":41004,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":41352,"parent_record_offset":41324,"ancestor_record_offset":41004,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1131,"record_offset":41392,"parent_record_offset":41352,"ancestor_record_offset":41004,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::WoodenWallColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":43028,"rva":17572880,"va":5386282000}
// 编译实例 RVAs: 0x10c2410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2817696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386282000,"raw_hex":"e86001d5ff","size":5,"soft_fail":false,"target_va":5383464432,"va":5386282123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8721903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386282000,"raw_hex":"e811ea7aff","size":5,"soft_fail":false,"target_va":5377560512,"va":5386282410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8155162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386282000,"raw_hex":"e8e68f83ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5386282101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6565855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386282000,"raw_hex":"e821d09bff","size":5,"soft_fail":false,"target_va":5379716240,"va":5386282090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142185160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17575192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386282000,"raw_hex":"e8182d0c01","size":5,"soft_fail":false,"target_va":5403857248,"va":5386282051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30339357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386282000,"raw_hex":"e81df1ce01","size":5,"soft_fail":false,"target_va":5416621600,"va":5386282238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30339258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386282000,"raw_hex":"e8baf0ce01","size":5,"soft_fail":false,"target_va":5416621632,"va":5386282369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2562319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386282000,"raw_hex":"e8f1e6d8ff","size":5,"soft_fail":false,"target_va":5383719888,"va":5386282202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386282000,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5386282072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":43292,"parent_record_offset":43028,"ancestor_record_offset":43028,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":43500,"parent_record_offset":43028,"ancestor_record_offset":43028,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":43232,"parent_record_offset":43028,"ancestor_record_offset":43028,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":43328,"parent_record_offset":43028,"ancestor_record_offset":43028,"inlinee":67148,"source_line":206,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":43452,"parent_record_offset":43384,"ancestor_record_offset":43028,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":43256,"parent_record_offset":43232,"ancestor_record_offset":43028,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":43356,"parent_record_offset":43328,"ancestor_record_offset":43028,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":43384,"parent_record_offset":43356,"ancestor_record_offset":43028,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1131,"record_offset":43424,"parent_record_offset":43384,"ancestor_record_offset":43028,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id_roof::RoofColorIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":42020,"rva":17572016,"va":5386281136}
// 编译实例 RVAs: 0x10c20b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2816832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281136,"raw_hex":"e8c004d5ff","size":5,"soft_fail":false,"target_va":5383464432,"va":5386281259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8721039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281136,"raw_hex":"e871ed7aff","size":5,"soft_fail":false,"target_va":5377560512,"va":5386281546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8154298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281136,"raw_hex":"e8469383ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5386281237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6564991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281136,"raw_hex":"e881d39bff","size":5,"soft_fail":false,"target_va":5379716240,"va":5386281226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142185160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17576056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281136,"raw_hex":"e878300c01","size":5,"soft_fail":false,"target_va":5403857248,"va":5386281187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30340221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281136,"raw_hex":"e87df4ce01","size":5,"soft_fail":false,"target_va":5416621600,"va":5386281374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30340122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281136,"raw_hex":"e81af4ce01","size":5,"soft_fail":false,"target_va":5416621632,"va":5386281505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2561455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386281136,"raw_hex":"e851ead8ff","size":5,"soft_fail":false,"target_va":5383719888,"va":5386281338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386281136,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5386281208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":42284,"parent_record_offset":42020,"ancestor_record_offset":42020,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":42492,"parent_record_offset":42020,"ancestor_record_offset":42020,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":42224,"parent_record_offset":42020,"ancestor_record_offset":42020,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":42320,"parent_record_offset":42020,"ancestor_record_offset":42020,"inlinee":67148,"source_line":206,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":42444,"parent_record_offset":42376,"ancestor_record_offset":42020,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":42248,"parent_record_offset":42224,"ancestor_record_offset":42020,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":42348,"parent_record_offset":42320,"ancestor_record_offset":42020,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":42376,"parent_record_offset":42348,"ancestor_record_offset":42020,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1131,"record_offset":42416,"parent_record_offset":42376,"ancestor_record_offset":42020,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::write_color_slot<country_core::resources::walls::wall_render_slots::SlotWallIdSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1131,"record_offset":41504,"rva":17571584,"va":5386280704}
// 编译实例 RVAs: 0x10c1f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2816400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280704,"raw_hex":"e87006d5ff","size":5,"soft_fail":false,"target_va":5383464432,"va":5386280827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140870fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8720607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280704,"raw_hex":"e821ef7aff","size":5,"soft_fail":false,"target_va":5377560512,"va":5386281114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8153866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280704,"raw_hex":"e8f69483ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5386280805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6564559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280704,"raw_hex":"e831d59bff","size":5,"soft_fail":false,"target_va":5379716240,"va":5386280794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142185160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_color_slots.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17576488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280704,"raw_hex":"e828320c01","size":5,"soft_fail":false,"target_va":5403857248,"va":5386280755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30340653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280704,"raw_hex":"e82df6ce01","size":5,"soft_fail":false,"target_va":5416621600,"va":5386280942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30340554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280704,"raw_hex":"e8caf5ce01","size":5,"soft_fail":false,"target_va":5416621632,"va":5386281073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::update_buffer_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-2561023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386280704,"raw_hex":"e801ecd8ff","size":5,"soft_fail":false,"target_va":5383719888,"va":5386280906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386280704,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5386280776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1131,"record_offset":41776,"parent_record_offset":41504,"ancestor_record_offset":41504,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":41984,"parent_record_offset":41504,"ancestor_record_offset":41504,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":41716,"parent_record_offset":41504,"ancestor_record_offset":41504,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":41812,"parent_record_offset":41504,"ancestor_record_offset":41504,"inlinee":67148,"source_line":206,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1131,"record_offset":41936,"parent_record_offset":41868,"ancestor_record_offset":41504,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1131,"record_offset":41740,"parent_record_offset":41716,"ancestor_record_offset":41504,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":41840,"parent_record_offset":41812,"ancestor_record_offset":41504,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1131,"record_offset":41868,"parent_record_offset":41840,"ancestor_record_offset":41504,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1131,"record_offset":41908,"parent_record_offset":41868,"ancestor_record_offset":41504,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/01131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::write_slot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A749
// 首条位置: {"module":2711,"inlinee":108361}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::wall_color_slots::rebuild
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A748
// 首条位置: {"module":2711,"inlinee":108360}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2711,"record_offset":3320,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2711,"record_offset":2568,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2711,"record_offset":2764,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2711,"record_offset":2736,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2711,"record_offset":3992,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2711,"record_offset":2624,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2711,"record_offset":2484,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2711,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":2316,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2711,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":2316,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2711,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":2316,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2711,"record_offset":4152,"parent_record_offset":4064,"ancestor_record_offset":2316,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":4092,"parent_record_offset":4064,"ancestor_record_offset":2316,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":2316,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2711,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":2316,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2711,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":2316,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2711,"record_offset":4120,"parent_record_offset":4092,"ancestor_record_offset":2316,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2711,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":2316,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2711,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":2316,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2711,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":2316,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2711,"record_offset":3260,"parent_record_offset":2764,"ancestor_record_offset":2316,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2711,"record_offset":3160,"parent_record_offset":2884,"ancestor_record_offset":2316,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2711,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":2316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2711,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":2316,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2711,"record_offset":3348,"parent_record_offset":3320,"ancestor_record_offset":2316,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2711,"record_offset":3376,"parent_record_offset":3348,"ancestor_record_offset":2316,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2711,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":2316,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2711,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":2316,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":2952,"parent_record_offset":2924,"ancestor_record_offset":2316,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":2316,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2711,"record_offset":2924,"parent_record_offset":2884,"ancestor_record_offset":2316,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2711,"record_offset":3048,"parent_record_offset":2884,"ancestor_record_offset":2316,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2711,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":2316,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":3104,"parent_record_offset":2884,"ancestor_record_offset":2316,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":2884,"parent_record_offset":2848,"ancestor_record_offset":2316,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2711,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":2316,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2711,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":2316,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2711,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":2316,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2711,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":2316,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/02711.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2711,"record_offset":2792,"parent_record_offset":2764,"ancestor_record_offset":2316,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/02711.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_color_slots.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_color::wall_color_slots::ColorTables
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_color::wall_color_slots::ColorTables>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_color::wall_color_slots::ColorTables"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_color_slots.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_color_slots.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_reader::MessageReader<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallHoles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<system_color::set_roof_customization::SetRoofColorIdCmd>", "bevy_ecs::message::message_reader::MessageReader<system_color::set_roof_customization::SetFlatRoofStyleCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallHoles>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_roof_customization::SetRoofColorIdCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_roof_customization::SetFlatRoofStyleCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallHoles>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetMagicColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>", "bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>", "bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>", "bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairRailingCmd>", "bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairRoofedCmd>", "bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairMaterialCmd>", "bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetMagicColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairRailingCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairRoofedCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetStairMaterialCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::set_decorator_color_id::SetMagicColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetGateColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetStoneFoundationColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetStoneFoundationColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::ColorTables::colors_of
// 信息 ID: ff1df1cdcd0fe4249a9ec49e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181fc7 -> 0x21819f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x21823bc -> 0x21819f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::default_filled<country_core::resources::color_id::GateColorIdSsbo>
// 信息 ID: 47149aee81df2425b90df9d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::default_filled -> ["country_core::resources::color_id::GateColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e33 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e47 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e54 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e61 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e7b -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab0c4 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab14c -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab1f2 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab298 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab33e -> 0x10c1610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::default_filled<country_core::resources::color_id::HalftimberColorIdSsbo>
// 信息 ID: f14efb430fe9f5609f697cd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::default_filled -> ["country_core::resources::color_id::HalftimberColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e33 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e47 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e54 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e61 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e7b -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab0c4 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab14c -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab1f2 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab298 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab33e -> 0x10c1610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::default_filled<country_core::resources::color_id::StoneFoundationColorIdSsbo>
// 信息 ID: 775dcf017996dcdee362f464
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::default_filled -> ["country_core::resources::color_id::StoneFoundationColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e6e -> 0x10c1760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab3e4 -> 0x10c1760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::default_filled<country_core::resources::color_id::WallColorIdSsbo>
// 信息 ID: 8f8d00665ddba79c69f92dab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::default_filled -> ["country_core::resources::color_id::WallColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e33 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e47 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e54 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e61 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e7b -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab0c4 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab14c -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab1f2 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab298 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab33e -> 0x10c1610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::default_filled<country_core::resources::color_id::WoodenWallColorIdSsbo>
// 信息 ID: cdddb7f72f75ab165663579a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::default_filled -> ["country_core::resources::color_id::WoodenWallColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e33 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e47 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e54 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e61 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e7b -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab0c4 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab14c -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab1f2 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab298 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab33e -> 0x10c1610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::default_filled<country_core::resources::color_id_roof::RoofColorIdSsbo>
// 信息 ID: 4a806f4370222758d1b453ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::default_filled -> ["country_core::resources::color_id_roof::RoofColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e33 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e47 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e54 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e61 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e7b -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab0c4 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab14c -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab1f2 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab298 -> 0x10c1610 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab33e -> 0x10c1610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::default_filled<country_core::resources::walls::wall_render_slots::SlotWallIdSsbo>
// 信息 ID: c97c61195fb296fd7a9cedef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::default_filled -> ["country_core::resources::walls::wall_render_slots::SlotWallIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2181e8b -> 0x10c18b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::init_color_ids_buffer @ 0x28ab48a -> 0x10c18b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::grown_capacity
// 信息 ID: 17a046a41565a92904238e42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::wall_color_slots::sync_wall_color_slots; module=2711, inline_record=2164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::rebuild
// 信息 ID: dcf0b85f6bda21a54f291752
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::wall_color_slots::sync_wall_color_slots; module=2711, inline_record=2316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::sync_wall_color_slots
// 信息 ID: bee518750de827963679b827
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e3a52c9db3858bc766baf543: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> @ 0x10c128b -> 0x2181b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > @ 0x2167c81 -> 0x2181b50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::GateColorIdSsbo>
// 信息 ID: 8a5214ab9f954eae6552e9a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::write_color_slot -> ["country_core::resources::color_id::GateColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x21823e8 -> 0x10c2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_gate_color_id::set_gate_color_id @ 0x28ac37a -> 0x10c2260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::HalftimberColorIdSsbo>
// 信息 ID: db6342f876e8f42c7fd21139
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::write_color_slot -> ["country_core::resources::color_id::HalftimberColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x21823fe -> 0x10c1ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_halftimber_color_id::set_halftimber_color_id @ 0x2189fb6 -> 0x10c1ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::StoneFoundationColorIdSsbo>
// 信息 ID: c434f9dd647f2c8bbef48f1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::write_color_slot -> ["country_core::resources::color_id::StoneFoundationColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x218242a -> 0x10c19f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_halftimber_color_id::set_stone_foundation_color_id @ 0x218a836 -> 0x10c19f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::WallColorIdSsbo>
// 信息 ID: bd59f3c31b7e3390b2e43f50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::write_color_slot -> ["country_core::resources::color_id::WallColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x21823d2 -> 0x10c1d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_wall_color_id::set_wall_color_id @ 0x28aada2 -> 0x10c1d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::WoodenWallColorIdSsbo>
// 信息 ID: 9f9dc901eaf74ff3687c0f8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::write_color_slot -> ["country_core::resources::color_id::WoodenWallColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2182414 -> 0x10c2410 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::set_halftimber_color_id::set_wooden_wall_color_id @ 0x218a3c5 -> 0x10c2410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::write_color_slot<country_core::resources::color_id_roof::RoofColorIdSsbo>
// 信息 ID: b64503d756c8796d6179472b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::write_color_slot -> ["country_core::resources::color_id_roof::RoofColorIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2182440 -> 0x10c20b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::write_roof_color_id_to_gpu::write_roof_color_id_to_gpu @ 0x28ac10a -> 0x10c20b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::write_color_slot<country_core::resources::walls::wall_render_slots::SlotWallIdSsbo>
// 信息 ID: 9fd9bdf8d5098a9ed4fa1da3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_color::wall_color_slots::write_color_slot -> ["country_core::resources::walls::wall_render_slots::SlotWallIdSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::wall_color_slots::sync_wall_color_slots @ 0x2182456 -> 0x10c1f00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::wall_color_slots::write_slot
// 信息 ID: ca637b5936f8630163633b71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::wall_color_slots::sync_wall_color_slots; module=2711, inline_record=8720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
