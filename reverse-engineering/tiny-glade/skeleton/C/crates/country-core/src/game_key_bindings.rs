// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/game_key_bindings.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 48150129c3a78d7124f0009a2757b7e912eb407010634009a89d81170f5a3d81
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: game_key_bindings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 59, 166, 241, 510, 1643, 1665, 1709, 1795, 1816, 2644, 2652, 2661, 3944, 3955, 3956, 3962, 3965, 3966, 3968, 3969, 3970, 3974, 3981
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_input; // namespace candidate
//   package identities: [{"name":"bevy_input","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBinding::action_desc
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":7360,"rva":9627776,"va":5378336896}
// 编译实例 RVAs: 0x92e880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-2658273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378336896,"raw_hex":"e81f70d7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378336956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=10
//     target_aliases=1699 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9614754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378336896,"raw_hex":"e85e4a6dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378336941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38718871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378336896,"raw_hex":"e897cd4e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5378338180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6456831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378336896,"raw_hex":"e8ff856200","size":5,"soft_fail":false,"target_va":5384794864,"va":5378338028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t38282314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378336896,"raw_hex":"e84a244802","size":5,"soft_fail":false,"target_va":5416620794,"va":5378338475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4797377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378336896,"raw_hex":"e8c1334900","size":5,"soft_fail":false,"target_va":5383134720,"va":5378337338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":510,"record_offset":11716,"parent_record_offset":7360,"ancestor_record_offset":7360,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::KeyBindingDesc::icon
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":10640,"parent_record_offset":7360,"ancestor_record_offset":7360,"inlinee":29266,"source_line":611,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::KeyBindingDesc::text
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":7508,"parent_record_offset":7360,"ancestor_record_offset":7360,"inlinee":29265,"source_line":604,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":7888,"parent_record_offset":7864,"ancestor_record_offset":7360,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":7912,"parent_record_offset":7888,"ancestor_record_offset":7360,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":7360,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":7360,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":510,"record_offset":9796,"parent_record_offset":9768,"ancestor_record_offset":7360,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":7776,"parent_record_offset":7748,"ancestor_record_offset":7360,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":7836,"parent_record_offset":7804,"ancestor_record_offset":7360,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":7804,"parent_record_offset":7776,"ancestor_record_offset":7360,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":10264,"parent_record_offset":10212,"ancestor_record_offset":7360,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":7360,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":7360,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":7360,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":7628,"parent_record_offset":7600,"ancestor_record_offset":7360,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":510,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":7360,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":7360,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":7748,"parent_record_offset":7712,"ancestor_record_offset":7360,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":510,"record_offset":10300,"parent_record_offset":10264,"ancestor_record_offset":7360,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":510,"record_offset":7544,"parent_record_offset":7508,"ancestor_record_offset":7360,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":510,"record_offset":10372,"parent_record_offset":10336,"ancestor_record_offset":7360,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":510,"record_offset":10336,"parent_record_offset":10300,"ancestor_record_offset":7360,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":9768,"parent_record_offset":9676,"ancestor_record_offset":7360,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":510,"record_offset":7992,"parent_record_offset":7712,"ancestor_record_offset":7360,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":8016,"parent_record_offset":7992,"ancestor_record_offset":7360,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::mouse_button_to_string
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":54292,"rva":9645968,"va":5378355088}
// 编译实例 RVAs: 0x932f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::str::pattern::StrSearcher>::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6468982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e876b56200","size":5,"soft_fail":false,"target_va":5384824608,"va":5378355621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <u16>::_fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6457034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e8ca866200","size":5,"soft_fail":false,"target_va":5384812400,"va":5378355361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2676531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e8cd28d7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378355214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2678364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e8a421d7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378357063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9633008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e810036dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378355195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37996164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e884c64302","size":5,"soft_fail":false,"target_va":5416353040,"va":5378356871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38701815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e8f78a4e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5378355236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38265473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e881e24702","size":5,"soft_fail":false,"target_va":5416620794,"va":5378355316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t38264594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e812df4702","size":5,"soft_fail":false,"target_va":5416621762,"va":5378357163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38264068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e804dd4702","size":5,"soft_fail":false,"target_va":5416621376,"va":5378357303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4779542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378355088,"raw_hex":"e816ee4800","size":5,"soft_fail":false,"target_va":5383134720,"va":5378355173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":510,"record_offset":56980,"parent_record_offset":54292,"ancestor_record_offset":54292,"inlinee":16752,"source_line":268,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":55952,"parent_record_offset":54292,"ancestor_record_offset":54292,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":510,"record_offset":54444,"parent_record_offset":54292,"ancestor_record_offset":54292,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":61072,"parent_record_offset":54292,"ancestor_record_offset":54292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":55232,"parent_record_offset":55208,"ancestor_record_offset":54292,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":55256,"parent_record_offset":55232,"ancestor_record_offset":54292,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":60960,"parent_record_offset":60936,"ancestor_record_offset":54292,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":60984,"parent_record_offset":60960,"ancestor_record_offset":54292,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":55280,"parent_record_offset":55256,"ancestor_record_offset":54292,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":61008,"parent_record_offset":60984,"ancestor_record_offset":54292,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":55208,"parent_record_offset":55172,"ancestor_record_offset":54292,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":60936,"parent_record_offset":60880,"ancestor_record_offset":54292,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":57768,"parent_record_offset":57740,"ancestor_record_offset":54292,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":54636,"parent_record_offset":54612,"ancestor_record_offset":54292,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":57852,"parent_record_offset":57828,"ancestor_record_offset":54292,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":60908,"parent_record_offset":60880,"ancestor_record_offset":54292,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":60880,"parent_record_offset":60856,"ancestor_record_offset":54292,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":57828,"parent_record_offset":57796,"ancestor_record_offset":54292,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":57796,"parent_record_offset":57768,"ancestor_record_offset":54292,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":54684,"parent_record_offset":54660,"ancestor_record_offset":54292,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":54660,"parent_record_offset":54636,"ancestor_record_offset":54292,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":60856,"parent_record_offset":60832,"ancestor_record_offset":54292,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":54708,"parent_record_offset":54684,"ancestor_record_offset":54292,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":54540,"parent_record_offset":54516,"ancestor_record_offset":54292,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":54564,"parent_record_offset":54540,"ancestor_record_offset":54292,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":54588,"parent_record_offset":54564,"ancestor_record_offset":54292,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":54516,"parent_record_offset":54492,"ancestor_record_offset":54292,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":510,"record_offset":54492,"parent_record_offset":54468,"ancestor_record_offset":54292,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":57568,"parent_record_offset":56980,"ancestor_record_offset":54292,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":57072,"parent_record_offset":56980,"ancestor_record_offset":54292,"inlinee":6749,"source_line":484,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$104::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":56000,"parent_record_offset":55952,"ancestor_record_offset":54292,"inlinee":4516,"source_line":2973,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":56096,"parent_record_offset":56048,"ancestor_record_offset":54292,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":56048,"parent_record_offset":56000,"ancestor_record_offset":54292,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":54468,"parent_record_offset":54444,"ancestor_record_offset":54292,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":57688,"parent_record_offset":57648,"ancestor_record_offset":54292,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":57608,"parent_record_offset":57568,"ancestor_record_offset":54292,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":57940,"parent_record_offset":57688,"ancestor_record_offset":54292,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":57740,"parent_record_offset":57688,"ancestor_record_offset":54292,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":57100,"parent_record_offset":57072,"ancestor_record_offset":54292,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":54612,"parent_record_offset":54588,"ancestor_record_offset":54292,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":510,"record_offset":57648,"parent_record_offset":57608,"ancestor_record_offset":54292,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":510,"record_offset":54732,"parent_record_offset":54708,"ancestor_record_offset":54292,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":510,"record_offset":59284,"parent_record_offset":58732,"ancestor_record_offset":54292,"inlinee":6762,"source_line":645,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":510,"record_offset":59308,"parent_record_offset":59284,"ancestor_record_offset":54292,"inlinee":6763,"source_line":1843,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":510,"record_offset":58464,"parent_record_offset":58436,"ancestor_record_offset":54292,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":510,"record_offset":54780,"parent_record_offset":54756,"ancestor_record_offset":54292,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":510,"record_offset":58556,"parent_record_offset":58260,"ancestor_record_offset":54292,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":510,"record_offset":58660,"parent_record_offset":58604,"ancestor_record_offset":54292,"inlinee":10953,"source_line":217,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":510,"record_offset":58492,"parent_record_offset":58436,"ancestor_record_offset":54292,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":510,"record_offset":58436,"parent_record_offset":58408,"ancestor_record_offset":54292,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":510,"record_offset":58604,"parent_record_offset":58580,"ancestor_record_offset":54292,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":510,"record_offset":58408,"parent_record_offset":58260,"ancestor_record_offset":54292,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":510,"record_offset":58580,"parent_record_offset":58556,"ancestor_record_offset":54292,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":510,"record_offset":54756,"parent_record_offset":54732,"ancestor_record_offset":54292,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":510,"record_offset":58516,"parent_record_offset":58492,"ancestor_record_offset":54292,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":510,"record_offset":58684,"parent_record_offset":58660,"ancestor_record_offset":54292,"inlinee":10954,"source_line":1011,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":510,"record_offset":58984,"parent_record_offset":58936,"ancestor_record_offset":54292,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":510,"record_offset":55448,"parent_record_offset":55052,"ancestor_record_offset":54292,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":510,"record_offset":59920,"parent_record_offset":59896,"ancestor_record_offset":54292,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":55472,"parent_record_offset":55448,"ancestor_record_offset":54292,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":60784,"parent_record_offset":56980,"ancestor_record_offset":54292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":510,"record_offset":57904,"parent_record_offset":57688,"ancestor_record_offset":54292,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":510,"record_offset":59896,"parent_record_offset":59872,"ancestor_record_offset":54292,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":510,"record_offset":59872,"parent_record_offset":59848,"ancestor_record_offset":54292,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":510,"record_offset":59128,"parent_record_offset":59100,"ancestor_record_offset":54292,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":510,"record_offset":58192,"parent_record_offset":58160,"ancestor_record_offset":54292,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":510,"record_offset":58220,"parent_record_offset":58192,"ancestor_record_offset":54292,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":510,"record_offset":59824,"parent_record_offset":59788,"ancestor_record_offset":54292,"inlinee":7436,"source_line":137,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":510,"record_offset":59848,"parent_record_offset":59824,"ancestor_record_offset":54292,"inlinee":7437,"source_line":89,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":510,"record_offset":59100,"parent_record_offset":59028,"ancestor_record_offset":54292,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":510,"record_offset":58936,"parent_record_offset":58908,"ancestor_record_offset":54292,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":510,"record_offset":57512,"parent_record_offset":56980,"ancestor_record_offset":54292,"inlinee":16753,"source_line":2107,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":510,"record_offset":58020,"parent_record_offset":57984,"ancestor_record_offset":54292,"inlinee":16758,"source_line":1059,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":510,"record_offset":57984,"parent_record_offset":56980,"ancestor_record_offset":54292,"inlinee":16756,"source_line":511,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":510,"record_offset":58160,"parent_record_offset":58056,"ancestor_record_offset":54292,"inlinee":18836,"source_line":1461,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":510,"record_offset":57536,"parent_record_offset":57512,"ancestor_record_offset":54292,"inlinee":9296,"source_line":975,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":510,"record_offset":58732,"parent_record_offset":58056,"ancestor_record_offset":54292,"inlinee":18705,"source_line":1119,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":510,"record_offset":58056,"parent_record_offset":58020,"ancestor_record_offset":54292,"inlinee":18827,"source_line":1171,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":510,"record_offset":58908,"parent_record_offset":58872,"ancestor_record_offset":54292,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":510,"record_offset":58872,"parent_record_offset":58804,"ancestor_record_offset":54292,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":510,"record_offset":58804,"parent_record_offset":58732,"ancestor_record_offset":54292,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":510,"record_offset":59028,"parent_record_offset":58804,"ancestor_record_offset":54292,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":510,"record_offset":59192,"parent_record_offset":59028,"ancestor_record_offset":54292,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":510,"record_offset":59164,"parent_record_offset":59028,"ancestor_record_offset":54292,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 5, 108, 213, 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2588
// 首条位置: {"module":59,"inlinee":9608}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":37380,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":37316,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":59,"record_offset":37356,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":9608,"source_line":308,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":59,"record_offset":37260,"parent_record_offset":37196,"ancestor_record_offset":37196,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":59,"record_offset":37852,"parent_record_offset":37828,"ancestor_record_offset":37740,"inlinee":9609,"source_line":39,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":59,"record_offset":37876,"parent_record_offset":37852,"ancestor_record_offset":37740,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":59,"record_offset":37284,"parent_record_offset":37260,"ancestor_record_offset":37196,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::expecting
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 5, 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72CA
// 首条位置: {"module":510,"record_offset":172512,"rva":9679520,"va":5378388640}
// 编译实例 RVAs: 0x93b2a0, 0x93b2c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t6435419","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378388640,"raw_hex":"e95b326200","size":5,"soft_fail":false,"target_va":5384824080,"va":5378388656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=6
// 源码行: 5, 108, 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":59,"record_offset":55016,"rva":1997520,"va":5370706640}
// 编译实例 RVAs: 0x1e7ad0, 0x1e7c30, 0x1e8fd0, 0x19abfe0, 0x19ac140, 0x19ad4e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t45848943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370706640,"raw_hex":"e86f99bb02","size":5,"soft_fail":false,"target_va":5416555792,"va":5370706844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4971791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370706640,"raw_hex":"e80fdd4b00","size":5,"soft_fail":false,"target_va":5375678704,"va":5370706908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13912084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370706640,"raw_hex":"e81448d400","size":5,"soft_fail":false,"target_va":5384618896,"va":5370706807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":59,"record_offset":55204,"parent_record_offset":55016,"ancestor_record_offset":55016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":59,"record_offset":55372,"parent_record_offset":55348,"ancestor_record_offset":55016,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":59,"record_offset":55396,"parent_record_offset":55372,"ancestor_record_offset":55016,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":59,"record_offset":55420,"parent_record_offset":55396,"ancestor_record_offset":55016,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":59,"record_offset":55348,"parent_record_offset":55324,"ancestor_record_offset":55016,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":59,"record_offset":55324,"parent_record_offset":55300,"ancestor_record_offset":55016,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":59,"record_offset":55300,"parent_record_offset":55276,"ancestor_record_offset":55016,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":59,"record_offset":55228,"parent_record_offset":55204,"ancestor_record_offset":55016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=4
// 源码行: 5, 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":59,"record_offset":57292,"rva":2001360,"va":5370710480}
// 编译实例 RVAs: 0x1e89d0, 0x1e9e50, 0x19acee0, 0x19ae360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=162
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":59,"record_offset":57480,"parent_record_offset":57292,"ancestor_record_offset":57292,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=162
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":59,"record_offset":57508,"parent_record_offset":57480,"ancestor_record_offset":57292,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=162
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":59,"record_offset":57536,"parent_record_offset":57508,"ancestor_record_offset":57292,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=162
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":59,"record_offset":57564,"parent_record_offset":57536,"ancestor_record_offset":57292,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 5, 108, 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2586
// 首条位置: {"module":59,"inlinee":9606}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 5, 108, 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x447E
// 首条位置: {"module":241,"inlinee":17534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":9828,"parent_record_offset":9796,"ancestor_record_offset":9796,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":9796,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":9796,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":10176,"parent_record_offset":10152,"ancestor_record_offset":9796,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":241,"record_offset":10300,"parent_record_offset":10276,"ancestor_record_offset":9796,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":241,"record_offset":10276,"parent_record_offset":9828,"ancestor_record_offset":9796,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":9796,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":10104,"parent_record_offset":10080,"ancestor_record_offset":9796,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":10080,"parent_record_offset":9828,"ancestor_record_offset":9796,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":241,"record_offset":10012,"parent_record_offset":9988,"ancestor_record_offset":9796,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":241,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":9796,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":241,"record_offset":9988,"parent_record_offset":9964,"ancestor_record_offset":9796,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":241,"record_offset":9964,"parent_record_offset":9940,"ancestor_record_offset":9796,"inlinee":9618,"source_line":213,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":241,"record_offset":9912,"parent_record_offset":9888,"ancestor_record_offset":9796,"inlinee":9606,"source_line":108,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":174240,"parent_record_offset":174096,"ancestor_record_offset":174060,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":9940,"parent_record_offset":9828,"ancestor_record_offset":9796,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":9888,"parent_record_offset":9828,"ancestor_record_offset":9796,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 5, 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72C9
// 首条位置: {"module":510,"record_offset":172208,"rva":9679456,"va":5378388576}
// 编译实例 RVAs: 0x93b260, 0x93b280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t6435483","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378388576,"raw_hex":"e99b326200","size":5,"soft_fail":false,"target_va":5384824080,"va":5378388592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x44AB
// 首条位置: {"module":241,"inlinee":17579}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=80
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":241,"record_offset":140756,"parent_record_offset":139784,"ancestor_record_offset":139784,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":140720,"parent_record_offset":139784,"ancestor_record_offset":139784,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":157900,"parent_record_offset":139784,"ancestor_record_offset":139784,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":139836,"parent_record_offset":139784,"ancestor_record_offset":139784,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=80
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":140784,"parent_record_offset":139784,"ancestor_record_offset":139784,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":140336,"parent_record_offset":140312,"ancestor_record_offset":139784,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":140384,"parent_record_offset":140360,"ancestor_record_offset":139784,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":140360,"parent_record_offset":140336,"ancestor_record_offset":139784,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":241,"record_offset":140484,"parent_record_offset":140460,"ancestor_record_offset":139784,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":241,"record_offset":140460,"parent_record_offset":140124,"ancestor_record_offset":139784,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":140312,"parent_record_offset":140288,"ancestor_record_offset":139784,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":140288,"parent_record_offset":140264,"ancestor_record_offset":139784,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":140264,"parent_record_offset":140124,"ancestor_record_offset":139784,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=148
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":241,"record_offset":141016,"parent_record_offset":140992,"ancestor_record_offset":139784,"inlinee":6491,"source_line":217,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::mouse::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/mouse.rs
//     evidence={"module":241,"record_offset":141328,"parent_record_offset":141304,"ancestor_record_offset":139784,"inlinee":9612,"source_line":65,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":241,"record_offset":140012,"parent_record_offset":139984,"ancestor_record_offset":139784,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":140668,"parent_record_offset":139916,"ancestor_record_offset":139784,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":241,"record_offset":139984,"parent_record_offset":139956,"ancestor_record_offset":139784,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":241,"record_offset":140204,"parent_record_offset":140180,"ancestor_record_offset":139784,"inlinee":9606,"source_line":108,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":241,"record_offset":140088,"parent_record_offset":140052,"ancestor_record_offset":139784,"inlinee":17534,"source_line":108,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":139956,"parent_record_offset":139916,"ancestor_record_offset":139784,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":140124,"parent_record_offset":140088,"ancestor_record_offset":139784,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=74
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":140944,"parent_record_offset":140912,"ancestor_record_offset":139784,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":139916,"parent_record_offset":139876,"ancestor_record_offset":139784,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=80
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":140848,"parent_record_offset":140816,"ancestor_record_offset":139784,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":139876,"parent_record_offset":139836,"ancestor_record_offset":139784,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=80
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":140816,"parent_record_offset":140784,"ancestor_record_offset":139784,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":140236,"parent_record_offset":140124,"ancestor_record_offset":139784,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":140180,"parent_record_offset":140124,"ancestor_record_offset":139784,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=82
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":140052,"parent_record_offset":139916,"ancestor_record_offset":139784,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=148
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":241,"record_offset":140992,"parent_record_offset":140944,"ancestor_record_offset":139784,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=74
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":241,"record_offset":140912,"parent_record_offset":140880,"ancestor_record_offset":139784,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x44AC
// 首条位置: {"module":241,"inlinee":17580}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":161400,"parent_record_offset":159688,"ancestor_record_offset":159688,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":160264,"parent_record_offset":159688,"ancestor_record_offset":159688,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":159748,"parent_record_offset":159688,"ancestor_record_offset":159688,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":241,"record_offset":160084,"parent_record_offset":160060,"ancestor_record_offset":159688,"inlinee":6491,"source_line":217,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":241,"record_offset":159916,"parent_record_offset":159860,"ancestor_record_offset":159688,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":241,"record_offset":159888,"parent_record_offset":159860,"ancestor_record_offset":159688,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":160216,"parent_record_offset":159812,"ancestor_record_offset":159688,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":241,"record_offset":159860,"parent_record_offset":159812,"ancestor_record_offset":159688,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":160012,"parent_record_offset":159980,"ancestor_record_offset":159688,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":159812,"parent_record_offset":159780,"ancestor_record_offset":159688,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":159780,"parent_record_offset":159748,"ancestor_record_offset":159688,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":159948,"parent_record_offset":159812,"ancestor_record_offset":159688,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":241,"record_offset":160060,"parent_record_offset":160012,"ancestor_record_offset":159688,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":241,"record_offset":159980,"parent_record_offset":159948,"ancestor_record_offset":159688,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$0::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":175400,"rva":9681040,"va":5378390160}
// 编译实例 RVAs: 0x93b890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":241,"record_offset":173488,"rva":6476864,"va":5375185984}
// 编译实例 RVAs: 0x62d440, 0x1b580d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=244
//     evidence={"assembly":"\tcallq\t485425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375185984,"raw_hex":"e831680700","size":5,"soft_fail":false,"target_va":5375678704,"va":5375193274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e8fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4474469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375185984,"raw_hex":"e89bb9bbff","size":5,"soft_fail":false,"target_va":5370712016,"va":5375186480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e9e50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4470731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375185984,"raw_hex":"e835c8bbff","size":5,"soft_fail":false,"target_va":5370715728,"va":5375186454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140625a00
//     owner=unresolved_namespace; count=41
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375185984,"raw_hex":"e82283ffff","size":5,"soft_fail":false,"target_va":5375154688,"va":5375186649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ba00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3199503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375185984,"raw_hex":"e80fd23000","size":5,"soft_fail":false,"target_va":5378390528,"va":5375191020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419ad4e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1748453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397381328,"raw_hex":"e81b52e5ff","size":5,"soft_fail":false,"target_va":5395633376,"va":5397381824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419ae360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1744715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397381328,"raw_hex":"e8b560e5ff","size":5,"soft_fail":false,"target_va":5395637088,"va":5397381798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4de40
//     owner=unresolved_namespace; count=41
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397381328,"raw_hex":"e8d25affff","size":5,"soft_fail":false,"target_va":5397339712,"va":5397381993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d72130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41171284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375185984,"raw_hex":"e854397402","size":5,"soft_fail":false,"target_va":5416362288,"va":5375190999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd9080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19397652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397381328,"raw_hex":"e814fc2701","size":5,"soft_fail":false,"target_va":5416784000,"va":5397386343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41863826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375185984,"raw_hex":"e892ca7e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5375193225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":241,"record_offset":174720,"parent_record_offset":173488,"ancestor_record_offset":173488,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=246
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":241,"record_offset":185520,"parent_record_offset":173488,"ancestor_record_offset":173488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":174948,"parent_record_offset":173488,"ancestor_record_offset":173488,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":197740,"parent_record_offset":173488,"ancestor_record_offset":173488,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":173756,"parent_record_offset":173488,"ancestor_record_offset":173488,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=84
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":174748,"parent_record_offset":173488,"ancestor_record_offset":173488,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=242
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":241,"record_offset":185708,"parent_record_offset":185684,"ancestor_record_offset":173488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=242
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":241,"record_offset":185732,"parent_record_offset":185708,"ancestor_record_offset":173488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=242
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":241,"record_offset":185756,"parent_record_offset":185732,"ancestor_record_offset":173488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=242
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":241,"record_offset":185684,"parent_record_offset":185592,"ancestor_record_offset":173488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":174340,"parent_record_offset":174316,"ancestor_record_offset":173488,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=246
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":185620,"parent_record_offset":185592,"ancestor_record_offset":173488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=246
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":185592,"parent_record_offset":185568,"ancestor_record_offset":173488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":174388,"parent_record_offset":174364,"ancestor_record_offset":173488,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":174364,"parent_record_offset":174340,"ancestor_record_offset":173488,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=246
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":185568,"parent_record_offset":185544,"ancestor_record_offset":173488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":241,"record_offset":174488,"parent_record_offset":174464,"ancestor_record_offset":173488,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":241,"record_offset":174464,"parent_record_offset":174096,"ancestor_record_offset":173488,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":174316,"parent_record_offset":174292,"ancestor_record_offset":173488,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":174292,"parent_record_offset":174268,"ancestor_record_offset":173488,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":241,"record_offset":174268,"parent_record_offset":174096,"ancestor_record_offset":173488,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":241,"record_offset":174848,"parent_record_offset":174824,"ancestor_record_offset":173488,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=244
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":241,"record_offset":185652,"parent_record_offset":185620,"ancestor_record_offset":173488,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":241,"record_offset":174824,"parent_record_offset":174796,"ancestor_record_offset":173488,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=328
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":241,"record_offset":185544,"parent_record_offset":185520,"ancestor_record_offset":173488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":241,"record_offset":173988,"parent_record_offset":173924,"ancestor_record_offset":173488,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":241,"record_offset":173960,"parent_record_offset":173924,"ancestor_record_offset":173488,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":174672,"parent_record_offset":173844,"ancestor_record_offset":173488,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":241,"record_offset":173924,"parent_record_offset":173892,"ancestor_record_offset":173488,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":241,"record_offset":174180,"parent_record_offset":174156,"ancestor_record_offset":173488,"inlinee":9606,"source_line":108,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":241,"record_offset":174060,"parent_record_offset":174024,"ancestor_record_offset":173488,"inlinee":17534,"source_line":108,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":173892,"parent_record_offset":173844,"ancestor_record_offset":173488,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":174096,"parent_record_offset":174060,"ancestor_record_offset":173488,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":173844,"parent_record_offset":173800,"ancestor_record_offset":173488,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=84
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":174796,"parent_record_offset":174772,"ancestor_record_offset":173488,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":173800,"parent_record_offset":173756,"ancestor_record_offset":173488,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=84
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":174772,"parent_record_offset":174748,"ancestor_record_offset":173488,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":174240,"parent_record_offset":174096,"ancestor_record_offset":173488,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":174212,"parent_record_offset":174096,"ancestor_record_offset":173488,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":174156,"parent_record_offset":174096,"ancestor_record_offset":173488,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=84
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":174024,"parent_record_offset":173844,"ancestor_record_offset":173488,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=82
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":241,"record_offset":174904,"parent_record_offset":174880,"ancestor_record_offset":173488,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_seq<ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":241,"record_offset":223952,"rva":6493584,"va":5375202704}
// 编译实例 RVAs: 0x631590, 0x1b5c220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=406
//     evidence={"assembly":"\tcallq\t474612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375202704,"raw_hex":"e8f43d0700","size":5,"soft_fail":false,"target_va":5375678704,"va":5375204087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140625a00
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-48388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375202704,"raw_hex":"e8fc42ffff","size":5,"soft_fail":false,"target_va":5375154688,"va":5375203071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ba00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3187771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375202704,"raw_hex":"e83ba43000","size":5,"soft_fail":false,"target_va":5378390528,"va":5375202752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4de40
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-58708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397398048,"raw_hex":"e8ac1affff","size":5,"soft_fail":false,"target_va":5397339712,"va":5397398415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::de::impl$6::next_element<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> >,alloc::vec::Vec<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global> >
//     owner=package_namespace_candidate; count=64
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t47371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375202704,"raw_hex":"e80bb90000","size":5,"soft_fail":false,"target_va":5375257728,"va":5375210352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":241,"record_offset":225492,"parent_record_offset":223952,"ancestor_record_offset":223952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=64
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":227924,"parent_record_offset":223952,"ancestor_record_offset":223952,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=82
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":225464,"parent_record_offset":223952,"ancestor_record_offset":223952,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":224212,"parent_record_offset":223952,"ancestor_record_offset":223952,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":241,"record_offset":225688,"parent_record_offset":225664,"ancestor_record_offset":223952,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":241,"record_offset":225712,"parent_record_offset":225688,"ancestor_record_offset":223952,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":241,"record_offset":225736,"parent_record_offset":225712,"ancestor_record_offset":223952,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":241,"record_offset":225664,"parent_record_offset":225564,"ancestor_record_offset":223952,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":225596,"parent_record_offset":225564,"ancestor_record_offset":223952,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":225564,"parent_record_offset":225540,"ancestor_record_offset":223952,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":241,"record_offset":225540,"parent_record_offset":225516,"ancestor_record_offset":223952,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":241,"record_offset":225632,"parent_record_offset":225596,"ancestor_record_offset":223952,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=406
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":241,"record_offset":225516,"parent_record_offset":225492,"ancestor_record_offset":223952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":241,"record_offset":224368,"parent_record_offset":224308,"ancestor_record_offset":223952,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":241,"record_offset":224340,"parent_record_offset":224308,"ancestor_record_offset":223952,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":241,"record_offset":224456,"parent_record_offset":224268,"ancestor_record_offset":223952,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":241,"record_offset":224308,"parent_record_offset":224268,"ancestor_record_offset":223952,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":241,"record_offset":224268,"parent_record_offset":224240,"ancestor_record_offset":223952,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":224240,"parent_record_offset":224212,"ancestor_record_offset":223952,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":241,"record_offset":224400,"parent_record_offset":224268,"ancestor_record_offset":223952,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":241,"record_offset":224424,"parent_record_offset":224400,"ancestor_record_offset":223952,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$1::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":175524,"rva":9681408,"va":5378390528}
// 编译实例 RVAs: 0x93ba00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=35
//     evidence={"assembly":"\tcallq\t-2711901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378390528,"raw_hex":"e8a39ed6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378390584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=34
//     evidence={"assembly":"\tcallq\t-2716627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378390528,"raw_hex":"e82d8cd6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378395326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=35
//     target_aliases=1699 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9668382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378390528,"raw_hex":"e8e2786cff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378390569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=35
//     evidence={"assembly":"\tcallq\t38227527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378390528,"raw_hex":"e8474e4702","size":5,"soft_fail":false,"target_va":5416620769,"va":5378393237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":510,"record_offset":175660,"parent_record_offset":175524,"ancestor_record_offset":175524,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":182088,"parent_record_offset":175524,"ancestor_record_offset":175524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":175744,"parent_record_offset":175720,"ancestor_record_offset":175524,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":175768,"parent_record_offset":175744,"ancestor_record_offset":175524,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":182208,"parent_record_offset":182184,"ancestor_record_offset":175524,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":182232,"parent_record_offset":182208,"ancestor_record_offset":175524,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":175792,"parent_record_offset":175768,"ancestor_record_offset":175524,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":182256,"parent_record_offset":182232,"ancestor_record_offset":175524,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":175720,"parent_record_offset":175688,"ancestor_record_offset":175524,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":182184,"parent_record_offset":182160,"ancestor_record_offset":175524,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":510,"record_offset":175688,"parent_record_offset":175660,"ancestor_record_offset":175524,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":182160,"parent_record_offset":182136,"ancestor_record_offset":175524,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":182136,"parent_record_offset":182112,"ancestor_record_offset":175524,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":182112,"parent_record_offset":182088,"ancestor_record_offset":175524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2592
// 首条位置: {"module":241,"inlinee":9618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":241,"record_offset":9988,"parent_record_offset":9964,"ancestor_record_offset":9964,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":241,"record_offset":10012,"parent_record_offset":9988,"ancestor_record_offset":9964,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":241,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":9964,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":59,"record_offset":54340,"rva":1997056,"va":5370706176}
// 编译实例 RVAs: 0x1e7900, 0x19abe10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t45848332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370706176,"raw_hex":"e80c97bb02","size":5,"soft_fail":false,"target_va":5416554848,"va":5370706511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060e420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4352779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370706176,"raw_hex":"e80b6b4200","size":5,"soft_fail":false,"target_va":5375058976,"va":5370706192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140625e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4449517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370706176,"raw_hex":"e8ede44300","size":5,"soft_fail":false,"target_va":5375155792,"va":5370706270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140625ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4449492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370706176,"raw_hex":"e8d4e44300","size":5,"soft_fail":false,"target_va":5375155936,"va":5370706439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4d0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1708683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395627536,"raw_hex":"e88b121a00","size":5,"soft_fail":false,"target_va":5397336240,"va":5395627552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4df90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1712244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395627536,"raw_hex":"e874201a00","size":5,"soft_fail":false,"target_va":5397340048,"va":5395627799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4e0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1712717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395627536,"raw_hex":"e84d221a00","size":5,"soft_fail":false,"target_va":5397340352,"va":5395627630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":54784,"parent_record_offset":54340,"ancestor_record_offset":54340,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":54624,"parent_record_offset":54340,"ancestor_record_offset":54340,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":54820,"parent_record_offset":54340,"ancestor_record_offset":54340,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":59,"record_offset":54596,"parent_record_offset":54340,"ancestor_record_offset":54340,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":59,"record_offset":54660,"parent_record_offset":54340,"ancestor_record_offset":54340,"inlinee":6481,"source_line":2180,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":59,"record_offset":54936,"parent_record_offset":54912,"ancestor_record_offset":54340,"inlinee":6491,"source_line":259,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::mouse::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/mouse.rs
//     evidence={"module":59,"record_offset":54748,"parent_record_offset":54724,"ancestor_record_offset":54340,"inlinee":9612,"source_line":65,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$52::newtype_variant_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":59,"record_offset":54688,"parent_record_offset":54660,"ancestor_record_offset":54340,"inlinee":9610,"source_line":2860,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":59,"record_offset":54724,"parent_record_offset":54688,"ancestor_record_offset":54340,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14548
// 首条位置: {"module":1816,"inlinee":83272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$14::clone
// 记录形式: inline_instance, inlinee_declaration; instances=28
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A5F4
// 首条位置: {"module":2644,"inlinee":108020}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$17::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14543
// 首条位置: {"module":1709,"inlinee":83267}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1709,"record_offset":147864,"parent_record_offset":147808,"ancestor_record_offset":147808,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$20::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":1709,"record_offset":147968,"parent_record_offset":147940,"ancestor_record_offset":147808,"inlinee":11502,"source_line":211,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$28::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":1709,"record_offset":147900,"parent_record_offset":147864,"ancestor_record_offset":147808,"inlinee":11500,"source_line":253,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::mouse::impl$12::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/mouse.rs
//     evidence={"module":1709,"record_offset":148136,"parent_record_offset":148108,"ancestor_record_offset":147808,"inlinee":11544,"source_line":59,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1709,"record_offset":148032,"parent_record_offset":148004,"ancestor_record_offset":147808,"inlinee":11504,"source_line":1878,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1709,"record_offset":148060,"parent_record_offset":148004,"ancestor_record_offset":147808,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1709,"record_offset":147940,"parent_record_offset":147900,"ancestor_record_offset":147808,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::InputBinding::format_to_string
// 记录形式: compiled_procedure; instances=1
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":52620,"rva":9645488,"va":5378354608}
// 编译实例 RVAs: 0x932db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2676223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378354608,"raw_hex":"e8012ad7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378354906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2676053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378354608,"raw_hex":"e8ab2ad7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378354752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9632700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378354608,"raw_hex":"e844046dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378354887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38702123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378354608,"raw_hex":"e82b8c4e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5378354928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6440151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378354608,"raw_hex":"e8d7446200","size":5,"soft_fail":false,"target_va":5384794864,"va":5378354708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38265909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378354608,"raw_hex":"e835e44702","size":5,"soft_fail":false,"target_va":5416620794,"va":5378354880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::keycode_to_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378354608,"raw_hex":"e80b010000","size":5,"soft_fail":false,"target_va":5378355040,"va":5378354768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_key_bindings::mouse_button_to_string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378354608,"raw_hex":"e8a2010000","size":5,"soft_fail":false,"target_va":5378355088,"va":5378354665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":510,"record_offset":52784,"parent_record_offset":52620,"ancestor_record_offset":52620,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":510,"record_offset":53312,"parent_record_offset":52620,"ancestor_record_offset":52620,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":510,"record_offset":52864,"parent_record_offset":52620,"ancestor_record_offset":52620,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":53200,"parent_record_offset":52620,"ancestor_record_offset":52620,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":52892,"parent_record_offset":52620,"ancestor_record_offset":52620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":53700,"parent_record_offset":53676,"ancestor_record_offset":52620,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":53724,"parent_record_offset":53700,"ancestor_record_offset":52620,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":53080,"parent_record_offset":53052,"ancestor_record_offset":52620,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":53108,"parent_record_offset":53080,"ancestor_record_offset":52620,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":53748,"parent_record_offset":53724,"ancestor_record_offset":52620,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":53140,"parent_record_offset":53108,"ancestor_record_offset":52620,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":53676,"parent_record_offset":53524,"ancestor_record_offset":52620,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":53052,"parent_record_offset":52992,"ancestor_record_offset":52620,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":510,"record_offset":52832,"parent_record_offset":52808,"ancestor_record_offset":52620,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":53468,"parent_record_offset":53444,"ancestor_record_offset":52620,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":53024,"parent_record_offset":52992,"ancestor_record_offset":52620,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":52992,"parent_record_offset":52964,"ancestor_record_offset":52620,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":53524,"parent_record_offset":53492,"ancestor_record_offset":52620,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":53492,"parent_record_offset":53468,"ancestor_record_offset":52620,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":52964,"parent_record_offset":52940,"ancestor_record_offset":52620,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":53564,"parent_record_offset":53524,"ancestor_record_offset":52620,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":53368,"parent_record_offset":53336,"ancestor_record_offset":52620,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":53392,"parent_record_offset":53368,"ancestor_record_offset":52620,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":53416,"parent_record_offset":53392,"ancestor_record_offset":52620,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":53336,"parent_record_offset":53312,"ancestor_record_offset":52620,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":53444,"parent_record_offset":53416,"ancestor_record_offset":52620,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":510,"record_offset":53588,"parent_record_offset":53564,"ancestor_record_offset":52620,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":510,"record_offset":53636,"parent_record_offset":53612,"ancestor_record_offset":52620,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":510,"record_offset":53612,"parent_record_offset":53588,"ancestor_record_offset":52620,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":52808,"parent_record_offset":52784,"ancestor_record_offset":52620,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":510,"record_offset":53804,"parent_record_offset":53416,"ancestor_record_offset":52620,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":53828,"parent_record_offset":53804,"ancestor_record_offset":52620,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":52916,"parent_record_offset":52892,"ancestor_record_offset":52620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$3::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":1356,"rva":9626192,"va":5378335312}
// 编译实例 RVAs: 0x92e250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$4::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":1480,"rva":9626208,"va":5378335328}
// 编译实例 RVAs: 0x92e260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":59,"record_offset":37016,"rva":1991568,"va":5370700688}
// 编译实例 RVAs: 0x1e6390, 0x19ab3e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401fede0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370700688,"raw_hex":"e8238a0100","size":5,"soft_fail":false,"target_va":5370801632,"va":5370700728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140626100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4455322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370700688,"raw_hex":"e89afb4300","size":5,"soft_fail":false,"target_va":5375156480,"va":5370701153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14062ce10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4483570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370700688,"raw_hex":"e8f2694400","size":5,"soft_fail":false,"target_va":5375184400,"va":5370700825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140936420
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7666711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370700688,"raw_hex":"e817fc7400","size":5,"soft_fail":false,"target_va":5378368544,"va":5370701828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a52f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t686867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395624928,"raw_hex":"e8137b0a00","size":5,"soft_fail":false,"target_va":5396311840,"va":5395624968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4f360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1720050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395624928,"raw_hex":"e8f23e1a00","size":5,"soft_fail":false,"target_va":5397345120,"va":5395625065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b50e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1726650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395624928,"raw_hex":"e8ba581a00","size":5,"soft_fail":false,"target_va":5397352048,"va":5395625393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45659564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370700688,"raw_hex":"e8acb5b802","size":5,"soft_fail":false,"target_va":5416360864,"va":5370701295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46355990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370700688,"raw_hex":"e81656c302","size":5,"soft_fail":false,"target_va":5417057056,"va":5370701061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<country_core::game_key_bindings::KeyBindingsDe>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370700688,"raw_hex":"e8a03f0000","size":5,"soft_fail":false,"target_va":5370717392,"va":5370701099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t16242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370700688,"raw_hex":"e8723f0000","size":5,"soft_fail":false,"target_va":5370717632,"va":5370701385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":37488,"parent_record_offset":37016,"ancestor_record_offset":37016,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":59,"record_offset":37196,"parent_record_offset":37016,"ancestor_record_offset":37016,"inlinee":9608,"source_line":308,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":59,"record_offset":37520,"parent_record_offset":37488,"ancestor_record_offset":37016,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":37380,"parent_record_offset":37196,"ancestor_record_offset":37016,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":37316,"parent_record_offset":37196,"ancestor_record_offset":37016,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":59,"record_offset":37356,"parent_record_offset":37196,"ancestor_record_offset":37016,"inlinee":9608,"source_line":308,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":59,"record_offset":37260,"parent_record_offset":37196,"ancestor_record_offset":37016,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":59,"record_offset":37284,"parent_record_offset":37260,"ancestor_record_offset":37016,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":59,"record_offset":37556,"rva":1992784,"va":5370701904}
// 编译实例 RVAs: 0x1e6850, 0x19ab8a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402026b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t114224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370701904,"raw_hex":"e830be0100","size":5,"soft_fail":false,"target_va":5370816176,"va":5370701947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140626100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4454103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370701904,"raw_hex":"e8d7f64300","size":5,"soft_fail":false,"target_va":5375156480,"va":5370702372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14062ce10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4482351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370701904,"raw_hex":"e82f654400","size":5,"soft_fail":false,"target_va":5375184400,"va":5370702044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140936420
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7665492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370701904,"raw_hex":"e854f77400","size":5,"soft_fail":false,"target_va":5378368544,"va":5370703047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4f360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1718831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395626144,"raw_hex":"e82f3a1a00","size":5,"soft_fail":false,"target_va":5397345120,"va":5395626284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b50e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1725431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395626144,"raw_hex":"e8f7531a00","size":5,"soft_fail":false,"target_va":5397352048,"va":5395626612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45658345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370701904,"raw_hex":"e8e9b0b802","size":5,"soft_fail":false,"target_va":5416360864,"va":5370702514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46354771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370701904,"raw_hex":"e85351c302","size":5,"soft_fail":false,"target_va":5417057056,"va":5370702280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<country_core::game_key_bindings::KeyBindingsDe>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t15069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370701904,"raw_hex":"e8dd3a0000","size":5,"soft_fail":false,"target_va":5370717392,"va":5370702318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t15023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370701904,"raw_hex":"e8af3a0000","size":5,"soft_fail":false,"target_va":5370717632,"va":5370702604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":38088,"parent_record_offset":37556,"ancestor_record_offset":37556,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":59,"record_offset":37740,"parent_record_offset":37556,"ancestor_record_offset":37556,"inlinee":9608,"source_line":308,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":59,"record_offset":38120,"parent_record_offset":38088,"ancestor_record_offset":37556,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":37980,"parent_record_offset":37740,"ancestor_record_offset":37556,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":59,"record_offset":37916,"parent_record_offset":37740,"ancestor_record_offset":37556,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":59,"record_offset":37956,"parent_record_offset":37740,"ancestor_record_offset":37556,"inlinee":9608,"source_line":308,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":59,"record_offset":37852,"parent_record_offset":37828,"ancestor_record_offset":37556,"inlinee":9609,"source_line":39,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":59,"record_offset":37876,"parent_record_offset":37852,"ancestor_record_offset":37556,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":59,"record_offset":37804,"parent_record_offset":37740,"ancestor_record_offset":37556,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":59,"record_offset":37828,"parent_record_offset":37804,"ancestor_record_offset":37556,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/00059.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1709,"record_offset":123596,"rva":26913232,"va":5395622352}
// 编译实例 RVAs: 0x19aa9d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403e8ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22815814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395622352,"raw_hex":"e8badba3fe","size":5,"soft_fail":false,"target_va":5372808896,"va":5395624705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c1fb10
//     owner=unresolved_namespace; count=41
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2576572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395622352,"raw_hex":"e8bc502700","size":5,"soft_fail":false,"target_va":5398199056,"va":5395622479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c232e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2590923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395622352,"raw_hex":"e8cb882700","size":5,"soft_fail":false,"target_va":5398213344,"va":5395622416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1709,"record_offset":125056,"parent_record_offset":123596,"ancestor_record_offset":123596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1709,"record_offset":123812,"parent_record_offset":123596,"ancestor_record_offset":123596,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1709,"record_offset":124996,"parent_record_offset":123596,"ancestor_record_offset":123596,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1709,"record_offset":125108,"parent_record_offset":125080,"ancestor_record_offset":123596,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/ser/mod.rs
//     evidence={"module":1709,"record_offset":125080,"parent_record_offset":125056,"ancestor_record_offset":123596,"inlinee":7141,"source_line":1117,"archive":"_pdb/C/modules/01709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$5::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":87216,"rva":9659424,"va":5378368544}
// 编译实例 RVAs: 0x936420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140936890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378368544,"raw_hex":"e858040000","size":5,"soft_fail":false,"target_va":5378369680,"va":5378368563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":510,"record_offset":87348,"parent_record_offset":87216,"ancestor_record_offset":87216,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::impl$8::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":87400,"parent_record_offset":87376,"ancestor_record_offset":87216,"inlinee":29295,"source_line":394,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$6::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":175048,"rva":9679824,"va":5378388944}
// 编译实例 RVAs: 0x93b3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ba00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378388944,"raw_hex":"e81b060000","size":5,"soft_fail":false,"target_va":5378390528,"va":5378388960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":510,"record_offset":175180,"parent_record_offset":175048,"ancestor_record_offset":175048,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::impl$8::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":175204,"parent_record_offset":175180,"ancestor_record_offset":175048,"inlinee":29295,"source_line":394,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$7::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":87596,"rva":9660560,"va":5378369680}
// 编译实例 RVAs: 0x936890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=39
//     evidence={"assembly":"\tcallq\t-2691085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378369680,"raw_hex":"e8f3efd6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378369768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=121
//     evidence={"assembly":"\tcallq\t-2696613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378369680,"raw_hex":"e85bdad6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378375312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=39
//     target_aliases=1699 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9647566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378369680,"raw_hex":"e832ca6cff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378369753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093ba00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378369680,"raw_hex":"e8b7460000","size":5,"soft_fail":false,"target_va":5378390528,"va":5378372420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141616510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13496665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378369680,"raw_hex":"e859f1cd00","size":5,"soft_fail":false,"target_va":5391869200,"va":5378372530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38243976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378369680,"raw_hex":"e8888e4702","size":5,"soft_fail":false,"target_va":5416620769,"va":5378376788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=36
//     evidence={"assembly":"\tcallq\t38243924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378369680,"raw_hex":"e8548e4702","size":5,"soft_fail":false,"target_va":5416620794,"va":5378376865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":510,"record_offset":96032,"parent_record_offset":87596,"ancestor_record_offset":87596,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88672,"parent_record_offset":87596,"ancestor_record_offset":87596,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":110020,"parent_record_offset":87596,"ancestor_record_offset":87596,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=121
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":121156,"parent_record_offset":87596,"ancestor_record_offset":87596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::impl$7::from::closure$0
//     owner=private_source_ownership_candidate; count=36
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":87744,"parent_record_offset":87596,"ancestor_record_offset":87596,"inlinee":29297,"source_line":332,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":88300,"parent_record_offset":88276,"ancestor_record_offset":87596,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":88324,"parent_record_offset":88300,"ancestor_record_offset":87596,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=121
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":121344,"parent_record_offset":121320,"ancestor_record_offset":87596,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=121
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":121368,"parent_record_offset":121344,"ancestor_record_offset":87596,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":88348,"parent_record_offset":88324,"ancestor_record_offset":87596,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=121
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":121392,"parent_record_offset":121368,"ancestor_record_offset":87596,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":88276,"parent_record_offset":88244,"ancestor_record_offset":87596,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=121
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":121320,"parent_record_offset":121228,"ancestor_record_offset":87596,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":510,"record_offset":96060,"parent_record_offset":96032,"ancestor_record_offset":87596,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":88176,"parent_record_offset":88144,"ancestor_record_offset":87596,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=118
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":121256,"parent_record_offset":121228,"ancestor_record_offset":87596,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=121
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":121228,"parent_record_offset":121204,"ancestor_record_offset":87596,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":88244,"parent_record_offset":88208,"ancestor_record_offset":87596,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":88208,"parent_record_offset":88176,"ancestor_record_offset":87596,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=121
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":121204,"parent_record_offset":121180,"ancestor_record_offset":87596,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88432,"parent_record_offset":88408,"ancestor_record_offset":87596,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88696,"parent_record_offset":88672,"ancestor_record_offset":87596,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88112,"parent_record_offset":87916,"ancestor_record_offset":87596,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88144,"parent_record_offset":88112,"ancestor_record_offset":87596,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":87828,"parent_record_offset":87784,"ancestor_record_offset":87596,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88720,"parent_record_offset":88696,"ancestor_record_offset":87596,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88568,"parent_record_offset":88548,"ancestor_record_offset":87596,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":510,"record_offset":88408,"parent_record_offset":87916,"ancestor_record_offset":87596,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":510,"record_offset":87872,"parent_record_offset":87828,"ancestor_record_offset":87596,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":510,"record_offset":87916,"parent_record_offset":87872,"ancestor_record_offset":87596,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":510,"record_offset":88480,"parent_record_offset":88456,"ancestor_record_offset":87596,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":510,"record_offset":87968,"parent_record_offset":87916,"ancestor_record_offset":87596,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":510,"record_offset":88528,"parent_record_offset":88504,"ancestor_record_offset":87596,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":510,"record_offset":110076,"parent_record_offset":110048,"ancestor_record_offset":87596,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":87784,"parent_record_offset":87744,"ancestor_record_offset":87596,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":88504,"parent_record_offset":88480,"ancestor_record_offset":87596,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":88456,"parent_record_offset":88432,"ancestor_record_offset":87596,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":88548,"parent_record_offset":88528,"ancestor_record_offset":87596,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=118
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":510,"record_offset":121288,"parent_record_offset":121256,"ancestor_record_offset":87596,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::len
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":88064,"parent_record_offset":88032,"ancestor_record_offset":87596,"inlinee":6140,"source_line":810,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$20::size_hint
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":88032,"parent_record_offset":88000,"ancestor_record_offset":87596,"inlinee":6139,"source_line":2486,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$36::size_hint
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":88000,"parent_record_offset":87968,"ancestor_record_offset":87596,"inlinee":6137,"source_line":2620,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=121
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":121180,"parent_record_offset":121156,"ancestor_record_offset":87596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":88592,"parent_record_offset":88568,"ancestor_record_offset":87596,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::impl$7::from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":88740,"parent_record_offset":88720,"ancestor_record_offset":87596,"inlinee":29298,"source_line":339,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$7::from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=37
// 源码行: 332
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7271
// 首条位置: {"module":510,"inlinee":29297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":87784,"parent_record_offset":87744,"ancestor_record_offset":87744,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":88300,"parent_record_offset":88276,"ancestor_record_offset":87744,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":88324,"parent_record_offset":88300,"ancestor_record_offset":87744,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":88348,"parent_record_offset":88324,"ancestor_record_offset":87744,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":88276,"parent_record_offset":88244,"ancestor_record_offset":87744,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":88176,"parent_record_offset":88144,"ancestor_record_offset":87744,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":88244,"parent_record_offset":88208,"ancestor_record_offset":87744,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":88208,"parent_record_offset":88176,"ancestor_record_offset":87744,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88432,"parent_record_offset":88408,"ancestor_record_offset":87744,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88112,"parent_record_offset":87916,"ancestor_record_offset":87744,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88144,"parent_record_offset":88112,"ancestor_record_offset":87744,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":87828,"parent_record_offset":87784,"ancestor_record_offset":87744,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":88568,"parent_record_offset":88548,"ancestor_record_offset":87744,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":510,"record_offset":88408,"parent_record_offset":87916,"ancestor_record_offset":87744,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":510,"record_offset":87872,"parent_record_offset":87828,"ancestor_record_offset":87744,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":510,"record_offset":87916,"parent_record_offset":87872,"ancestor_record_offset":87744,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":510,"record_offset":88480,"parent_record_offset":88456,"ancestor_record_offset":87744,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":510,"record_offset":87968,"parent_record_offset":87916,"ancestor_record_offset":87744,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":510,"record_offset":88528,"parent_record_offset":88504,"ancestor_record_offset":87744,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":88504,"parent_record_offset":88480,"ancestor_record_offset":87744,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":88456,"parent_record_offset":88432,"ancestor_record_offset":87744,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":510,"record_offset":88548,"parent_record_offset":88528,"ancestor_record_offset":87744,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::len
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":88064,"parent_record_offset":88032,"ancestor_record_offset":87744,"inlinee":6140,"source_line":810,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$20::size_hint
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":88032,"parent_record_offset":88000,"ancestor_record_offset":87744,"inlinee":6139,"source_line":2486,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$36::size_hint
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":88000,"parent_record_offset":87968,"ancestor_record_offset":87744,"inlinee":6137,"source_line":2620,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":88592,"parent_record_offset":88568,"ancestor_record_offset":87744,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$7::from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 339
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7272
// 首条位置: {"module":510,"inlinee":29298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$8::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x726F
// 首条位置: {"module":510,"inlinee":29295}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindings::assign_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":16764,"rva":9629696,"va":5378338816}
// 编译实例 RVAs: 0x92f000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2665584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378338816,"raw_hex":"e89053d7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378344283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419aec80
//     owner=unresolved_namespace; count=78
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17300470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378338816,"raw_hex":"e8f6fb0701","size":5,"soft_fail":false,"target_va":5395639424,"va":5378338949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=39
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38706234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378338816,"raw_hex":"e83a9c4e02","size":5,"soft_fail":false,"target_va":5417057072,"va":5378350833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":510,"record_offset":49124,"parent_record_offset":16764,"ancestor_record_offset":16764,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":50120,"parent_record_offset":16764,"ancestor_record_offset":16764,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::impl$9::assign_input::closure$0
//     owner=private_source_ownership_candidate; count=39
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":16916,"parent_record_offset":16764,"ancestor_record_offset":16764,"inlinee":29268,"source_line":443,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":50308,"parent_record_offset":50284,"ancestor_record_offset":16764,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":50332,"parent_record_offset":50308,"ancestor_record_offset":16764,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":50356,"parent_record_offset":50332,"ancestor_record_offset":16764,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":50284,"parent_record_offset":50192,"ancestor_record_offset":16764,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":16764,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":50220,"parent_record_offset":50192,"ancestor_record_offset":16764,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":50192,"parent_record_offset":50168,"ancestor_record_offset":16764,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":16764,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":16764,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":50168,"parent_record_offset":50144,"ancestor_record_offset":16764,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17152,"parent_record_offset":17128,"ancestor_record_offset":16764,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17128,"parent_record_offset":17096,"ancestor_record_offset":16764,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17068,"parent_record_offset":16988,"ancestor_record_offset":16764,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":16952,"parent_record_offset":16916,"ancestor_record_offset":16764,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":16988,"parent_record_offset":16952,"ancestor_record_offset":16764,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17580,"parent_record_offset":16988,"ancestor_record_offset":16764,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17096,"parent_record_offset":16988,"ancestor_record_offset":16764,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17364,"parent_record_offset":16988,"ancestor_record_offset":16764,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17632,"parent_record_offset":17608,"ancestor_record_offset":16764,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":510,"record_offset":17300,"parent_record_offset":16988,"ancestor_record_offset":16764,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":510,"record_offset":17328,"parent_record_offset":17300,"ancestor_record_offset":16764,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":510,"record_offset":50252,"parent_record_offset":50220,"ancestor_record_offset":16764,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":17692,"parent_record_offset":17632,"ancestor_record_offset":16764,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":17468,"parent_record_offset":16988,"ancestor_record_offset":16764,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":17608,"parent_record_offset":16988,"ancestor_record_offset":16764,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":510,"record_offset":17440,"parent_record_offset":16988,"ancestor_record_offset":16764,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":510,"record_offset":17404,"parent_record_offset":17384,"ancestor_record_offset":16764,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":510,"record_offset":17268,"parent_record_offset":17096,"ancestor_record_offset":16764,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":49152,"parent_record_offset":49124,"ancestor_record_offset":16764,"inlinee":29272,"source_line":553,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::impl$9::assign_input::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=78
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":17384,"parent_record_offset":17364,"ancestor_record_offset":16764,"inlinee":29270,"source_line":444,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$9::assign_input::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=40
// 源码行: 443
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7254
// 首条位置: {"module":510,"inlinee":29268}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":16952,"parent_record_offset":16916,"ancestor_record_offset":16916,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":16916,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":16916,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":16916,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17152,"parent_record_offset":17128,"ancestor_record_offset":16916,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17128,"parent_record_offset":17096,"ancestor_record_offset":16916,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17068,"parent_record_offset":16988,"ancestor_record_offset":16916,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":16988,"parent_record_offset":16952,"ancestor_record_offset":16916,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17580,"parent_record_offset":16988,"ancestor_record_offset":16916,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17096,"parent_record_offset":16988,"ancestor_record_offset":16916,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17364,"parent_record_offset":16988,"ancestor_record_offset":16916,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":17632,"parent_record_offset":17608,"ancestor_record_offset":16916,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":510,"record_offset":17300,"parent_record_offset":16988,"ancestor_record_offset":16916,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":510,"record_offset":17328,"parent_record_offset":17300,"ancestor_record_offset":16916,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":17692,"parent_record_offset":17632,"ancestor_record_offset":16916,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":17468,"parent_record_offset":16988,"ancestor_record_offset":16916,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":17608,"parent_record_offset":16988,"ancestor_record_offset":16916,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":510,"record_offset":17440,"parent_record_offset":16988,"ancestor_record_offset":16916,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":510,"record_offset":17404,"parent_record_offset":17384,"ancestor_record_offset":16916,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":510,"record_offset":17268,"parent_record_offset":17096,"ancestor_record_offset":16916,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::impl$9::assign_input::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=78
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":510,"record_offset":17384,"parent_record_offset":17364,"ancestor_record_offset":16916,"inlinee":29270,"source_line":444,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$9::assign_input::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=80
// 源码行: 444
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7256
// 首条位置: {"module":510,"inlinee":29270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":510,"record_offset":17404,"parent_record_offset":17384,"ancestor_record_offset":17384,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$24::eq
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 553
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7258
// 首条位置: {"module":510,"inlinee":29272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindingDesc::text
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 604
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7251
// 首条位置: {"module":510,"inlinee":29265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":510,"record_offset":7544,"parent_record_offset":7508,"ancestor_record_offset":7508,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":510,"record_offset":9768,"parent_record_offset":9676,"ancestor_record_offset":9676,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":7888,"parent_record_offset":7864,"ancestor_record_offset":7508,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":7912,"parent_record_offset":7888,"ancestor_record_offset":7508,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":7508,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":510,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":7508,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":510,"record_offset":9796,"parent_record_offset":9768,"ancestor_record_offset":9676,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":7776,"parent_record_offset":7748,"ancestor_record_offset":7508,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":7836,"parent_record_offset":7804,"ancestor_record_offset":7508,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":7804,"parent_record_offset":7776,"ancestor_record_offset":7508,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":510,"record_offset":10264,"parent_record_offset":10212,"ancestor_record_offset":9676,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":7508,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":7508,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":7508,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":510,"record_offset":7628,"parent_record_offset":7600,"ancestor_record_offset":7508,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":510,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":7508,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":510,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":7508,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":510,"record_offset":7748,"parent_record_offset":7712,"ancestor_record_offset":7508,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":510,"record_offset":10300,"parent_record_offset":10264,"ancestor_record_offset":9676,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":510,"record_offset":10372,"parent_record_offset":10336,"ancestor_record_offset":9676,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":510,"record_offset":10336,"parent_record_offset":10300,"ancestor_record_offset":9676,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":510,"record_offset":7992,"parent_record_offset":7712,"ancestor_record_offset":7508,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":510,"record_offset":8016,"parent_record_offset":7992,"ancestor_record_offset":7508,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindingDesc::icon
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 611
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7252
// 首条位置: {"module":510,"inlinee":29266}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBinding::get_assigned_inputs
// 记录形式: compiled_procedure; instances=1
// 源码行: 671
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":16624,"rva":9629648,"va":5378338768}
// 编译实例 RVAs: 0x92efd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::keycode_to_str
// 记录形式: compiled_procedure; instances=1
// 源码行: 720
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":510,"record_offset":54168,"rva":9645920,"va":5378355040}
// 编译实例 RVAs: 0x932f60
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
// 完整结构及原始类型记录: game_key_bindings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::InputBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::game_key_bindings::InputBinding>", "constructor": "enum2$", "arguments": ["country_core::game_key_bindings::InputBinding"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::remove<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::remove", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global,tiny_glade::gui::settings::ui_settings_keybindings::closure_env$6>", "constructor": "alloc::vec::Vec::retain", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>", "alloc::alloc::Global", "tiny_glade::gui::settings::ui_settings_keybindings::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<enum2$<country_core::game_key_bindings::InputBinding> >", "constructor": "core::slice::iter::Iter", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<country_core::game_key_bindings::InputBinding> >,tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<enum2$<country_core::game_key_bindings::InputBinding> >", "tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<country_core::game_key_bindings::InputBinding> >,tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::string::String", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<country_core::game_key_bindings::InputBinding> >,tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<enum2$<country_core::game_key_bindings::InputBinding>,ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > > >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>", "ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<enum2$<country_core::game_key_bindings::InputBinding> >", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::private::de::content::impl$41::deserialize_seq<enum2$<ron::error::Error>,serde_core::de::impls::impl$19::deserialize::VecVisitor<enum2$<country_core::game_key_bindings::InputBinding> > >", "constructor": "serde::private::de::content::impl$41::deserialize_seq", "arguments": ["enum2$<ron::error::Error>", "serde_core::de::impls::impl$19::deserialize::VecVisitor<enum2$<country_core::game_key_bindings::InputBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$6::next_element<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> >,alloc::vec::Vec<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global> >", "constructor": "serde_core::de::impl$6::next_element", "arguments": ["serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> >", "alloc::vec::Vec<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::option::IntoIter<enum2$<bevy_input::keyboard::KeyCode> >,enum2$<country_core::game_key_bindings::InputBinding> (*)(enum2$<bevy_input::keyboard::KeyCode>)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::option::IntoIter<enum2$<bevy_input::keyboard::KeyCode> >", "enum2$<country_core::game_key_bindings::InputBinding> (*)(enum2$<bevy_input::keyboard::KeyCode>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<enum2$<country_core::game_key_bindings::InputBinding> >", "constructor": "core::iter::sources::once::Once", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::option::IntoIter<enum2$<bevy_input::keyboard::KeyCode> >,enum2$<country_core::game_key_bindings::InputBinding> (*)(enum2$<bevy_input::keyboard::KeyCode>)>,core::iter::sources::once::Once<enum2$<country_core::game_key_bindings::InputBinding> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<core::option::IntoIter<enum2$<bevy_input::keyboard::KeyCode> >,enum2$<country_core::game_key_bindings::InputBinding> (*)(enum2$<bevy_input::keyboard::KeyCode>)>", "core::iter::sources::once::Once<enum2$<country_core::game_key_bindings::InputBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<country_core::game_key_bindings::InputBinding>,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::option::IntoIter<enum2$<bevy_input::keyboard::KeyCode> >,enum2$<country_core::game_key_bindings::InputBinding> (*)(enum2$<bevy_input::keyboard::KeyCode>)>,core::iter::sources::once::Once<enum2$<country_core::game_key_bindings::InputBinding> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::option::IntoIter<enum2$<bevy_input::keyboard::KeyCode> >,enum2$<country_core::game_key_bindings::InputBinding> (*)(enum2$<bevy_input::keyboard::KeyCode>)>,core::iter::sources::once::Once<enum2$<country_core::game_key_bindings::InputBinding> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::cmp::impl$17::slice_contains<enum2$<country_core::game_key_bindings::InputBinding> >", "constructor": "core::slice::cmp::impl$17::slice_contains", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global> >", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "alloc::vec::Vec<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<enum2$<country_core::game_key_bindings::InputBinding> >", "constructor": "ref$", "arguments": ["enum2$<country_core::game_key_bindings::InputBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > >", "constructor": "core::option::Option", "arguments": ["ref$<enum2$<country_core::game_key_bindings::InputBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_onboarding::hint_binding::BindingHintDisplay::adjacent<enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > > >", "constructor": "system_onboarding::hint_binding::BindingHintDisplay::adjacent", "arguments": ["enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > >,4>", "constructor": "array$", "arguments": ["enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > >", "4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_onboarding::hint_binding::BindingHintDisplay::alternate<array$<enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > >,4> >", "constructor": "system_onboarding::hint_binding::BindingHintDisplay::alternate", "arguments": ["array$<enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > >,4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_onboarding::hint_binding::BindingHintDisplay::alternate<enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > > >", "constructor": "system_onboarding::hint_binding::BindingHintDisplay::alternate", "arguments": ["enum2$<core::option::Option<ref$<enum2$<country_core::game_key_bindings::InputBinding> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::game_key_bindings::KeyBinding", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >", "constructor": "tuple$", "arguments": ["ref$<str$>", "alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>", "enum2$<core::option::Option<alloc::string::String> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<alloc::string::String> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBindingDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBindings
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::game_key_bindings::KeyBindings>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::game_key_bindings::KeyBindings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::game_key_bindings::KeyBindings>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::game_key_bindings::KeyBindings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBindingsDe
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::game_key_bindings::KeyBindingsDe>", "constructor": "enum2$", "arguments": ["country_core::game_key_bindings::KeyBindingsDe"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<enum2$<country_core::game_key_bindings::KeyBindingsDe>,enum2$<ron::error::Error> >", "constructor": "core::result::Result", "arguments": ["enum2$<country_core::game_key_bindings::KeyBindingsDe>", "enum2$<ron::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<enum2$<country_core::game_key_bindings::KeyBindingsDe>,enum2$<ron::error::Error> > >", "constructor": "enum2$", "arguments": ["core::result::Result<enum2$<country_core::game_key_bindings::KeyBindingsDe>,enum2$<ron::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<country_core::game_key_bindings::KeyBindingsDe>,enum2$<ron::error::Error> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<enum2$<country_core::game_key_bindings::KeyBindingsDe>,enum2$<ron::error::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: game_key_bindings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: game_key_bindings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 21; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>)
// 候选参数类型: ["bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>"]
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_fmod::PlayGroupWallMoveAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::WallBendAudioParamReset>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_fmod::PlayGroupWallMoveAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::WallBendAudioParamReset>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_fmod::PlayGroupWallMoveAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::WallBendAudioParamReset>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::InputBinding::format_to_string
// 信息 ID: e2b8efaf220e04f47c9499d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<country_core::game_key_bindings::InputBinding> >,tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1::closure_env$0> > @ 0x3004f0 -> 0x932db0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$4 @ 0x312a0e -> 0x932db0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui::closure$7 @ 0x119b930 -> 0x932db0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_binding_renderer::render_hints::fmtinput @ 0x2b13b31 -> 0x932db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBinding::action_desc
// 信息 ID: 82e74ee563dbbde26f306062
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$3 @ 0x313234 -> 0x92e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x3c1aff -> 0x92e880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBinding::get_assigned_inputs
// 信息 ID: 35926b7e83b6f933bd9da520
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1 @ 0x30ed79 -> 0x92efd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1 @ 0x30eefc -> 0x92efd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindingDesc::icon
// 信息 ID: b7c55723566926991d5f0017
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=10640; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=10668; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=12372; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=12400; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=13324; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=13352; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=13464; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=13492; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindingDesc::text
// 信息 ID: 6922d25af3e5bdc2c5758cbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=7508; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=8080; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=8652; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=9164; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=9676; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=10696; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=11268; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=510, inline_record=11800; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindings::assign_input
// 信息 ID: 280d8d4efbd0022db1d3b58a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$5 @ 0x31271e -> 0x92f000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize
// 信息 ID: 116bcc254ad72d616215f95d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=59, inline_record=37196; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=59, inline_record=37356; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=59, inline_record=37416; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >; module=59, inline_record=37740; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >; module=59, inline_record=37956; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >; module=59, inline_record=38016; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<enum2$<country_core::game_key_bindings::InputBinding>,ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > > >; module=166, inline_record=8836; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<enum2$<country_core::game_key_bindings::InputBinding>,ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > > >; module=1665, inline_record=71160; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: e9c4030c4e5b6268fa4069d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> @ 0x1ea4b0 -> 0x93b2a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> | serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> @ 0x1ea490 -> 0x93b2c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor>; module=1709, inline_record=142220; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor>; module=1709, inline_record=142432; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 信息 ID: 06a10a35a6ac335eed0f8144
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_bytes -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: b77990d8bee01f3a0386d4b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=241, inline_record=9964; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=241, inline_record=10472; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=1795, inline_record=728; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=1795, inline_record=1236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 6e67d0caef7148a3e4f4d058
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x626721 -> 0x1e89d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > > @ 0x62d616 -> 0x1e9e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > > @ 0x1b582a6 -> 0x19ae360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: b19bda11812955fb33e2ff34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=241, inline_record=9912; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=241, inline_record=10248; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=241, inline_record=140204; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=241, inline_record=140432; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > >; module=241, inline_record=174180; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > >; module=241, inline_record=174436; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=1795, inline_record=676; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=1795, inline_record=1012; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 12ae2e75b47a607f0e2fbdd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=241, inline_record=9796; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=241, inline_record=140088; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > >; module=241, inline_record=174060; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<country_core::game_key_bindings::_::impl$0::deserialize::__Field> >; module=1795, inline_record=560; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=45172; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > >; module=1795, inline_record=82444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 10fa79d8b154b01f95f4495b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x1ea4c0 -> 0x93b260 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x1ea4a0 -> 0x93b280 (direct_jump_outside_procedure); 同地址别名未裁定。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1709, inline_record=142640; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1709, inline_record=142848; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 3e7414c5e2737c88e35fec4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::EnumRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x625df3 -> 0x1e7900 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x1b4e183 -> 0x19abe10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: eeb3bc33e56428b559bb7c82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=241, inline_record=139784; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=44836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > >
// 信息 ID: 811ce52273651103efa2a989
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::MapRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map -> ["ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x62ce75 -> 0x62d440 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x1b4f3c5 -> 0x1b580d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: fc3ad647d4dc57cf2e7a68e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=241, inline_record=159688; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=64644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_seq<ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > > >
// 信息 ID: 811587f03d0096b0b7cf4e6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::SeqRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_seq -> ["ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x62cf12 -> 0x631590 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x1b4f462 -> 0x1b5c220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 信息 ID: 287e9ace50502616632c1ff8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize -> ["ref_mut$<ron::de::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >
// 信息 ID: eb44c89546ee2cdc6d5d8f4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize -> ["ref_mut$<ron::de::tag::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::serialize
// 信息 ID: ddebd3f7dffb717bfefc3b26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<enum2$<country_core::game_key_bindings::InputBinding>,alloc::alloc::Global> >; module=1816, inline_record=72540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: 9395c8480d1d7c6a383ff490
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::game_key_bindings::KeyBindings> @ 0x1c1c479 -> 0x19aa9d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$0::default
// 信息 ID: 94cc37e3e00ceefe66da04cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x62903e -> 0x93b890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x626361 -> 0x93b890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$14::clone
// 信息 ID: 49b8b17b17cd6f05d3f80c07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<enum2$<system_onboarding::hint_binding::BindingHintDisplay>,alloc::alloc::Global>; module=2644, inline_record=12300; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::component::clone::component_clone_via_clone<enum2$<system_onboarding::hint_binding::BindingHintDisplay> >; module=2652, inline_record=1568; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_clutter_focus_edit::hint_clutter_focused_edit_hint; module=2661, inline_record=1532; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > >; module=3944, inline_record=4920; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_deselect_tool::hint_deselect_tool; module=3955, inline_record=5552; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_focused_edit::hint_focused_edit_hint; module=3956, inline_record=5080; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_unlink_windows::hint_unlink_windows; module=3962, inline_record=2132; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_disable_snapping::hint_disable_snapping; module=3965, inline_record=1836; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$17::eq
// 信息 ID: 50ee22a964f4a896d0b064a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::cmp::impl$17::slice_contains<enum2$<country_core::game_key_bindings::InputBinding> >; module=1709, inline_record=147808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$1::default
// 信息 ID: c7a1187a96453ca2e2736c1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > > @ 0x62e7ec -> 0x93ba00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_seq<ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > > > @ 0x6315c0 -> 0x93ba00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::impl$7::from @ 0x937344 -> 0x93ba00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::impl$6::default @ 0x93b3e0 -> 0x93ba00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map<ref_mut$<serde::private::de::content::MapRefDeserializer<enum2$<ron::error::Error> > > > @ 0x1b5947c -> 0x93ba00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_seq<ref_mut$<serde::private::de::content::SeqRefDeserializer<enum2$<ron::error::Error> > > > @ 0x1b5c250 -> 0x93ba00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$24::eq
// 信息 ID: 2c1c9a457f7a3dc0d10a3c15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=49152; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=50084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$3::from
// 信息 ID: 646672e960b58e5a0d36cdf5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x32ce85 -> 0x92e250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$4::from
// 信息 ID: 4e16e1c90ec26d24d6b1aee7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x32cf2d -> 0x92e260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$5::from
// 信息 ID: 471210f02a85c33e14bc471b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> > @ 0x1e6804 -> 0x936420 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> > @ 0x1e6cc7 -> 0x936420 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> > @ 0x19ab854 -> 0x936420 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_key_bindings::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> > @ 0x19abd17 -> 0x936420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$6::default
// 信息 ID: 6161b5a3e966dac2396e16f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::game_profile::_::impl$0::deserialize::__Visitor> @ 0x20a689 -> 0x93b3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$8 @ 0x312863 -> 0x93b3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::load_game_profile @ 0xa284f3 -> 0x93b3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::load_game_profile @ 0xa28819 -> 0x93b3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::load_game_profile @ 0xa2862b -> 0x93b3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::game_profile::_::impl$0::deserialize::__Visitor> @ 0x1a598c3 -> 0x93b3d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$7::from
// 信息 ID: f28525b3bf5fdfc9adce4b38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::game_key_bindings::impl$5::from @ 0x936433 -> 0x936890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$7::from::closure$0
// 信息 ID: fb7733beb5b57c2c7bf2537a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::game_key_bindings::impl$7::from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 36 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=87744; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=88776; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=89644; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=90548; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=91452; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=92356; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=93260; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=94164; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$7::from::closure$1
// 信息 ID: bdb473956b3ce2e49d50bb20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::game_key_bindings::impl$7::from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::impl$7::from; module=510, inline_record=88740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$8::from
// 信息 ID: 90dfd36a702dc2eaae57ab87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::impl$5::from; module=510, inline_record=87400; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::impl$6::default; module=510, inline_record=175204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$9::assign_input::closure$0
// 信息 ID: 9019c6ea4a6942ba57ee2013
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::game_key_bindings::impl$9::assign_input", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 39 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=16916; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=17768; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=18644; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=19508; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=20384; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=21252; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=22128; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=22992; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$9::assign_input::closure$0::closure$0
// 信息 ID: bd56383262339774f0e04991
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::game_key_bindings::impl$9::assign_input", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 78 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=17384; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=17520; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=18256; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=18396; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=19120; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=19260; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=19996; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_input; module=510, inline_record=20136; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::keycode_to_str
// 信息 ID: 5e24c2b837728cc2cd74abe9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::game_key_bindings::InputBinding::format_to_string @ 0x932e50 -> 0x932f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::mouse_button_to_string
// 信息 ID: be00520f1c4489af22e56672
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::game_key_bindings::InputBinding::format_to_string @ 0x932de9 -> 0x932f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_binding::HintMouseInput::compact_text @ 0x2b04bb8 -> 0x932f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_binding::HintMouseInput::compact_text @ 0x2b04c4b -> 0x932f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
