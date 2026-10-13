// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/inheritable.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5c19e731c2af936e8d8d2aacd637ee1e122731dfae6a9c2a530a2b3808f18ccf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: inheritable.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 533, 539, 546, 2059, 2144, 3385, 3574, 3612, 3613, 3614, 3615, 4500, 4501
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$7::clone
// 记录形式: inline_instance, inlinee_declaration; instances=76
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC0B
// 首条位置: {"module":3385,"inlinee":121867}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4316,"parent_record_offset":4276,"ancestor_record_offset":4276,"inlinee":121872,"source_line":70,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5980,"parent_record_offset":5932,"ancestor_record_offset":5932,"inlinee":121878,"source_line":196,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$20::clone
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":5296,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3385,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":4392,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":4276,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":4276,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4276,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":4276,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":4276,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4276,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=28
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4360,"parent_record_offset":4316,"ancestor_record_offset":4276,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4460,"parent_record_offset":4428,"ancestor_record_offset":4276,"inlinee":33159,"source_line":107,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5236,"parent_record_offset":5200,"ancestor_record_offset":5164,"inlinee":121874,"source_line":113,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5896,"parent_record_offset":5856,"ancestor_record_offset":5816,"inlinee":121876,"source_line":174,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5980,"parent_record_offset":5932,"ancestor_record_offset":5816,"inlinee":121878,"source_line":196,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":5164,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":5164,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3385,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":4276,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":4536,"parent_record_offset":4316,"ancestor_record_offset":4276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":5164,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::inherit_shallow
// 记录形式: inline_instance, inlinee_declaration; instances=42
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1802A
// 首条位置: {"module":2144,"inlinee":98346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":3968,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3614,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":1288,"inlinee":33159,"source_line":107,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3614,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":420,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3614,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":420,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3614,"record_offset":1356,"parent_record_offset":1324,"ancestor_record_offset":1288,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3614,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":420,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::as_concrete
// 记录形式: inline_instance, inlinee_declaration; instances=42
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E86
// 首条位置: {"module":533,"inlinee":32390}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::Inheritable::into_concrete
// 记录形式: inlinee_declaration; instances=6
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F7FD
// 首条位置: {"module":3574,"inlinee":129021}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":12140,"rva":41048512,"va":5409757632}
// 编译实例 RVAs: 0x27259c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-24962085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409757632,"raw_hex":"e8db1b83fe","size":5,"soft_fail":false,"target_va":5384795984,"va":5409758064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-34079510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409757632,"raw_hex":"e8eafcf7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409758209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6863892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409757632,"raw_hex":"e814bc6800","size":5,"soft_fail":false,"target_va":5416622208,"va":5409758311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6863948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409757632,"raw_hex":"e84cbc6800","size":5,"soft_fail":false,"target_va":5416622240,"va":5409758287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":12320,"parent_record_offset":12140,"ancestor_record_offset":12140,"inlinee":32390,"source_line":37,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":12668,"parent_record_offset":12140,"ancestor_record_offset":12140,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$10::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":12364,"parent_record_offset":12140,"ancestor_record_offset":12140,"inlinee":129643,"source_line":210,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":12908,"parent_record_offset":12140,"ancestor_record_offset":12140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":13164,"parent_record_offset":13140,"ancestor_record_offset":12140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":13188,"parent_record_offset":13164,"ancestor_record_offset":12140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":13212,"parent_record_offset":13188,"ancestor_record_offset":12140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":13140,"parent_record_offset":13084,"ancestor_record_offset":12140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":13112,"parent_record_offset":13084,"ancestor_record_offset":12140,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":12140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":12140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":12408,"parent_record_offset":12364,"ancestor_record_offset":12140,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":12468,"parent_record_offset":12440,"ancestor_record_offset":12140,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":12712,"parent_record_offset":12668,"ancestor_record_offset":12140,"inlinee":121878,"source_line":196,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3615,"record_offset":12496,"parent_record_offset":12468,"ancestor_record_offset":12140,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3615,"record_offset":12520,"parent_record_offset":12496,"ancestor_record_offset":12140,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":12940,"parent_record_offset":12908,"ancestor_record_offset":12140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":12544,"parent_record_offset":12520,"ancestor_record_offset":12140,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":9976,"rva":41047504,"va":5409756624}
// 编译实例 RVAs: 0x27255d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-24960953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409756624,"raw_hex":"e8472083fe","size":5,"soft_fail":false,"target_va":5384795984,"va":5409756932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-34078504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409756624,"raw_hex":"e8d800f8fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409757203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6864820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409756624,"raw_hex":"e8b4bf6800","size":5,"soft_fail":false,"target_va":5416622208,"va":5409757383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6864876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409756624,"raw_hex":"e8ecbf6800","size":5,"soft_fail":false,"target_va":5416622240,"va":5409757359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1427230d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-9620","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5409756624,"raw_hex":"e96cdaffff","size":5,"soft_fail":false,"target_va":5409747152,"va":5409756767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":10152,"parent_record_offset":9976,"ancestor_record_offset":9976,"inlinee":32390,"source_line":37,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":10196,"parent_record_offset":9976,"ancestor_record_offset":9976,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":11080,"parent_record_offset":9976,"ancestor_record_offset":9976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":9976,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":9976,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":9976,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":9976,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":11284,"parent_record_offset":11256,"ancestor_record_offset":9976,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":9976,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":9976,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":10276,"parent_record_offset":10232,"ancestor_record_offset":9976,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":10232,"parent_record_offset":10196,"ancestor_record_offset":9976,"inlinee":121872,"source_line":70,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":10380,"parent_record_offset":10348,"ancestor_record_offset":9976,"inlinee":33159,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3615,"record_offset":10348,"parent_record_offset":10308,"ancestor_record_offset":9976,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":10452,"parent_record_offset":10232,"ancestor_record_offset":9976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":8752,"rva":41046800,"va":5409755920}
// 编译实例 RVAs: 0x2725310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24960372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409755920,"raw_hex":"e88c2283fe","size":5,"soft_fail":false,"target_va":5384795984,"va":5409756351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-34077747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409755920,"raw_hex":"e8cd03f8fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409756446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1427259c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409755920,"raw_hex":"e83a060000","size":5,"soft_fail":false,"target_va":5409757632,"va":5409756033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6865654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409755920,"raw_hex":"e8f6c26800","size":5,"soft_fail":false,"target_va":5416622208,"va":5409756549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6865710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409755920,"raw_hex":"e82ec36800","size":5,"soft_fail":false,"target_va":5416622240,"va":5409756525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":8964,"parent_record_offset":8752,"ancestor_record_offset":8752,"inlinee":32390,"source_line":37,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":9008,"parent_record_offset":8752,"ancestor_record_offset":8752,"inlinee":98354,"source_line":99,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":9184,"parent_record_offset":8752,"ancestor_record_offset":8752,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":9520,"parent_record_offset":8752,"ancestor_record_offset":8752,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":8752,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":9864,"parent_record_offset":9840,"ancestor_record_offset":8752,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":8752,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3615,"record_offset":9816,"parent_record_offset":9760,"ancestor_record_offset":8752,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":9788,"parent_record_offset":9760,"ancestor_record_offset":8752,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":8752,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3615,"record_offset":9736,"parent_record_offset":9712,"ancestor_record_offset":8752,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$6::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":8752,"inlinee":95487,"source_line":152,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":9276,"parent_record_offset":9248,"ancestor_record_offset":8752,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$11::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":8752,"inlinee":98352,"source_line":223,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9248,"parent_record_offset":9212,"ancestor_record_offset":8752,"inlinee":121876,"source_line":174,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9316,"parent_record_offset":9276,"ancestor_record_offset":8752,"inlinee":121878,"source_line":196,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$8::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9040,"parent_record_offset":9008,"ancestor_record_offset":8752,"inlinee":98348,"source_line":182,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$9::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9068,"parent_record_offset":9008,"ancestor_record_offset":8752,"inlinee":98350,"source_line":189,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3615,"record_offset":9380,"parent_record_offset":9352,"ancestor_record_offset":8752,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3615,"record_offset":9404,"parent_record_offset":9380,"ancestor_record_offset":8752,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3615,"record_offset":9212,"parent_record_offset":9184,"ancestor_record_offset":8752,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":9544,"parent_record_offset":9520,"ancestor_record_offset":8752,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":9428,"parent_record_offset":9404,"ancestor_record_offset":8752,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":13288,"rva":41049264,"va":5409758384}
// 编译实例 RVAs: 0x2725cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142722f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409758384,"raw_hex":"e84fd2ffff","size":5,"soft_fail":false,"target_va":5409746784,"va":5409758476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6862987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409758384,"raw_hex":"e88bb86800","size":5,"soft_fail":false,"target_va":5416622208,"va":5409759216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6863043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409758384,"raw_hex":"e8c3b86800","size":5,"soft_fail":false,"target_va":5416622240,"va":5409759192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":13492,"parent_record_offset":13288,"ancestor_record_offset":13288,"inlinee":32390,"source_line":37,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":13536,"parent_record_offset":13288,"ancestor_record_offset":13288,"inlinee":98354,"source_line":99,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":13856,"parent_record_offset":13288,"ancestor_record_offset":13288,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":13632,"parent_record_offset":13572,"ancestor_record_offset":13288,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":13972,"parent_record_offset":13920,"ancestor_record_offset":13288,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":13920,"parent_record_offset":13888,"ancestor_record_offset":13288,"inlinee":121874,"source_line":113,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$6::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":13572,"parent_record_offset":13536,"ancestor_record_offset":13288,"inlinee":98345,"source_line":136,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3615,"record_offset":14000,"parent_record_offset":13972,"ancestor_record_offset":13288,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3615,"record_offset":14024,"parent_record_offset":14000,"ancestor_record_offset":13288,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3615,"record_offset":13888,"parent_record_offset":13856,"ancestor_record_offset":13288,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":13288,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$2::concrete_default
// 记录形式: inlinee_declaration; instances=6
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17501
// 首条位置: {"module":2059,"inlinee":95489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$3::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18032
// 首条位置: {"module":3615,"inlinee":98354}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$6::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":13572,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":98345,"source_line":136,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$8::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9040,"parent_record_offset":9008,"ancestor_record_offset":9008,"inlinee":98348,"source_line":182,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$9::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9068,"parent_record_offset":9008,"ancestor_record_offset":9008,"inlinee":98350,"source_line":189,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":13632,"parent_record_offset":13572,"ancestor_record_offset":13536,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$6::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":9008,"inlinee":95487,"source_line":152,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$11::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":9008,"inlinee":98352,"source_line":223,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3385,"record_offset":260,"rva":40181152,"va":5408890272}
// 编译实例 RVAs: 0x2651da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-24094963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408890272,"raw_hex":"e80d5790fe","size":5,"soft_fail":false,"target_va":5384795984,"va":5408890942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-33211925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408890272,"raw_hex":"e8eb3905fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5408890624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140993800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30143205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408890272,"raw_hex":"e81b0d34fe","size":5,"soft_fail":false,"target_va":5378750464,"va":5408893664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142650c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408890272,"raw_hex":"e8deebffff","size":5,"soft_fail":false,"target_va":5408885808,"va":5408890957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142725310
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t865326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408890272,"raw_hex":"e82e340d00","size":5,"soft_fail":false,"target_va":5409755920,"va":5408890589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1427255d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t866061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408890272,"raw_hex":"e80d370d00","size":5,"soft_fail":false,"target_va":5409756624,"va":5408890558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142725cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t867807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408890272,"raw_hex":"e8df3d0d00","size":5,"soft_fail":false,"target_va":5409758384,"va":5408890572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..prefab..config..PrefabRenderConfig$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8772853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408890272,"raw_hex":"e80b237aff","size":5,"soft_fail":false,"target_va":5400120464,"va":5408893312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::prefab::config::impl$2::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":2236,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":121861,"source_line":44,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":2264,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":121863,"source_line":55,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":2336,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::and_modify
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":3864,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":26227,"source_line":2589,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":4104,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12158,"source_line":2511,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":4048,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8388,"source_line":975,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":1404,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":20111,"source_line":576,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$35::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8500,"source_line":2025,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":768,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$55::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":1712,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21910,"source_line":2271,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":260,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":260,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":260,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":2584,"parent_record_offset":2528,"ancestor_record_offset":260,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":2556,"parent_record_offset":2528,"ancestor_record_offset":260,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":260,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":260,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":2300,"parent_record_offset":2264,"ancestor_record_offset":260,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$5::inherit_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":3892,"parent_record_offset":3864,"ancestor_record_offset":260,"inlinee":121865,"source_line":131,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$5::inherit_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4144,"parent_record_offset":4104,"ancestor_record_offset":260,"inlinee":121868,"source_line":132,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":260,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4172,"parent_record_offset":4144,"ancestor_record_offset":260,"inlinee":121870,"source_line":26,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4316,"parent_record_offset":4276,"ancestor_record_offset":260,"inlinee":121872,"source_line":70,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4460,"parent_record_offset":4428,"ancestor_record_offset":260,"inlinee":33159,"source_line":107,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5236,"parent_record_offset":5200,"ancestor_record_offset":260,"inlinee":121874,"source_line":113,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5896,"parent_record_offset":5856,"ancestor_record_offset":260,"inlinee":121876,"source_line":174,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5980,"parent_record_offset":5932,"ancestor_record_offset":260,"inlinee":121878,"source_line":196,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":3924,"parent_record_offset":3892,"ancestor_record_offset":260,"inlinee":121863,"source_line":55,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":260,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":260,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3385,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":260,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3385,"record_offset":6452,"parent_record_offset":6424,"ancestor_record_offset":260,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3385,"record_offset":6608,"parent_record_offset":6556,"ancestor_record_offset":260,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3385,"record_offset":6988,"parent_record_offset":6948,"ancestor_record_offset":260,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3385,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":260,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3385,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":260,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3385,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":260,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3385,"record_offset":716,"parent_record_offset":636,"ancestor_record_offset":260,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":6480,"parent_record_offset":6452,"ancestor_record_offset":260,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3385,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":260,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3385,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":260,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":260,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":260,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3385,"record_offset":956,"parent_record_offset":924,"ancestor_record_offset":260,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3385,"record_offset":988,"parent_record_offset":956,"ancestor_record_offset":260,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3385,"record_offset":1076,"parent_record_offset":924,"ancestor_record_offset":260,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3385,"record_offset":924,"parent_record_offset":884,"ancestor_record_offset":260,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3385,"record_offset":6424,"parent_record_offset":6380,"ancestor_record_offset":260,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3385,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":260,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3385,"record_offset":6556,"parent_record_offset":6520,"ancestor_record_offset":260,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":3385,"record_offset":7028,"parent_record_offset":6292,"ancestor_record_offset":260,"inlinee":7447,"source_line":28,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3385,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":260,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3385,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":260,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3385,"record_offset":800,"parent_record_offset":768,"ancestor_record_offset":260,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3385,"record_offset":1768,"parent_record_offset":1740,"ancestor_record_offset":260,"inlinee":13411,"source_line":3267,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":260,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":1312,"parent_record_offset":800,"ancestor_record_offset":260,"inlinee":8503,"source_line":573,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":260,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":1164,"parent_record_offset":884,"ancestor_record_offset":260,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":7140,"parent_record_offset":6292,"ancestor_record_offset":260,"inlinee":7448,"source_line":561,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":636,"parent_record_offset":580,"ancestor_record_offset":260,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":884,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":6292,"parent_record_offset":6264,"ancestor_record_offset":260,"inlinee":7438,"source_line":1114,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":260,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":7056,"parent_record_offset":6292,"ancestor_record_offset":260,"inlinee":7446,"source_line":2382,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":260,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":6380,"parent_record_offset":6340,"ancestor_record_offset":260,"inlinee":7440,"source_line":1992,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":6520,"parent_record_offset":6380,"ancestor_record_offset":260,"inlinee":7441,"source_line":1787,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":6704,"parent_record_offset":6380,"ancestor_record_offset":260,"inlinee":7443,"source_line":1750,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":6740,"parent_record_offset":6704,"ancestor_record_offset":260,"inlinee":7444,"source_line":2685,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":260,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":6340,"parent_record_offset":6292,"ancestor_record_offset":260,"inlinee":7439,"source_line":1950,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":260,"inlinee":7463,"source_line":2606,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":6924,"parent_record_offset":6340,"ancestor_record_offset":260,"inlinee":7445,"source_line":2540,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3385,"record_offset":840,"parent_record_offset":800,"ancestor_record_offset":260,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":3385,"record_offset":6264,"parent_record_offset":6236,"ancestor_record_offset":260,"inlinee":7470,"source_line":512,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":260,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":260,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":6236,"parent_record_offset":4104,"ancestor_record_offset":260,"inlinee":8397,"source_line":2881,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$41::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":1740,"parent_record_offset":1712,"ancestor_record_offset":260,"inlinee":21911,"source_line":2113,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3385,"record_offset":4072,"parent_record_offset":4048,"ancestor_record_offset":260,"inlinee":19384,"source_line":2976,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$5::inherit_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC09
// 首条位置: {"module":3385,"inlinee":121865}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$3::inherit_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":3924,"parent_record_offset":3892,"ancestor_record_offset":3892,"inlinee":121863,"source_line":55,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":3968,"parent_record_offset":3924,"ancestor_record_offset":3892,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":3892,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$5::inherit_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC0C
// 首条位置: {"module":3385,"inlinee":121868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4172,"parent_record_offset":4144,"ancestor_record_offset":4144,"inlinee":121870,"source_line":26,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":4144,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":4144,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4144,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":4144,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":4144,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4144,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4248,"parent_record_offset":4172,"ancestor_record_offset":4144,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4316,"parent_record_offset":4276,"ancestor_record_offset":4144,"inlinee":121872,"source_line":70,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4460,"parent_record_offset":4428,"ancestor_record_offset":4144,"inlinee":33159,"source_line":107,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5236,"parent_record_offset":5200,"ancestor_record_offset":4144,"inlinee":121874,"source_line":113,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5896,"parent_record_offset":5856,"ancestor_record_offset":4144,"inlinee":121876,"source_line":174,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5980,"parent_record_offset":5932,"ancestor_record_offset":4144,"inlinee":121878,"source_line":196,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":4144,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":4144,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3385,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":4144,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":4536,"parent_record_offset":4316,"ancestor_record_offset":4144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4144,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::impl$6::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x174FF
// 首条位置: {"module":2059,"inlinee":95487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::impl$11::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":9092,"inlinee":98352,"source_line":223,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable
// 记录形式: inline_instance, inlinee_declaration; instances=56
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA3A
// 首条位置: {"module":3612,"inlinee":129594}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3612,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":8496,"inlinee":98343,"source_line":232,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3612,"record_offset":8200,"parent_record_offset":8148,"ancestor_record_offset":8148,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3612,"record_offset":8172,"parent_record_offset":8148,"ancestor_record_offset":8148,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":42120,"parent_record_offset":42096,"ancestor_record_offset":42096,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":23108,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":31996,"parent_record_offset":31972,"ancestor_record_offset":31380,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":32020,"parent_record_offset":31996,"ancestor_record_offset":31380,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":32044,"parent_record_offset":32020,"ancestor_record_offset":31380,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":31972,"parent_record_offset":31824,"ancestor_record_offset":31380,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31764,"parent_record_offset":31736,"ancestor_record_offset":31380,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31824,"parent_record_offset":31792,"ancestor_record_offset":31380,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31792,"parent_record_offset":31764,"ancestor_record_offset":31380,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31860,"parent_record_offset":31824,"ancestor_record_offset":31380,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31652,"parent_record_offset":31624,"ancestor_record_offset":31380,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31680,"parent_record_offset":31652,"ancestor_record_offset":31380,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31708,"parent_record_offset":31680,"ancestor_record_offset":31380,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31624,"parent_record_offset":31596,"ancestor_record_offset":31380,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4501,"record_offset":31596,"parent_record_offset":31568,"ancestor_record_offset":31380,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4501,"record_offset":31736,"parent_record_offset":31708,"ancestor_record_offset":31380,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4501,"record_offset":31884,"parent_record_offset":31860,"ancestor_record_offset":31380,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":31380,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":31380,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":31508,"parent_record_offset":31476,"ancestor_record_offset":31380,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":42144,"parent_record_offset":42120,"ancestor_record_offset":42096,"inlinee":9919,"source_line":361,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31476,"parent_record_offset":31444,"ancestor_record_offset":31380,"inlinee":7149,"source_line":461,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31444,"parent_record_offset":31412,"ancestor_record_offset":31380,"inlinee":7148,"source_line":471,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":32344,"parent_record_offset":32312,"ancestor_record_offset":32248,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":32312,"parent_record_offset":32280,"ancestor_record_offset":32248,"inlinee":7156,"source_line":25,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31540,"parent_record_offset":31476,"ancestor_record_offset":31380,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":31568,"parent_record_offset":31540,"ancestor_record_offset":31380,"inlinee":7668,"source_line":592,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabAudioConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":6944,"rva":41044400,"va":5409753520}
// 编译实例 RVAs: 0x27249b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1427213e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409753520,"raw_hex":"e876c9ffff","size":5,"soft_fail":false,"target_va":5409739744,"va":5409753701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a7a970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3497873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409753520,"raw_hex":"e8915f3500","size":5,"soft_fail":false,"target_va":5413251440,"va":5409753562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6606238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409753520,"raw_hex":"e89ecd6400","size":5,"soft_fail":false,"target_va":5416360864,"va":5409754621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409753520,"raw_hex":"e8a2e6ffff","size":5,"soft_fail":false,"target_va":5409747600,"va":5409754089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-6611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409753520,"raw_hex":"e82de6ffff","size":5,"soft_fail":false,"target_va":5409747792,"va":5409754398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":7160,"parent_record_offset":6944,"ancestor_record_offset":6944,"inlinee":129654,"source_line":175,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":7736,"parent_record_offset":6944,"ancestor_record_offset":6944,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":7780,"parent_record_offset":6944,"ancestor_record_offset":6944,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":6944,"inlinee":98343,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":7612,"parent_record_offset":7160,"ancestor_record_offset":6944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":7520,"parent_record_offset":7492,"ancestor_record_offset":6944,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":7268,"parent_record_offset":7160,"ancestor_record_offset":6944,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":7212,"parent_record_offset":7160,"ancestor_record_offset":6944,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":7336,"parent_record_offset":7304,"ancestor_record_offset":6944,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3615,"record_offset":7236,"parent_record_offset":7212,"ancestor_record_offset":6944,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":7380,"parent_record_offset":7336,"ancestor_record_offset":6944,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":7304,"parent_record_offset":7160,"ancestor_record_offset":6944,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabRtConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":5248,"rva":41041856,"va":5409750976}
// 编译实例 RVAs: 0x2723fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14271f560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409750976,"raw_hex":"e8efb4ffff","size":5,"soft_fail":false,"target_va":5409731936,"va":5409751148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a7a970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3500419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409750976,"raw_hex":"e883693500","size":5,"soft_fail":false,"target_va":5413251440,"va":5409751016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6608630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409750976,"raw_hex":"e8f6d66400","size":5,"soft_fail":false,"target_va":5416360864,"va":5409752229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409750976,"raw_hex":"e840efffff","size":5,"soft_fail":false,"target_va":5409747600,"va":5409751883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-3838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409750976,"raw_hex":"e802f1ffff","size":5,"soft_fail":false,"target_va":5409747792,"va":5409751625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":5464,"parent_record_offset":5248,"ancestor_record_offset":5248,"inlinee":129654,"source_line":175,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":5996,"parent_record_offset":5248,"ancestor_record_offset":5248,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":6032,"parent_record_offset":5248,"ancestor_record_offset":5248,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":5248,"inlinee":98343,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":5872,"parent_record_offset":5464,"ancestor_record_offset":5248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5248,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":5552,"parent_record_offset":5464,"ancestor_record_offset":5248,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":5496,"parent_record_offset":5464,"ancestor_record_offset":5248,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":5616,"parent_record_offset":5588,"ancestor_record_offset":5248,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3615,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":5248,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":5656,"parent_record_offset":5616,"ancestor_record_offset":5248,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":5588,"parent_record_offset":5464,"ancestor_record_offset":5248,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":6064,"rva":41043264,"va":5409752384}
// 编译实例 RVAs: 0x2724540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14271ed20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409752384,"raw_hex":"e828a7ffff","size":5,"soft_fail":false,"target_va":5409729824,"va":5409752563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a7a970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3499011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409752384,"raw_hex":"e803643500","size":5,"soft_fail":false,"target_va":5413251440,"va":5409752424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6607510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409752384,"raw_hex":"e896d26400","size":5,"soft_fail":false,"target_va":5416360864,"va":5409753349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409752384,"raw_hex":"e83eebffff","size":5,"soft_fail":false,"target_va":5409747600,"va":5409752909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409752384,"raw_hex":"e803ebffff","size":5,"soft_fail":false,"target_va":5409747792,"va":5409753160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":6292,"parent_record_offset":6064,"ancestor_record_offset":6064,"inlinee":129654,"source_line":175,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":6868,"parent_record_offset":6064,"ancestor_record_offset":6064,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":6912,"parent_record_offset":6064,"ancestor_record_offset":6064,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":6064,"inlinee":98343,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":6744,"parent_record_offset":6292,"ancestor_record_offset":6064,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":6652,"parent_record_offset":6624,"ancestor_record_offset":6064,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":6400,"parent_record_offset":6292,"ancestor_record_offset":6064,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":6344,"parent_record_offset":6292,"ancestor_record_offset":6064,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":6468,"parent_record_offset":6436,"ancestor_record_offset":6064,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3615,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":6064,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":6512,"parent_record_offset":6468,"ancestor_record_offset":6064,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":6436,"parent_record_offset":6292,"ancestor_record_offset":6064,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":4316,"rva":41040576,"va":5409749696}
// 编译实例 RVAs: 0x2723ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402026b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38933570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409749696,"raw_hex":"e8beebadfd","size":5,"soft_fail":false,"target_va":5370816176,"va":5409749741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1427213e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409749696,"raw_hex":"e863d8ffff","size":5,"soft_fail":false,"target_va":5409739744,"va":5409749880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6610059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409749696,"raw_hex":"e88bdc6400","size":5,"soft_fail":false,"target_va":5416360864,"va":5409750800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409749696,"raw_hex":"e88ff5ffff","size":5,"soft_fail":false,"target_va":5409747600,"va":5409750268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-2790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409749696,"raw_hex":"e81af5ffff","size":5,"soft_fail":false,"target_va":5409747792,"va":5409750577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":4540,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":129654,"source_line":175,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":5172,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":5216,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":4316,"inlinee":98343,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":5048,"parent_record_offset":4540,"ancestor_record_offset":4316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":4316,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":4704,"parent_record_offset":4540,"ancestor_record_offset":4316,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":3615,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":4316,"inlinee":9609,"source_line":39,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":3615,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":4316,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":4592,"parent_record_offset":4540,"ancestor_record_offset":4316,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":4772,"parent_record_offset":4740,"ancestor_record_offset":4316,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3615,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":4316,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":4816,"parent_record_offset":4772,"ancestor_record_offset":4316,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":4740,"parent_record_offset":4540,"ancestor_record_offset":4316,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":3440,"rva":41039168,"va":5409748288}
// 编译实例 RVAs: 0x2723540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402026b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38932160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409748288,"raw_hex":"e840f1adfd","size":5,"soft_fail":false,"target_va":5370816176,"va":5409748331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271f560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409748288,"raw_hex":"e86cbfffff","size":5,"soft_fail":false,"target_va":5409731936,"va":5409748463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6611315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409748288,"raw_hex":"e873e16400","size":5,"soft_fail":false,"target_va":5416360864,"va":5409749544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409748288,"raw_hex":"e8bdf9ffff","size":5,"soft_fail":false,"target_va":5409747600,"va":5409749198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-1153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409748288,"raw_hex":"e87ffbffff","size":5,"soft_fail":false,"target_va":5409747792,"va":5409748940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":3660,"parent_record_offset":3440,"ancestor_record_offset":3440,"inlinee":129654,"source_line":175,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":4248,"parent_record_offset":3440,"ancestor_record_offset":3440,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":4284,"parent_record_offset":3440,"ancestor_record_offset":3440,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3440,"inlinee":98343,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":4124,"parent_record_offset":3660,"ancestor_record_offset":3440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":3440,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":3804,"parent_record_offset":3660,"ancestor_record_offset":3440,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":3615,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":3440,"inlinee":9609,"source_line":39,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":3615,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":3440,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":3692,"parent_record_offset":3660,"ancestor_record_offset":3440,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":3440,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3615,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":3440,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":3908,"parent_record_offset":3868,"ancestor_record_offset":3440,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":3840,"parent_record_offset":3660,"ancestor_record_offset":3440,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3615,"record_offset":7812,"rva":41045664,"va":5409754784}
// 编译实例 RVAs: 0x2724ea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402026b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38938656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409754784,"raw_hex":"e8e0d7adfd","size":5,"soft_fail":false,"target_va":5370816176,"va":5409754827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271ed20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409754784,"raw_hex":"e8c59dffff","size":5,"soft_fail":false,"target_va":5409729824,"va":5409754966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: inheritable.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6605107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409754784,"raw_hex":"e833c96400","size":5,"soft_fail":false,"target_va":5416360864,"va":5409755752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409754784,"raw_hex":"e8dbe1ffff","size":5,"soft_fail":false,"target_va":5409747600,"va":5409755312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-7776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409754784,"raw_hex":"e8a0e1ffff","size":5,"soft_fail":false,"target_va":5409747792,"va":5409755563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":8044,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":129654,"source_line":175,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":8676,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":8720,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":7812,"inlinee":98343,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":8552,"parent_record_offset":8044,"ancestor_record_offset":7812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":7812,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":8208,"parent_record_offset":8044,"ancestor_record_offset":7812,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":3615,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":7812,"inlinee":9609,"source_line":39,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":3615,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":7812,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":8096,"parent_record_offset":8044,"ancestor_record_offset":7812,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":8276,"parent_record_offset":8244,"ancestor_record_offset":7812,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3615,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":7812,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":8320,"parent_record_offset":8276,"ancestor_record_offset":7812,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":8244,"parent_record_offset":8044,"ancestor_record_offset":7812,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA76
// 首条位置: {"module":3615,"inlinee":129654}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":4152,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":98343,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":4124,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":4084,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":3804,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":3692,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":3840,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3660,"inlinee":98343,"source_line":107,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3615,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":3660,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":3615,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":3660,"inlinee":9609,"source_line":39,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":3615,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":3660,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_option
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3615,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":3660,"inlinee":16638,"source_line":2245,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3615,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":3660,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3615,"record_offset":3908,"parent_record_offset":3868,"ancestor_record_offset":3660,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/03615.jsonl.gz"}
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
// 完整结构及原始类型记录: inheritable.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::inheritable::Inheritable
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: inheritable.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: inheritable.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::prefab::PrefabsModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::prefab::PrefabsModifiedCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::prefab::PrefabsModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::as_concrete
// 信息 ID: 549bc296c55d9392c1ac74f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 24 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>; module=533, inline_record=150536; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>; module=533, inline_record=150580; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>; module=533, inline_record=150624; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=39956; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=40000; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=40392; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=40440; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=75008; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::inherit_shallow
// 信息 ID: 537f361df10e068d34e6a93b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=2300; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=3968; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::impl$6::inherit_from; module=3614, inline_record=388; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::impl$6::inherit_from; module=3614, inline_record=420; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::impl$6::inherit_from; module=3614, inline_record=564; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::impl$6::inherit_from; module=3614, inline_record=712; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::impl$6::inherit_from; module=3614, inline_record=864; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::impl$6::inherit_from; module=3614, inline_record=1016; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::Inheritable::into_concrete
// 信息 ID: fd74dd1a8c984a85e3164b57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable
// 信息 ID: 9cfa74f1c49859b42d07ef53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 26 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=3612, inline_record=8148; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=3612, inline_record=8496; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=14448; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=14700; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=14984; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=15288; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=15572; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=15876; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option::_::impl$0::deserialize
// 信息 ID: e2a15306042d8f437451e4e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>; module=3615, inline_record=3660; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>; module=3615, inline_record=4540; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabRtConfig>; module=3615, inline_record=5464; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >; module=3615, inline_record=6292; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabAudioConfig>; module=3615, inline_record=7160; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >; module=3615, inline_record=8044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabAudioConfig>
// 信息 ID: ec7b2068cabfd47552054ace
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::Deserializer>", "asset::prefab::config::PrefabAudioConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabRtConfig>
// 信息 ID: e701bdd38a0f18e2d0748aec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::Deserializer>", "asset::prefab::config::PrefabRtConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >
// 信息 ID: 364a438cc740cce0a34ce4db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["asset::prefab::config::MeshProcessorConfig"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::Deserializer>", "enum2$<asset::prefab::config::MeshProcessorConfig>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2a7d749 -> 0x2724540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>
// 信息 ID: 13041f3ce955124be77b05a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabAudioConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>
// 信息 ID: 4edcf158271f2ca8457dd0e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabRtConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >
// 信息 ID: 30a4f615a0360d7cd744bd9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["asset::prefab::config::MeshProcessorConfig"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::deserialize_inheritable_option -> ["ref_mut$<ron::de::tag::Deserializer>", "enum2$<asset::prefab::config::MeshProcessorConfig>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2a7d79e -> 0x2724ea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>
// 信息 ID: b017af8999272c78f62cb512
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["asset::prefab::config::AudioTriggerConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > > @ 0x2725381 -> 0x27259c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > > @ 0x27253cc -> 0x27259c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>
// 信息 ID: 5884945c113de3ae4f948012
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["asset::prefab::config::PrefabRenderConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2651ebe -> 0x27255d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2651f9f -> 0x27255d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >
// 信息 ID: d5e2464afa7ddaa787897877
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["asset::prefab::config::PrefabAudioConfig"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<asset::prefab::config::PrefabAudioConfig>"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2651edd -> 0x2725310 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2651fc5 -> 0x2725310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >
// 信息 ID: ea64f987095fe6a595809397
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["asset::prefab::config::PrefabRtConfig"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<asset::prefab::config::PrefabRtConfig>"]; 字段布局 = unknown。
// [类型关系] asset::inheritable::impl$1::inherit_from -> ["enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2651ecc -> 0x2725cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig> @ 0x2651fb3 -> 0x2725cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$2::concrete_default
// 信息 ID: 63c76eb1c9c1e4a253d9aed8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$3::inherit_from
// 信息 ID: 7bf9de3c8286fbb9723b64f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=3615, inline_record=9008; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >; module=3615, inline_record=13536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from::closure$0
// 信息 ID: fd73ac226eac6b0fd017d62c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::inheritable::impl$5::inherit_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=3892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from::closure$1
// 信息 ID: 787fadf563244ff88cfce3e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset::inheritable::impl$5::inherit_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>
// 信息 ID: 682b30be8d226ccf295e4e6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::inheritable::impl$5::inherit_from -> ["alloc::string::String", "asset::prefab::config::PrefabPartConfig"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x1bf1817 -> 0x2651da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x1bf1d68 -> 0x2651da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$6::concrete_default
// 信息 ID: 9a892b236897f8025f2cf97d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=3615, inline_record=9092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::inheritable::impl$7::clone
// 信息 ID: ffb2fcc08528f64f5b539104
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 43 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4004; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4248; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4276; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4360; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4392; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4500; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=5164; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=5296; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
