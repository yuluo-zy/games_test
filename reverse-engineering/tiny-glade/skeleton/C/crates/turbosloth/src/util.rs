// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/turbosloth/src/util.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9348af863892879bfbeedabce1f3f277478cccdbe7457411f5c9d37682665058
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: util.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 529, 599, 1643, 1659, 1660, 1777, 3339, 3342, 3343, 3348, 3349, 3350, 3394, 4477, 4478
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bincode; // namespace candidate
//   package identities: [{"name":"bincode","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.3.3"},{"name":"bincode","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.0.1"}]
// use core; // namespace candidate
//   package identities: []
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use std; // namespace candidate
//   package identities: []
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// use twox_hash; // namespace candidate
//   package identities: [{"name":"twox-hash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.6.3"},{"name":"twox-hash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.1.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: turbosloth::util::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 11, 36, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8AA
// 首条位置: {"module":3342,"inlinee":121002}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bincode::de::impl$3::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":15164,"parent_record_offset":15136,"ancestor_record_offset":15136,"inlinee":120991,"source_line":263,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_direct] bincode::de::impl$3::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":9988,"parent_record_offset":9960,"ancestor_record_offset":9960,"inlinee":29850,"source_line":402,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":3343,"record_offset":10272,"parent_record_offset":10240,"ancestor_record_offset":9960,"inlinee":81179,"source_line":255,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::deserialize_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":3343,"record_offset":10212,"parent_record_offset":10184,"ancestor_record_offset":9960,"inlinee":81176,"source_line":396,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::Deserializer::deserialize_literal_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10240,"parent_record_offset":10212,"ancestor_record_offset":9960,"inlinee":81177,"source_line":36,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_tuple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10016,"parent_record_offset":9988,"ancestor_record_offset":9960,"inlinee":29851,"source_line":293,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_tuple::impl$0::next_element_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":9960,"inlinee":29855,"source_line":307,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10184,"parent_record_offset":10156,"ancestor_record_offset":9960,"inlinee":101658,"source_line":108,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::read::impl$2::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/read.rs
//     evidence={"module":3343,"record_offset":10320,"parent_record_offset":10296,"ancestor_record_offset":9960,"inlinee":81182,"source_line":73,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::read::impl$2::read_exact
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/read.rs
//     evidence={"module":3343,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":9960,"inlinee":81181,"source_line":85,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3343,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":9960,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3343,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":9960,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3343,"record_offset":10436,"parent_record_offset":10408,"ancestor_record_offset":9960,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3343,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":9960,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3343,"record_offset":10572,"parent_record_offset":10240,"ancestor_record_offset":9960,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3343,"record_offset":10796,"parent_record_offset":10712,"ancestor_record_offset":9960,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10500,"parent_record_offset":10476,"ancestor_record_offset":9960,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10476,"parent_record_offset":10320,"ancestor_record_offset":9960,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10352,"parent_record_offset":10320,"ancestor_record_offset":9960,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10380,"parent_record_offset":10352,"ancestor_record_offset":9960,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10408,"parent_record_offset":10380,"ancestor_record_offset":9960,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10072,"parent_record_offset":10044,"ancestor_record_offset":9960,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10128,"parent_record_offset":10100,"ancestor_record_offset":9960,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3343,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":9960,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$73::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3343,"record_offset":10156,"parent_record_offset":10128,"ancestor_record_offset":9960,"inlinee":17603,"source_line":132,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize::impl$2::visit_seq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3343,"record_offset":10044,"parent_record_offset":10016,"ancestor_record_offset":9960,"inlinee":121006,"source_line":11,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 11, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8AE
// 首条位置: {"module":3343,"inlinee":121006}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3343,"record_offset":15248,"parent_record_offset":14828,"ancestor_record_offset":14828,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10072,"parent_record_offset":10044,"ancestor_record_offset":10044,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3343,"record_offset":15496,"parent_record_offset":15472,"ancestor_record_offset":14828,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3343,"record_offset":15520,"parent_record_offset":15496,"ancestor_record_offset":14828,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3343,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":14828,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3343,"record_offset":15472,"parent_record_offset":15416,"ancestor_record_offset":14828,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3343,"record_offset":15444,"parent_record_offset":15416,"ancestor_record_offset":14828,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3343,"record_offset":15416,"parent_record_offset":15392,"ancestor_record_offset":14828,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3343,"record_offset":15392,"parent_record_offset":15368,"ancestor_record_offset":14828,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":3343,"record_offset":10272,"parent_record_offset":10240,"ancestor_record_offset":10044,"inlinee":81179,"source_line":255,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::deserialize_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":3343,"record_offset":10212,"parent_record_offset":10184,"ancestor_record_offset":10044,"inlinee":81176,"source_line":396,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::Deserializer::deserialize_literal_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10240,"parent_record_offset":10212,"ancestor_record_offset":10044,"inlinee":81177,"source_line":36,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":15164,"parent_record_offset":15136,"ancestor_record_offset":14828,"inlinee":120991,"source_line":263,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":14964,"parent_record_offset":14940,"ancestor_record_offset":14828,"inlinee":83481,"source_line":240,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_tuple::impl$0::next_element_seed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":10044,"inlinee":29855,"source_line":307,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10184,"parent_record_offset":10156,"ancestor_record_offset":10044,"inlinee":101658,"source_line":108,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::read::impl$2::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/read.rs
//     evidence={"module":3343,"record_offset":10320,"parent_record_offset":10296,"ancestor_record_offset":10044,"inlinee":81182,"source_line":73,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::read::impl$2::read_exact
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/read.rs
//     evidence={"module":3343,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":10044,"inlinee":81181,"source_line":85,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3343,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":10044,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3343,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":10044,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3343,"record_offset":10436,"parent_record_offset":10408,"ancestor_record_offset":10044,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3343,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":10044,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3343,"record_offset":15272,"parent_record_offset":15248,"ancestor_record_offset":14828,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3343,"record_offset":10572,"parent_record_offset":10240,"ancestor_record_offset":10044,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3343,"record_offset":10796,"parent_record_offset":10712,"ancestor_record_offset":10044,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10500,"parent_record_offset":10476,"ancestor_record_offset":10044,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10476,"parent_record_offset":10320,"ancestor_record_offset":10044,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10352,"parent_record_offset":10320,"ancestor_record_offset":10044,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10380,"parent_record_offset":10352,"ancestor_record_offset":10044,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3343,"record_offset":10408,"parent_record_offset":10380,"ancestor_record_offset":10044,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3343,"record_offset":10128,"parent_record_offset":10100,"ancestor_record_offset":10044,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3343,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":10044,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$29::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3343,"record_offset":14940,"parent_record_offset":14916,"ancestor_record_offset":14828,"inlinee":7146,"source_line":1870,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$73::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3343,"record_offset":10156,"parent_record_offset":10128,"ancestor_record_offset":10044,"inlinee":17603,"source_line":132,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3343,"record_offset":15136,"parent_record_offset":15108,"ancestor_record_offset":14828,"inlinee":121002,"source_line":36,"archive":"_pdb/C/modules/03343.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=28
// 源码行: 11, 36, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BC7
// 首条位置: {"module":529,"inlinee":31687}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bincode::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":5536,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":31698,"source_line":580,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] bincode::ser::impl$16::serialize_field
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":599,"record_offset":80640,"parent_record_offset":80616,"ancestor_record_offset":80616,"inlinee":31710,"source_line":700,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_direct] bincode::ser::impl$1::serialize_struct_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":4912,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":31696,"source_line":188,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] bincode::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":4288,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":31689,"source_line":220,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] bincode::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":6256,"inlinee":31685,"source_line":558,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4760,"parent_record_offset":4728,"ancestor_record_offset":4260,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4580,"parent_record_offset":4552,"ancestor_record_offset":4260,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4676,"parent_record_offset":4648,"ancestor_record_offset":4260,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4648,"parent_record_offset":4612,"ancestor_record_offset":4260,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":4260,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":4260,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4612,"parent_record_offset":4580,"ancestor_record_offset":4260,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4512,"parent_record_offset":4484,"ancestor_record_offset":4260,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4728,"parent_record_offset":4512,"ancestor_record_offset":4260,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":599,"record_offset":80936,"parent_record_offset":80912,"ancestor_record_offset":80616,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":599,"record_offset":80912,"parent_record_offset":80888,"ancestor_record_offset":80616,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":4260,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4848,"parent_record_offset":4512,"ancestor_record_offset":4260,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4552,"parent_record_offset":4512,"ancestor_record_offset":4260,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":599,"record_offset":80888,"parent_record_offset":80864,"ancestor_record_offset":80616,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":529,"record_offset":4484,"parent_record_offset":4456,"ancestor_record_offset":4260,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::Wtf8Buf::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":599,"record_offset":80864,"parent_record_offset":80840,"ancestor_record_offset":80616,"inlinee":6907,"source_line":175,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::WriteBytesExt::write_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":529,"record_offset":5676,"parent_record_offset":5648,"ancestor_record_offset":4260,"inlinee":31704,"source_line":370,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::WriteBytesExt::write_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":529,"record_offset":4372,"parent_record_offset":4344,"ancestor_record_offset":4260,"inlinee":31695,"source_line":327,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":529,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":4260,"inlinee":31708,"source_line":341,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::serialize_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":529,"record_offset":5620,"parent_record_offset":5592,"ancestor_record_offset":4260,"inlinee":31702,"source_line":432,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::serialize_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":529,"record_offset":4316,"parent_record_offset":4288,"ancestor_record_offset":4260,"inlinee":31691,"source_line":362,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::serialize_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":529,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":4260,"inlinee":31706,"source_line":366,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::Serializer::serialize_literal_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":4260,"inlinee":31703,"source_line":27,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::Serializer::serialize_literal_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":4260,"inlinee":31693,"source_line":27,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::Serializer::serialize_literal_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":6396,"parent_record_offset":6368,"ancestor_record_offset":4260,"inlinee":31707,"source_line":27,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::SizeChecker::add_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":599,"record_offset":81288,"parent_record_offset":81264,"ancestor_record_offset":80616,"inlinee":31714,"source_line":252,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::SizeChecker::add_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":599,"record_offset":81312,"parent_record_offset":81288,"ancestor_record_offset":80616,"inlinee":31715,"source_line":240,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$1::serialize_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":5592,"parent_record_offset":5564,"ancestor_record_offset":4260,"inlinee":31701,"source_line":59,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$1::serialize_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":6340,"parent_record_offset":6312,"ancestor_record_offset":4260,"inlinee":31705,"source_line":59,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":599,"record_offset":81264,"parent_record_offset":81240,"ancestor_record_offset":80616,"inlinee":31712,"source_line":316,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":4260,"inlinee":31685,"source_line":558,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":599,"record_offset":81200,"parent_record_offset":81168,"ancestor_record_offset":80616,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$0::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":599,"record_offset":81240,"parent_record_offset":81088,"ancestor_record_offset":80616,"inlinee":8824,"source_line":42,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$31::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":599,"record_offset":81088,"parent_record_offset":80668,"ancestor_record_offset":80616,"inlinee":11102,"source_line":909,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$32::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":599,"record_offset":80668,"parent_record_offset":80640,"ancestor_record_offset":80616,"inlinee":11148,"source_line":923,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$49::serialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":529,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":4260,"inlinee":10591,"source_line":11,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$50::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":529,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":4260,"inlinee":31700,"source_line":11,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$5::serialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":529,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":4260,"inlinee":8752,"source_line":104,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":599,"record_offset":81144,"parent_record_offset":81120,"ancestor_record_offset":80616,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":599,"record_offset":80768,"parent_record_offset":80744,"ancestor_record_offset":80616,"inlinee":11100,"source_line":1710,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":599,"record_offset":80816,"parent_record_offset":80792,"ancestor_record_offset":80616,"inlinee":6903,"source_line":656,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":599,"record_offset":80792,"parent_record_offset":80768,"ancestor_record_offset":80616,"inlinee":6901,"source_line":674,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":529,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":4260,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":529,"record_offset":4400,"parent_record_offset":4372,"ancestor_record_offset":4260,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":599,"record_offset":80744,"parent_record_offset":80720,"ancestor_record_offset":80616,"inlinee":11098,"source_line":2383,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":599,"record_offset":81120,"parent_record_offset":81088,"ancestor_record_offset":80616,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::path::PathBuf::as_path
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":599,"record_offset":80696,"parent_record_offset":80668,"ancestor_record_offset":80616,"inlinee":11095,"source_line":1264,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":599,"record_offset":80720,"parent_record_offset":80696,"ancestor_record_offset":80616,"inlinee":11097,"source_line":2066,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Buf::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":599,"record_offset":80840,"parent_record_offset":80816,"ancestor_record_offset":80616,"inlinee":6905,"source_line":157,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":599,"record_offset":81168,"parent_record_offset":81144,"ancestor_record_offset":80616,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":529,"record_offset":6256,"parent_record_offset":6228,"ancestor_record_offset":4260,"inlinee":31687,"source_line":68,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::_::impl$0::serialize<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >
// 记录形式: compiled_procedure; instances=3
// 源码行: 11, 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":3736,"rva":10527712,"va":5379236832}
// 编译实例 RVAs: 0xa0a3e0, 0x2642720, 0x2642860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417bac00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14354436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379236832,"raw_hex":"e80408db00","size":5,"soft_fail":false,"target_va":5393591296,"va":5379236855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37115870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379236832,"raw_hex":"e8de573602","size":5,"soft_fail":false,"target_va":5416353040,"va":5379237165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::ser::impls::_$LT$impl$u20$serde_core..ser..Serialize$u20$for$u20$std..path..PathBuf$GT$::serialize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4344724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408827488,"raw_hex":"e8944b4200","size":5,"soft_fail":false,"target_va":5413172240,"va":5408827511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1417b2ae0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t14321099","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379236832,"raw_hex":"e9cb85da00","size":5,"soft_fail":false,"target_va":5393558240,"va":5379237136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1417b2b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: util.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t14321200","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379236832,"raw_hex":"e93086da00","size":5,"soft_fail":false,"target_va":5393558320,"va":5379237115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bincode::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":4204,"parent_record_offset":3736,"ancestor_record_offset":3736,"inlinee":31685,"source_line":558,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4760,"parent_record_offset":4728,"ancestor_record_offset":3736,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4580,"parent_record_offset":4552,"ancestor_record_offset":3736,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4676,"parent_record_offset":4648,"ancestor_record_offset":3736,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4648,"parent_record_offset":4612,"ancestor_record_offset":3736,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":3736,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":3736,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":4612,"parent_record_offset":4580,"ancestor_record_offset":3736,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4512,"parent_record_offset":4484,"ancestor_record_offset":3736,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4728,"parent_record_offset":4512,"ancestor_record_offset":3736,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":3736,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4848,"parent_record_offset":4512,"ancestor_record_offset":3736,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":4552,"parent_record_offset":4512,"ancestor_record_offset":3736,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":529,"record_offset":4484,"parent_record_offset":4456,"ancestor_record_offset":3736,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::WriteBytesExt::write_u128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":529,"record_offset":5676,"parent_record_offset":5648,"ancestor_record_offset":3736,"inlinee":31704,"source_line":370,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::WriteBytesExt::write_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":529,"record_offset":4372,"parent_record_offset":4344,"ancestor_record_offset":3736,"inlinee":31695,"source_line":327,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":529,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":3736,"inlinee":31708,"source_line":341,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::WriteBytesExt::write_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":3348,"record_offset":3196,"parent_record_offset":3160,"ancestor_record_offset":1988,"inlinee":52473,"source_line":303,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::serialize_u128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":529,"record_offset":5620,"parent_record_offset":5592,"ancestor_record_offset":3736,"inlinee":31702,"source_line":432,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::serialize_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":529,"record_offset":4316,"parent_record_offset":4288,"ancestor_record_offset":3736,"inlinee":31691,"source_line":362,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::serialize_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":529,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":3736,"inlinee":31706,"source_line":366,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::Serializer::serialize_literal_u128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":3736,"inlinee":31703,"source_line":27,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::Serializer::serialize_literal_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":3736,"inlinee":31693,"source_line":27,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::Serializer::serialize_literal_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":6396,"parent_record_offset":6368,"ancestor_record_offset":3736,"inlinee":31707,"source_line":27,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":5536,"parent_record_offset":4260,"ancestor_record_offset":3736,"inlinee":31698,"source_line":580,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$1::serialize_none
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":3348,"record_offset":4324,"parent_record_offset":3120,"ancestor_record_offset":1988,"inlinee":52477,"source_line":137,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$1::serialize_some
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":3348,"record_offset":3160,"parent_record_offset":3120,"ancestor_record_offset":1988,"inlinee":52476,"source_line":141,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$1::serialize_struct_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":4912,"parent_record_offset":4260,"ancestor_record_offset":3736,"inlinee":31696,"source_line":188,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$1::serialize_u128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":5592,"parent_record_offset":5564,"ancestor_record_offset":3736,"inlinee":31701,"source_line":59,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$1::serialize_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":6340,"parent_record_offset":6312,"ancestor_record_offset":3736,"inlinee":31705,"source_line":59,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":4288,"parent_record_offset":4260,"ancestor_record_offset":3736,"inlinee":31689,"source_line":220,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bincode::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/ser/mod.rs
//     evidence={"module":529,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":3736,"inlinee":31685,"source_line":558,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$49::serialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":529,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":3736,"inlinee":10591,"source_line":11,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$50::serialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":529,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":3736,"inlinee":31700,"source_line":11,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$5::serialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":529,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":3736,"inlinee":8752,"source_line":104,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":529,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":3736,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":529,"record_offset":4400,"parent_record_offset":4372,"ancestor_record_offset":3736,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":529,"record_offset":4260,"parent_record_offset":4232,"ancestor_record_offset":3736,"inlinee":31687,"source_line":68,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$10::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C35
// 首条位置: {"module":529,"inlinee":31797}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":529,"record_offset":75928,"parent_record_offset":75896,"ancestor_record_offset":75896,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":529,"record_offset":76284,"parent_record_offset":75896,"ancestor_record_offset":75896,"inlinee":31799,"source_line":36,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$26::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":76340,"parent_record_offset":76316,"ancestor_record_offset":75896,"inlinee":31801,"source_line":1878,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":76100,"parent_record_offset":75952,"ancestor_record_offset":75896,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":529,"record_offset":76216,"parent_record_offset":76184,"ancestor_record_offset":75896,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":76184,"parent_record_offset":75952,"ancestor_record_offset":75896,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":529,"record_offset":76040,"parent_record_offset":76016,"ancestor_record_offset":75896,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":529,"record_offset":76424,"parent_record_offset":76396,"ancestor_record_offset":75896,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":529,"record_offset":76124,"parent_record_offset":76100,"ancestor_record_offset":75896,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":529,"record_offset":76148,"parent_record_offset":76124,"ancestor_record_offset":75896,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":529,"record_offset":76016,"parent_record_offset":75952,"ancestor_record_offset":75896,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":529,"record_offset":75992,"parent_record_offset":75952,"ancestor_record_offset":75896,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":529,"record_offset":75952,"parent_record_offset":75928,"ancestor_record_offset":75896,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":529,"record_offset":76396,"parent_record_offset":76372,"ancestor_record_offset":75896,"inlinee":31803,"source_line":68,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D6F
// 首条位置: {"module":1643,"inlinee":81263}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1643,"record_offset":260204,"parent_record_offset":260164,"ancestor_record_offset":260164,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::impl$11::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1643,"record_offset":260892,"parent_record_offset":260164,"ancestor_record_offset":260164,"inlinee":81265,"source_line":36,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":260704,"parent_record_offset":260680,"ancestor_record_offset":260164,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":260728,"parent_record_offset":260704,"ancestor_record_offset":260164,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":260752,"parent_record_offset":260728,"ancestor_record_offset":260164,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":260680,"parent_record_offset":260532,"ancestor_record_offset":260164,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":260472,"parent_record_offset":260444,"ancestor_record_offset":260164,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":260532,"parent_record_offset":260500,"ancestor_record_offset":260164,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":260500,"parent_record_offset":260472,"ancestor_record_offset":260164,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":260568,"parent_record_offset":260532,"ancestor_record_offset":260164,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1643,"record_offset":260408,"parent_record_offset":260344,"ancestor_record_offset":260164,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":260380,"parent_record_offset":260344,"ancestor_record_offset":260164,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":260444,"parent_record_offset":260408,"ancestor_record_offset":260164,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":260316,"parent_record_offset":260288,"ancestor_record_offset":260164,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":260344,"parent_record_offset":260316,"ancestor_record_offset":260164,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1643,"record_offset":260288,"parent_record_offset":260260,"ancestor_record_offset":260164,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1643,"record_offset":260592,"parent_record_offset":260568,"ancestor_record_offset":260164,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1643,"record_offset":260640,"parent_record_offset":260616,"ancestor_record_offset":260164,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":260616,"parent_record_offset":260592,"ancestor_record_offset":260164,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1643,"record_offset":260808,"parent_record_offset":260408,"ancestor_record_offset":260164,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":260832,"parent_record_offset":260808,"ancestor_record_offset":260164,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1643,"record_offset":260232,"parent_record_offset":260204,"ancestor_record_offset":260164,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1643,"record_offset":260260,"parent_record_offset":260232,"ancestor_record_offset":260164,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$8::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1411A
// 首条位置: {"module":1660,"inlinee":82202}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$65::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1660,"record_offset":54648,"parent_record_offset":54620,"ancestor_record_offset":54620,"inlinee":20210,"source_line":2257,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::impl$14::hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1660,"record_offset":55068,"parent_record_offset":54620,"ancestor_record_offset":54620,"inlinee":82204,"source_line":36,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1660,"record_offset":54936,"parent_record_offset":54912,"ancestor_record_offset":54620,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1660,"record_offset":54984,"parent_record_offset":54960,"ancestor_record_offset":54620,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1660,"record_offset":54960,"parent_record_offset":54936,"ancestor_record_offset":54620,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1660,"record_offset":54912,"parent_record_offset":54888,"ancestor_record_offset":54620,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1660,"record_offset":54888,"parent_record_offset":54864,"ancestor_record_offset":54620,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1660,"record_offset":54864,"parent_record_offset":54840,"ancestor_record_offset":54620,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::Wtf8Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1660,"record_offset":54840,"parent_record_offset":54816,"ancestor_record_offset":54620,"inlinee":6907,"source_line":175,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55120,"parent_record_offset":55096,"ancestor_record_offset":54620,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55288,"parent_record_offset":55264,"ancestor_record_offset":54620,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55456,"parent_record_offset":55432,"ancestor_record_offset":54620,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55144,"parent_record_offset":55120,"ancestor_record_offset":54620,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55432,"parent_record_offset":55404,"ancestor_record_offset":54620,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55096,"parent_record_offset":55068,"ancestor_record_offset":54620,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$19::hash
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55264,"parent_record_offset":55068,"ancestor_record_offset":54620,"inlinee":77768,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":55220,"parent_record_offset":55196,"ancestor_record_offset":54620,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55312,"parent_record_offset":55288,"ancestor_record_offset":54620,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55168,"parent_record_offset":55144,"ancestor_record_offset":54620,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55480,"parent_record_offset":55456,"ancestor_record_offset":54620,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$56::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1660,"record_offset":55572,"parent_record_offset":55404,"ancestor_record_offset":54620,"inlinee":14031,"source_line":594,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1660,"record_offset":54744,"parent_record_offset":54720,"ancestor_record_offset":54620,"inlinee":11100,"source_line":1710,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1660,"record_offset":54792,"parent_record_offset":54768,"ancestor_record_offset":54620,"inlinee":6903,"source_line":656,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1660,"record_offset":54768,"parent_record_offset":54744,"ancestor_record_offset":54620,"inlinee":6901,"source_line":674,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1660,"record_offset":55196,"parent_record_offset":55144,"ancestor_record_offset":54620,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1660,"record_offset":54720,"parent_record_offset":54696,"ancestor_record_offset":54620,"inlinee":11098,"source_line":2383,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::path::PathBuf::as_path
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1660,"record_offset":54672,"parent_record_offset":54648,"ancestor_record_offset":54620,"inlinee":11095,"source_line":1264,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1660,"record_offset":54696,"parent_record_offset":54672,"ancestor_record_offset":54620,"inlinee":11097,"source_line":2066,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1660,"record_offset":54816,"parent_record_offset":54792,"ancestor_record_offset":54620,"inlinee":6905,"source_line":157,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::impl$22::hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1660,"record_offset":55404,"parent_record_offset":55068,"ancestor_record_offset":54620,"inlinee":82206,"source_line":68,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::FileDependency::present
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D87E
// 首条位置: {"module":3339,"inlinee":120958}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3339,"record_offset":16024,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":3339,"record_offset":16072,"parent_record_offset":16048,"ancestor_record_offset":15996,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3339,"record_offset":16048,"parent_record_offset":16024,"ancestor_record_offset":15996,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":3339,"record_offset":16096,"parent_record_offset":16072,"ancestor_record_offset":15996,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::FileDependency::missing
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D87D
// 首条位置: {"module":3339,"inlinee":120957}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3339,"record_offset":15852,"parent_record_offset":15824,"ancestor_record_offset":15824,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":3339,"record_offset":15900,"parent_record_offset":15876,"ancestor_record_offset":15824,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3339,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":15824,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":3339,"record_offset":15924,"parent_record_offset":15900,"ancestor_record_offset":15824,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8B1
// 首条位置: {"module":3343,"inlinee":121009}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::Error::invalid_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9236,"parent_record_offset":9208,"ancestor_record_offset":9208,"inlinee":47812,"source_line":231,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8B0
// 首条位置: {"module":3343,"inlinee":121008}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::value::impl$12::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":3348,"record_offset":9124,"parent_record_offset":9096,"ancestor_record_offset":9096,"inlinee":120998,"source_line":125,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::invalid_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9236,"parent_record_offset":9208,"ancestor_record_offset":9096,"inlinee":47812,"source_line":231,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9180,"parent_record_offset":9152,"ancestor_record_offset":9096,"inlinee":11196,"source_line":1446,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$12::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/value.rs
//     evidence={"module":3348,"record_offset":9152,"parent_record_offset":9124,"ancestor_record_offset":9096,"inlinee":120999,"source_line":385,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3348,"record_offset":9208,"parent_record_offset":9180,"ancestor_record_offset":9096,"inlinee":121009,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8AD
// 首条位置: {"module":3343,"inlinee":121005}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::ReadBytesExt::read_u128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":3348,"record_offset":9544,"parent_record_offset":9520,"ancestor_record_offset":9352,"inlinee":83479,"source_line":284,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":3348,"record_offset":10212,"parent_record_offset":10188,"ancestor_record_offset":9352,"inlinee":81179,"source_line":255,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::deserialize_u128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":3348,"record_offset":9496,"parent_record_offset":9472,"ancestor_record_offset":9352,"inlinee":83477,"source_line":446,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::deserialize_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":3348,"record_offset":10164,"parent_record_offset":10140,"ancestor_record_offset":9352,"inlinee":81176,"source_line":396,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::Deserializer::deserialize_literal_u128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":9352,"inlinee":83478,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::Deserializer::deserialize_literal_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":10188,"parent_record_offset":10164,"ancestor_record_offset":9352,"inlinee":81177,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9964,"parent_record_offset":9940,"ancestor_record_offset":9352,"inlinee":29850,"source_line":402,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_tuple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9992,"parent_record_offset":9964,"ancestor_record_offset":9352,"inlinee":29851,"source_line":293,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_tuple::impl$0::next_element_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9400,"parent_record_offset":9376,"ancestor_record_offset":9352,"inlinee":29855,"source_line":307,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_u128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9472,"parent_record_offset":9448,"ancestor_record_offset":9352,"inlinee":83476,"source_line":108,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":10140,"parent_record_offset":10116,"ancestor_record_offset":9352,"inlinee":101658,"source_line":108,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::read::impl$2::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/read.rs
//     evidence={"module":3348,"record_offset":9592,"parent_record_offset":9568,"ancestor_record_offset":9352,"inlinee":81182,"source_line":73,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::read::impl$2::read_exact
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/read.rs
//     evidence={"module":3348,"record_offset":9568,"parent_record_offset":9544,"ancestor_record_offset":9352,"inlinee":81181,"source_line":85,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3348,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":9352,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3348,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":9352,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3348,"record_offset":10652,"parent_record_offset":10568,"ancestor_record_offset":9352,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":9756,"parent_record_offset":9732,"ancestor_record_offset":9352,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":9732,"parent_record_offset":9592,"ancestor_record_offset":9352,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":9620,"parent_record_offset":9592,"ancestor_record_offset":9352,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":9352,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":9668,"parent_record_offset":9644,"ancestor_record_offset":9352,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":9352,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9424,"parent_record_offset":9400,"ancestor_record_offset":9352,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3348,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":9352,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$73::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3348,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":9352,"inlinee":17603,"source_line":132,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$84::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3348,"record_offset":9448,"parent_record_offset":9424,"ancestor_record_offset":9352,"inlinee":83475,"source_line":132,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3348,"record_offset":9940,"parent_record_offset":9912,"ancestor_record_offset":9352,"inlinee":121002,"source_line":68,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize::impl$2::visit_seq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3348,"record_offset":10020,"parent_record_offset":9992,"ancestor_record_offset":9352,"inlinee":121006,"source_line":68,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum<ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3348,"record_offset":8172,"rva":40118832,"va":5408827952}
// 编译实例 RVAs: 0x2642a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14263fcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408827952,"raw_hex":"e8f0d1ffff","size":5,"soft_fail":false,"target_va":5408816304,"va":5408828091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bincode::error::_$LT$impl$u20$core..convert..From$LT$std..io..error..Error$GT$$u20$for$u20$alloc..boxed..Box$LT$bincode..error..ErrorKind$GT$$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"bincode","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10299103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408827952,"raw_hex":"e821d962ff","size":5,"soft_fail":false,"target_va":5398529040,"va":5408828138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bincode::error::_$LT$impl$u20$serde_core..de..Error$u20$for$u20$alloc..boxed..Box$LT$bincode..error..ErrorKind$GT$$GT$::custom::h743dc947208a8295 (.llvm.7797686884345738163)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"bincode","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408827952,"raw_hex":"e857c1ffff","size":5,"soft_fail":false,"target_va":5408812272,"va":5408828308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bincode::de::impl$4::struct_variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9296,"parent_record_offset":8172,"ancestor_record_offset":8172,"inlinee":121011,"source_line":486,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":8644,"parent_record_offset":8172,"ancestor_record_offset":8172,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::ReadBytesExt::read_u128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":3348,"record_offset":9544,"parent_record_offset":9520,"ancestor_record_offset":8172,"inlinee":83479,"source_line":284,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":3348,"record_offset":8760,"parent_record_offset":8736,"ancestor_record_offset":8172,"inlinee":101687,"source_line":241,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::byteorder::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/byteorder.rs
//     evidence={"module":3348,"record_offset":10212,"parent_record_offset":10188,"ancestor_record_offset":8172,"inlinee":81179,"source_line":255,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::deserialize_u128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":3348,"record_offset":9496,"parent_record_offset":9472,"ancestor_record_offset":8172,"inlinee":83477,"source_line":446,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":3348,"record_offset":8712,"parent_record_offset":8672,"ancestor_record_offset":8172,"inlinee":120990,"source_line":390,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::int::impl$1::deserialize_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/int.rs
//     evidence={"module":3348,"record_offset":10164,"parent_record_offset":10140,"ancestor_record_offset":8172,"inlinee":81176,"source_line":396,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::Deserializer::deserialize_literal_u128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":8172,"inlinee":83478,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::Deserializer::deserialize_literal_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":8736,"parent_record_offset":8712,"ancestor_record_offset":8172,"inlinee":101686,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::Deserializer::deserialize_literal_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":10188,"parent_record_offset":10164,"ancestor_record_offset":8172,"inlinee":81177,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_enum::impl$0::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":8672,"parent_record_offset":8644,"ancestor_record_offset":8172,"inlinee":120994,"source_line":280,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9964,"parent_record_offset":9940,"ancestor_record_offset":8172,"inlinee":29850,"source_line":402,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_tuple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9324,"parent_record_offset":9296,"ancestor_record_offset":8172,"inlinee":29851,"source_line":293,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_tuple::impl$0::next_element_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9400,"parent_record_offset":9376,"ancestor_record_offset":8172,"inlinee":29855,"source_line":307,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_u128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9472,"parent_record_offset":9448,"ancestor_record_offset":8172,"inlinee":83476,"source_line":108,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::impl$3::deserialize_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":3348,"record_offset":10140,"parent_record_offset":10116,"ancestor_record_offset":8172,"inlinee":101658,"source_line":108,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::read::impl$2::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/read.rs
//     evidence={"module":3348,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":8172,"inlinee":81182,"source_line":73,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::read::impl$2::read_exact
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/read.rs
//     evidence={"module":3348,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":8172,"inlinee":81181,"source_line":85,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3348,"record_offset":8912,"parent_record_offset":8888,"ancestor_record_offset":8172,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3348,"record_offset":9008,"parent_record_offset":8980,"ancestor_record_offset":8172,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3348,"record_offset":10652,"parent_record_offset":10568,"ancestor_record_offset":8172,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":8980,"parent_record_offset":8952,"ancestor_record_offset":8172,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":8952,"parent_record_offset":8808,"ancestor_record_offset":8172,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":8840,"parent_record_offset":8808,"ancestor_record_offset":8172,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":8864,"parent_record_offset":8840,"ancestor_record_offset":8172,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3348,"record_offset":8888,"parent_record_offset":8864,"ancestor_record_offset":8172,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::invalid_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9236,"parent_record_offset":9208,"ancestor_record_offset":8172,"inlinee":47812,"source_line":231,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":8172,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9180,"parent_record_offset":9152,"ancestor_record_offset":8172,"inlinee":11196,"source_line":1446,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3348,"record_offset":9068,"parent_record_offset":8672,"ancestor_record_offset":8172,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3348,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":8172,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$73::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3348,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":8172,"inlinee":17603,"source_line":132,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$84::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3348,"record_offset":9448,"parent_record_offset":9424,"ancestor_record_offset":8172,"inlinee":83475,"source_line":132,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$12::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/value.rs
//     evidence={"module":3348,"record_offset":9152,"parent_record_offset":9124,"ancestor_record_offset":8172,"inlinee":120999,"source_line":385,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$12::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":3348,"record_offset":9124,"parent_record_offset":9096,"ancestor_record_offset":8172,"inlinee":120998,"source_line":125,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3348,"record_offset":9940,"parent_record_offset":9912,"ancestor_record_offset":8172,"inlinee":121002,"source_line":68,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3348,"record_offset":9208,"parent_record_offset":9180,"ancestor_record_offset":8172,"inlinee":121009,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3348,"record_offset":9096,"parent_record_offset":9068,"ancestor_record_offset":8172,"inlinee":121008,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3348,"record_offset":9352,"parent_record_offset":9324,"ancestor_record_offset":8172,"inlinee":121005,"source_line":36,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::_::impl$0::deserialize::impl$2::visit_seq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3348,"record_offset":10020,"parent_record_offset":9992,"ancestor_record_offset":8172,"inlinee":121006,"source_line":68,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$11::clone
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D71
// 首条位置: {"module":1643,"inlinee":81265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$14::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1411C
// 首条位置: {"module":1660,"inlinee":82204}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55096,"parent_record_offset":55068,"ancestor_record_offset":55068,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_direct] core::hash::impls::impl$19::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55264,"parent_record_offset":55068,"ancestor_record_offset":55068,"inlinee":77768,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::impl$22::hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1660,"record_offset":55404,"parent_record_offset":55068,"ancestor_record_offset":55068,"inlinee":82206,"source_line":68,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55120,"parent_record_offset":55096,"ancestor_record_offset":55068,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55288,"parent_record_offset":55264,"ancestor_record_offset":55068,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55456,"parent_record_offset":55432,"ancestor_record_offset":55068,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55144,"parent_record_offset":55120,"ancestor_record_offset":55068,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55432,"parent_record_offset":55404,"ancestor_record_offset":55068,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55600,"parent_record_offset":55572,"ancestor_record_offset":55068,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$19::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55768,"parent_record_offset":55572,"ancestor_record_offset":55068,"inlinee":77768,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":55220,"parent_record_offset":55196,"ancestor_record_offset":55068,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55312,"parent_record_offset":55288,"ancestor_record_offset":55068,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55168,"parent_record_offset":55144,"ancestor_record_offset":55068,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55480,"parent_record_offset":55456,"ancestor_record_offset":55068,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$56::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1660,"record_offset":55572,"parent_record_offset":55404,"ancestor_record_offset":55068,"inlinee":14031,"source_line":594,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1660,"record_offset":55196,"parent_record_offset":55144,"ancestor_record_offset":55068,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$16::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C37
// 首条位置: {"module":529,"inlinee":31799}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":76316,"parent_record_offset":76284,"ancestor_record_offset":76284,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$26::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":76340,"parent_record_offset":76316,"ancestor_record_offset":76284,"inlinee":31801,"source_line":1878,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":529,"record_offset":76424,"parent_record_offset":76396,"ancestor_record_offset":76284,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":529,"record_offset":76396,"parent_record_offset":76372,"ancestor_record_offset":76284,"inlinee":31803,"source_line":68,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::FileDependencyState::content_matches
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x150C9
// 首条位置: {"module":1777,"inlinee":86217}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1777,"record_offset":108696,"parent_record_offset":108664,"ancestor_record_offset":108664,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$26::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1777,"record_offset":108720,"parent_record_offset":108696,"ancestor_record_offset":108664,"inlinee":31801,"source_line":1878,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$22::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1411E
// 首条位置: {"module":1660,"inlinee":82206}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55432,"parent_record_offset":55404,"ancestor_record_offset":55404,"inlinee":10560,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$56::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1660,"record_offset":55572,"parent_record_offset":55404,"ancestor_record_offset":55404,"inlinee":14031,"source_line":594,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55624,"parent_record_offset":55600,"ancestor_record_offset":55404,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55792,"parent_record_offset":55768,"ancestor_record_offset":55404,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55456,"parent_record_offset":55432,"ancestor_record_offset":55404,"inlinee":6720,"source_line":380,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55648,"parent_record_offset":55624,"ancestor_record_offset":55404,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55600,"parent_record_offset":55572,"ancestor_record_offset":55404,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$19::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":55768,"parent_record_offset":55572,"ancestor_record_offset":55404,"inlinee":77768,"source_line":812,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":55532,"parent_record_offset":55508,"ancestor_record_offset":55404,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55816,"parent_record_offset":55792,"ancestor_record_offset":55404,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55672,"parent_record_offset":55648,"ancestor_record_offset":55404,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1660,"record_offset":55480,"parent_record_offset":55456,"ancestor_record_offset":55404,"inlinee":6727,"source_line":3925,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1660,"record_offset":55508,"parent_record_offset":55456,"ancestor_record_offset":55404,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$24::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C3B
// 首条位置: {"module":529,"inlinee":31803}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":529,"record_offset":76424,"parent_record_offset":76396,"ancestor_record_offset":76396,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::FileStamp::from_metadata
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x150C6
// 首条位置: {"module":1777,"inlinee":86214}
// 编译实例 RVAs: 0x263de50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::SystemTime>::duration_since
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24147457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408808528,"raw_hex":"e8ff898ffe","size":5,"soft_fail":false,"target_va":5384661120,"va":5408808572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":105796,"parent_record_offset":105772,"ancestor_record_offset":105772,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1777,"record_offset":105820,"parent_record_offset":105796,"ancestor_record_offset":105772,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1777,"record_offset":105872,"parent_record_offset":105844,"ancestor_record_offset":105772,"inlinee":52443,"source_line":631,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::system_time_ns
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1777,"record_offset":105844,"parent_record_offset":105820,"ancestor_record_offset":105772,"inlinee":86215,"source_line":168,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::normalize_file_path<ref$<std::path::Path> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3348,"record_offset":264,"rva":40117664,"va":5408826784}
// 编译实例 RVAs: 0x26425a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24170685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408826784,"raw_hex":"e8432f8ffe","size":5,"soft_fail":false,"target_va":5384656240,"va":5408826920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_absolute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24173439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408826784,"raw_hex":"e881248ffe","size":5,"soft_fail":false,"target_va":5384653392,"va":5408826826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24173122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408826784,"raw_hex":"e8be258ffe","size":5,"soft_fail":false,"target_va":5384653728,"va":5408826845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..path..Path$u20$as$u20$normalize_path..NormalizePath$GT$::normalize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11511787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408826784,"raw_hex":"e8155850ff","size":5,"soft_fail":false,"target_va":5397315184,"va":5408826966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-33148246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408826784,"raw_hex":"e8aa3206fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5408826945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24115485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408826784,"raw_hex":"e8e30690fe","size":5,"soft_fail":false,"target_va":5384711376,"va":5408826856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3348,"record_offset":512,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3348,"record_offset":424,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3348,"record_offset":456,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3348,"record_offset":484,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3348,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3348,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3348,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3348,"record_offset":736,"parent_record_offset":680,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3348,"record_offset":708,"parent_record_offset":680,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3348,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3348,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3348,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03348.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::normalize_file_path<ref$<std::path::PathBuf> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":1980,"rva":10527328,"va":5379236448}
// 编译实例 RVAs: 0xa0a260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5419649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379236448,"raw_hex":"e881b25200","size":5,"soft_fail":false,"target_va":5384656240,"va":5379236586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_absolute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5416895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379236448,"raw_hex":"e8bfa75200","size":5,"soft_fail":false,"target_va":5384653392,"va":5379236492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5417212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379236448,"raw_hex":"e8fca85200","size":5,"soft_fail":false,"target_va":5384653728,"va":5379236511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..path..Path$u20$as$u20$normalize_path..NormalizePath$GT$::normalize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t18078547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379236448,"raw_hex":"e853db1301","size":5,"soft_fail":false,"target_va":5397315184,"va":5379236632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3557912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379236448,"raw_hex":"e8e8b5c9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379236611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5474849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379236448,"raw_hex":"e8218a5300","size":5,"soft_fail":false,"target_va":5384711376,"va":5379236522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":529,"record_offset":2144,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":2260,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":529,"record_offset":2172,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":529,"record_offset":2204,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":529,"record_offset":2232,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":1980,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":1980,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":1980,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":2484,"parent_record_offset":2428,"ancestor_record_offset":1980,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":2456,"parent_record_offset":2428,"ancestor_record_offset":1980,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":1980,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":1980,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":1980,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::impl$25::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x150A3
// 首条位置: {"module":1777,"inlinee":86179}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":43204,"parent_record_offset":43160,"ancestor_record_offset":43160,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":44084,"parent_record_offset":43160,"ancestor_record_offset":43160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1777,"record_offset":43332,"parent_record_offset":43160,"ancestor_record_offset":43160,"inlinee":81263,"source_line":11,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":43860,"parent_record_offset":43836,"ancestor_record_offset":43160,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":43884,"parent_record_offset":43860,"ancestor_record_offset":43160,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":43908,"parent_record_offset":43884,"ancestor_record_offset":43160,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":43836,"parent_record_offset":43684,"ancestor_record_offset":43160,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":43628,"parent_record_offset":43604,"ancestor_record_offset":43160,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":43684,"parent_record_offset":43652,"ancestor_record_offset":43160,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":43652,"parent_record_offset":43628,"ancestor_record_offset":43160,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":43724,"parent_record_offset":43684,"ancestor_record_offset":43160,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1777,"record_offset":43576,"parent_record_offset":43516,"ancestor_record_offset":43160,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1777,"record_offset":43240,"parent_record_offset":43204,"ancestor_record_offset":43160,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1777,"record_offset":44132,"parent_record_offset":44108,"ancestor_record_offset":43160,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1777,"record_offset":43548,"parent_record_offset":43516,"ancestor_record_offset":43160,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1777,"record_offset":43604,"parent_record_offset":43576,"ancestor_record_offset":43160,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1777,"record_offset":43492,"parent_record_offset":43464,"ancestor_record_offset":43160,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1777,"record_offset":43516,"parent_record_offset":43492,"ancestor_record_offset":43160,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1777,"record_offset":43464,"parent_record_offset":43432,"ancestor_record_offset":43160,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1777,"record_offset":43748,"parent_record_offset":43724,"ancestor_record_offset":43160,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1777,"record_offset":43796,"parent_record_offset":43772,"ancestor_record_offset":43160,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1777,"record_offset":43772,"parent_record_offset":43748,"ancestor_record_offset":43160,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1777,"record_offset":43964,"parent_record_offset":43576,"ancestor_record_offset":43160,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":43988,"parent_record_offset":43964,"ancestor_record_offset":43160,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":44108,"parent_record_offset":44084,"ancestor_record_offset":43160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":43268,"parent_record_offset":43240,"ancestor_record_offset":43160,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":44168,"parent_record_offset":44132,"ancestor_record_offset":43160,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":43292,"parent_record_offset":43268,"ancestor_record_offset":43160,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":44192,"parent_record_offset":44168,"ancestor_record_offset":43160,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":44224,"parent_record_offset":44132,"ancestor_record_offset":43160,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1777,"record_offset":43400,"parent_record_offset":43368,"ancestor_record_offset":43160,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1777,"record_offset":43368,"parent_record_offset":43332,"ancestor_record_offset":43160,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1777,"record_offset":43432,"parent_record_offset":43400,"ancestor_record_offset":43160,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::impl$11::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1777,"record_offset":44048,"parent_record_offset":43332,"ancestor_record_offset":43160,"inlinee":81265,"source_line":36,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::SourceFileSnapshot::read
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3339,"record_offset":14396,"rva":40098256,"va":5408807376}
// 编译实例 RVAs: 0x263d9d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-24014443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e8959191fe","size":5,"soft_fail":false,"target_va":5384793264,"va":5408807702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::SystemTime>::duration_since
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24146522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e8a68d8ffe","size":5,"soft_fail":false,"target_va":5384661120,"va":5408807637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-33128852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e86c7e06fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5408807551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_realloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-33129154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e83e7d06fe","size":5,"soft_fail":false,"target_va":5375678720,"va":5408807869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39468948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e86cc0a5fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5408807791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14263c7d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e83fedffff","size":5,"soft_fail":false,"target_va":5408802768,"va":5408807564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7812899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e823377700","size":5,"soft_fail":false,"target_va":5416620794,"va":5408807890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-32859222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e8aa9b0afe","size":5,"soft_fail":false,"target_va":5375949040,"va":5408808257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::from_box_in
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25602978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e85e5479fe","size":5,"soft_fail":false,"target_va":5383204960,"va":5408807933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24055493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e83bf190fe","size":5,"soft_fail":false,"target_va":5384751936,"va":5408807424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24082586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e8668790fe","size":5,"soft_fail":false,"target_va":5384724912,"va":5408807493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::util::hash_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807376,"raw_hex":"e8a0efffff","size":5,"soft_fail":false,"target_va":5408803280,"va":5408807467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$75::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3339,"record_offset":15968,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":38873,"source_line":4000,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3339,"record_offset":15132,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":6562,"source_line":1708,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3339,"record_offset":14684,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3339,"record_offset":14656,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":3339,"record_offset":14588,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":3339,"record_offset":14560,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":3339,"record_offset":14944,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::FileDependency::missing
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3339,"record_offset":15824,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":120957,"source_line":28,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::FileDependency::present
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3339,"record_offset":15996,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":120958,"source_line":18,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::FileStamp::from_metadata
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3339,"record_offset":14972,"parent_record_offset":14396,"ancestor_record_offset":14396,"inlinee":86214,"source_line":75,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":14836,"parent_record_offset":14812,"ancestor_record_offset":14396,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":14860,"parent_record_offset":14836,"ancestor_record_offset":14396,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":15348,"parent_record_offset":15324,"ancestor_record_offset":14396,"inlinee":6570,"source_line":353,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":15372,"parent_record_offset":15348,"ancestor_record_offset":14396,"inlinee":6571,"source_line":264,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":14888,"parent_record_offset":14860,"ancestor_record_offset":14396,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":14812,"parent_record_offset":14756,"ancestor_record_offset":14396,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":15324,"parent_record_offset":15284,"ancestor_record_offset":14396,"inlinee":6569,"source_line":471,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":14396,"inlinee":6572,"source_line":134,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":15200,"parent_record_offset":15168,"ancestor_record_offset":14396,"inlinee":6565,"source_line":398,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":14784,"parent_record_offset":14756,"ancestor_record_offset":14396,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":14396,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":15256,"parent_record_offset":15224,"ancestor_record_offset":14396,"inlinee":6567,"source_line":811,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":15224,"parent_record_offset":15200,"ancestor_record_offset":14396,"inlinee":6566,"source_line":743,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":15284,"parent_record_offset":15256,"ancestor_record_offset":14396,"inlinee":6568,"source_line":827,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":14396,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3339,"record_offset":16188,"parent_record_offset":16164,"ancestor_record_offset":14396,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3339,"record_offset":15168,"parent_record_offset":15132,"ancestor_record_offset":14396,"inlinee":6563,"source_line":1579,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3339,"record_offset":15852,"parent_record_offset":15824,"ancestor_record_offset":14396,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3339,"record_offset":15028,"parent_record_offset":15000,"ancestor_record_offset":14396,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3339,"record_offset":15000,"parent_record_offset":14972,"ancestor_record_offset":14396,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3339,"record_offset":14708,"parent_record_offset":14684,"ancestor_record_offset":14396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3339,"record_offset":14616,"parent_record_offset":14588,"ancestor_record_offset":14396,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3339,"record_offset":15436,"parent_record_offset":15284,"ancestor_record_offset":14396,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3339,"record_offset":16224,"parent_record_offset":16188,"ancestor_record_offset":14396,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3339,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":14396,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3339,"record_offset":16280,"parent_record_offset":16188,"ancestor_record_offset":14396,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":3339,"record_offset":15088,"parent_record_offset":15056,"ancestor_record_offset":14396,"inlinee":52443,"source_line":631,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":3339,"record_offset":15900,"parent_record_offset":15876,"ancestor_record_offset":14396,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3339,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":14396,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":3339,"record_offset":15924,"parent_record_offset":15900,"ancestor_record_offset":14396,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::system_time_ns
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":3339,"record_offset":15056,"parent_record_offset":15028,"ancestor_record_offset":14396,"inlinee":86215,"source_line":168,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::FileObservedState::read
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x150C2
// 首条位置: {"module":1777,"inlinee":86210}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1777,"record_offset":105676,"parent_record_offset":105636,"ancestor_record_offset":105636,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":1777,"record_offset":105744,"parent_record_offset":105636,"ancestor_record_offset":105636,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::util::FileStamp::from_metadata
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1777,"record_offset":105772,"parent_record_offset":105636,"ancestor_record_offset":105636,"inlinee":86214,"source_line":75,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1777,"record_offset":105820,"parent_record_offset":105796,"ancestor_record_offset":105636,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":105796,"parent_record_offset":105772,"ancestor_record_offset":105636,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1777,"record_offset":105704,"parent_record_offset":105676,"ancestor_record_offset":105636,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1777,"record_offset":105872,"parent_record_offset":105844,"ancestor_record_offset":105636,"inlinee":52443,"source_line":631,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::util::system_time_ns
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1777,"record_offset":105844,"parent_record_offset":105820,"ancestor_record_offset":105636,"inlinee":86215,"source_line":168,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::FileObservedState::matches_dependency
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x150C4
// 首条位置: {"module":1777,"inlinee":86212}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] turbosloth::util::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/util.rs
//     evidence={"module":1777,"record_offset":107064,"parent_record_offset":107028,"ancestor_record_offset":107028,"inlinee":31803,"source_line":68,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":107092,"parent_record_offset":107064,"ancestor_record_offset":107028,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::FileObservedState::from_dependency
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x150C3
// 首条位置: {"module":1777,"inlinee":86211}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::system_time_ns
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x150C7
// 首条位置: {"module":1777,"inlinee":86215}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1777,"record_offset":105872,"parent_record_offset":105844,"ancestor_record_offset":105844,"inlinee":52443,"source_line":631,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::hash_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3339,"record_offset":4544,"rva":40094160,"va":5408803280}
// 编译实例 RVAs: 0x263c9d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] std_detect::detect::cache::detect_and_initialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7823703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408803280,"raw_hex":"e857617700","size":5,"soft_fail":false,"target_va":5416630640,"va":5408806932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] twox_hash::xxhash3_128::mix_two_chunks
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"twox-hash","version":"1.6.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-15713807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408803280,"raw_hex":"e8f13910ff","size":5,"soft_fail":false,"target_va":5393090512,"va":5408804314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] twox_hash::xxhash3_128::impl_241_plus_bytes::do_avx2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"twox-hash","version":"1.6.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t1604","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5408803280,"raw_hex":"e944060000","size":5,"soft_fail":false,"target_va":5408808656,"va":5408807047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] twox_hash::xxhash3_128::Hasher::oneshot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":4656,"parent_record_offset":4544,"ancestor_record_offset":4544,"inlinee":81827,"source_line":35,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":3339,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":4544,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3339,"record_offset":8724,"parent_record_offset":7732,"ancestor_record_offset":4544,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3339,"record_offset":8748,"parent_record_offset":8724,"ancestor_record_offset":4544,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3339,"record_offset":8804,"parent_record_offset":8748,"ancestor_record_offset":4544,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3339,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":4544,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3339,"record_offset":12368,"parent_record_offset":12328,"ancestor_record_offset":4544,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$65::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":3339,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":4544,"inlinee":19427,"source_line":79,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$72::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":3339,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":4544,"inlinee":9260,"source_line":79,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_add_epi64
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3339,"record_offset":5708,"parent_record_offset":5472,"ancestor_record_offset":4544,"inlinee":48811,"source_line":116,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3339,"record_offset":5556,"parent_record_offset":5472,"ancestor_record_offset":4544,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_mul_epu32
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3339,"record_offset":5788,"parent_record_offset":5472,"ancestor_record_offset":4544,"inlinee":48809,"source_line":347,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_srli_epi64
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3339,"record_offset":5748,"parent_record_offset":5472,"ancestor_record_offset":4544,"inlinee":48810,"source_line":830,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_xor_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3339,"record_offset":5668,"parent_record_offset":5472,"ancestor_record_offset":4544,"inlinee":19535,"source_line":901,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3339,"record_offset":13180,"parent_record_offset":13152,"ancestor_record_offset":4544,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":3339,"record_offset":8224,"parent_record_offset":7732,"ancestor_record_offset":4544,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3339,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":4544,"inlinee":5521,"source_line":587,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3339,"record_offset":9184,"parent_record_offset":9160,"ancestor_record_offset":4544,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3339,"record_offset":5876,"parent_record_offset":5420,"ancestor_record_offset":4544,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3339,"record_offset":8280,"parent_record_offset":8252,"ancestor_record_offset":4544,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3339,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":4544,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3339,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":4544,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3339,"record_offset":9160,"parent_record_offset":7732,"ancestor_record_offset":4544,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3339,"record_offset":8308,"parent_record_offset":8280,"ancestor_record_offset":4544,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3339,"record_offset":8776,"parent_record_offset":8748,"ancestor_record_offset":4544,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3339,"record_offset":7332,"parent_record_offset":7256,"ancestor_record_offset":4544,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::swap_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3339,"record_offset":13152,"parent_record_offset":12980,"ancestor_record_offset":4544,"inlinee":13482,"source_line":559,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::swap_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3339,"record_offset":12456,"parent_record_offset":12160,"ancestor_record_offset":4544,"inlinee":23560,"source_line":559,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3339,"record_offset":7372,"parent_record_offset":7256,"ancestor_record_offset":4544,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3339,"record_offset":6036,"parent_record_offset":5984,"ancestor_record_offset":4544,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3339,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":4544,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":3339,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":4544,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3339,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":4544,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3339,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":4544,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3339,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":4544,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3339,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":4544,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3339,"record_offset":9304,"parent_record_offset":9280,"ancestor_record_offset":4544,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3339,"record_offset":8364,"parent_record_offset":8336,"ancestor_record_offset":4544,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3339,"record_offset":9280,"parent_record_offset":9256,"ancestor_record_offset":4544,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3339,"record_offset":5140,"parent_record_offset":5100,"ancestor_record_offset":4544,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3339,"record_offset":9924,"parent_record_offset":7732,"ancestor_record_offset":4544,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3339,"record_offset":5288,"parent_record_offset":5260,"ancestor_record_offset":4544,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3339,"record_offset":5192,"parent_record_offset":5164,"ancestor_record_offset":4544,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3339,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":4544,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3339,"record_offset":8336,"parent_record_offset":8308,"ancestor_record_offset":4544,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3339,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":4544,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3339,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":4544,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3339,"record_offset":4936,"parent_record_offset":4900,"ancestor_record_offset":4544,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3339,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":4544,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::arch::x86::__is_feature_detected::avx2
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\macros.rs
//     evidence={"module":3339,"record_offset":4816,"parent_record_offset":4768,"ancestor_record_offset":4544,"inlinee":69386,"source_line":197,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::Cache::test
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":3339,"record_offset":4900,"parent_record_offset":4872,"ancestor_record_offset":4544,"inlinee":19908,"source_line":90,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":3339,"record_offset":4872,"parent_record_offset":4844,"ancestor_record_offset":4544,"inlinee":19906,"source_line":194,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":3339,"record_offset":4992,"parent_record_offset":4900,"ancestor_record_offset":4544,"inlinee":19910,"source_line":202,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::check_for
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\mod.rs
//     evidence={"module":3339,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":4544,"inlinee":19904,"source_line":89,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::impl$0::into_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/lib.rs
//     evidence={"module":3339,"record_offset":13064,"parent_record_offset":13028,"ancestor_record_offset":4544,"inlinee":82239,"source_line":58,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::impl$2::into_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/lib.rs
//     evidence={"module":3339,"record_offset":12328,"parent_record_offset":12160,"ancestor_record_offset":4544,"inlinee":81866,"source_line":75,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::avalanche
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":7408,"parent_record_offset":7256,"ancestor_record_offset":4544,"inlinee":81855,"source_line":182,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::avalanche_xxh64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":13212,"parent_record_offset":12980,"ancestor_record_offset":4544,"inlinee":82240,"source_line":190,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::impl$6::first_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":12696,"parent_record_offset":12640,"ancestor_record_offset":4544,"inlinee":82235,"source_line":275,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::impl$6::last_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":12216,"parent_record_offset":12160,"ancestor_record_offset":4544,"inlinee":81864,"source_line":290,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::impl$7::bp_as_chunks
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":6236,"parent_record_offset":6208,"ancestor_record_offset":4544,"inlinee":81854,"source_line":304,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::impl$7::bp_as_rchunks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":9756,"parent_record_offset":7732,"ancestor_record_offset":4544,"inlinee":81858,"source_line":330,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::impl_17_to_128_bytes_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":10636,"parent_record_offset":10560,"ancestor_record_offset":4544,"inlinee":81857,"source_line":125,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::impl_1_to_3_bytes_combined
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":13028,"parent_record_offset":12980,"ancestor_record_offset":4544,"inlinee":82237,"source_line":114,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::Algorithm::final_merge
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large.rs
//     evidence={"module":3339,"record_offset":7256,"parent_record_offset":6120,"ancestor_record_offset":4544,"inlinee":81872,"source_line":278,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::Algorithm::finalize_128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large.rs
//     evidence={"module":3339,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":4544,"inlinee":81869,"source_line":229,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::Algorithm::last_round
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large.rs
//     evidence={"module":3339,"record_offset":6160,"parent_record_offset":6120,"ancestor_record_offset":4544,"inlinee":81870,"source_line":252,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::Algorithm::oneshot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large.rs
//     evidence={"module":3339,"record_offset":5100,"parent_record_offset":5068,"ancestor_record_offset":4544,"inlinee":81835,"source_line":145,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::Algorithm::round
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large.rs
//     evidence={"module":3339,"record_offset":5388,"parent_record_offset":5228,"ancestor_record_offset":4544,"inlinee":81837,"source_line":190,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::Algorithm::round_accumulate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large.rs
//     evidence={"module":3339,"record_offset":5420,"parent_record_offset":5388,"ancestor_record_offset":4544,"inlinee":81838,"source_line":198,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::Algorithm::rounds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large.rs
//     evidence={"module":3339,"record_offset":5228,"parent_record_offset":5100,"ancestor_record_offset":4544,"inlinee":81836,"source_line":176,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::scalar::impl$0::round_scramble
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large/scalar.rs
//     evidence={"module":3339,"record_offset":5984,"parent_record_offset":5960,"ancestor_record_offset":4544,"inlinee":81846,"source_line":9,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::sse2::accumulate_sse2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large/sse2.rs
//     evidence={"module":3339,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":4544,"inlinee":81842,"source_line":48,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::sse2::impl$1::accumulate
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large/sse2.rs
//     evidence={"module":3339,"record_offset":5448,"parent_record_offset":5420,"ancestor_record_offset":4544,"inlinee":81840,"source_line":27,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::sse2::impl$1::round_scramble
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large/sse2.rs
//     evidence={"module":3339,"record_offset":5936,"parent_record_offset":5388,"ancestor_record_offset":4544,"inlinee":81843,"source_line":21,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::large::sse2::round_scramble_sse2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large/sse2.rs
//     evidence={"module":3339,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":4544,"inlinee":81844,"source_line":38,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::mix_step
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":8028,"parent_record_offset":7928,"ancestor_record_offset":4544,"inlinee":81851,"source_line":153,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::pairs_of_u64_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":7840,"parent_record_offset":7732,"ancestor_record_offset":4544,"inlinee":81852,"source_line":175,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::stripes_with_tail
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":6208,"parent_record_offset":6160,"ancestor_record_offset":4544,"inlinee":81871,"source_line":200,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3::to_u64s
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3.rs
//     evidence={"module":3339,"record_offset":7972,"parent_record_offset":7928,"ancestor_record_offset":4544,"inlinee":81850,"source_line":168,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::finalize_medium
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":10256,"parent_record_offset":7732,"ancestor_record_offset":4544,"inlinee":81861,"source_line":401,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl$3::large
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":6096,"parent_record_offset":5100,"ancestor_record_offset":4544,"inlinee":81868,"source_line":208,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_129_to_240_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":7732,"parent_record_offset":4684,"ancestor_record_offset":4544,"inlinee":81847,"source_line":352,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_17_to_128_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":10560,"parent_record_offset":4684,"ancestor_record_offset":4544,"inlinee":81856,"source_line":339,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_17_to_128_bytes::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":10816,"parent_record_offset":10636,"ancestor_record_offset":4544,"inlinee":81860,"source_line":344,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_1_to_3_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":12980,"parent_record_offset":4684,"ancestor_record_offset":4544,"inlinee":82236,"source_line":251,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_241_plus_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":4768,"parent_record_offset":4684,"ancestor_record_offset":4544,"inlinee":81830,"source_line":415,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_241_plus_bytes::do_sse2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3/large.rs
//     evidence={"module":3339,"record_offset":5036,"parent_record_offset":4768,"ancestor_record_offset":4544,"inlinee":81832,"source_line":74,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_4_to_8_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":12640,"parent_record_offset":4684,"ancestor_record_offset":4544,"inlinee":82234,"source_line":273,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_9_to_16_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":12160,"parent_record_offset":4684,"ancestor_record_offset":4544,"inlinee":81862,"source_line":306,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::impl_oneshot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":4684,"parent_record_offset":4656,"ancestor_record_offset":4544,"inlinee":81829,"source_line":222,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::mix_two_chunks
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":7928,"parent_record_offset":7732,"ancestor_record_offset":4544,"inlinee":81848,"source_line":384,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] twox_hash::xxhash3_128::oneshot_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"twox-hash","version":"2.1.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/twox-hash-2.1.2/src/xxhash3_128.rs
//     evidence={"module":3339,"record_offset":5068,"parent_record_offset":5036,"ancestor_record_offset":4544,"inlinee":81833,"source_line":424,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::util::write_atomic
// 记录形式: compiled_procedure; instances=1
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3339,"record_offset":13412,"rva":40097936,"va":5408807056}
// 编译实例 RVAs: 0x263d890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_with_extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24151540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807056,"raw_hex":"e80c7a8ffe","size":5,"soft_fail":false,"target_va":5384655584,"va":5408807119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-33128510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807056,"raw_hex":"e8c27f06fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5408807209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24054690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807056,"raw_hex":"e85ef490fe","size":5,"soft_fail":false,"target_va":5384752464,"va":5408807149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24082738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408807056,"raw_hex":"e8ce8690fe","size":5,"soft_fail":false,"target_va":5384724448,"va":5408807181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3339,"record_offset":14024,"parent_record_offset":13412,"ancestor_record_offset":13412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":3339,"record_offset":13624,"parent_record_offset":13412,"ancestor_record_offset":13412,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":3339,"record_offset":13568,"parent_record_offset":13412,"ancestor_record_offset":13412,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::with_extension
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3339,"record_offset":13540,"parent_record_offset":13412,"ancestor_record_offset":13412,"inlinee":38705,"source_line":3081,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":13900,"parent_record_offset":13876,"ancestor_record_offset":13412,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":13412,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":13948,"parent_record_offset":13924,"ancestor_record_offset":13412,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3339,"record_offset":13876,"parent_record_offset":13820,"ancestor_record_offset":13412,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":13848,"parent_record_offset":13820,"ancestor_record_offset":13412,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":13820,"parent_record_offset":13796,"ancestor_record_offset":13412,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3339,"record_offset":13796,"parent_record_offset":13772,"ancestor_record_offset":13412,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3339,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":13412,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/03339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3339,"record_offset":13652,"parent_record_offset":13624,"ancestor_record_offset":13412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03339.jsonl.gz"}
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
// 完整结构及原始类型记录: util.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] turbosloth::util::FileDependency
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["turbosloth::util::FileDependency", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::set::IntoIter<turbosloth::util::FileDependency,alloc::alloc::Global>", "constructor": "std::collections::hash::set::IntoIter", "arguments": ["turbosloth::util::FileDependency", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::set::IntoIter<turbosloth::util::FileDependency,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::set::IntoIter<turbosloth::util::FileDependency,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::impl$0::equivalent<turbosloth::util::FileDependency,turbosloth::util::FileDependency>", "constructor": "hashbrown::impl$0::equivalent", "arguments": ["turbosloth::util::FileDependency", "turbosloth::util::FileDependency"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::set::HashSet<turbosloth::util::FileDependency,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::set::HashSet", "arguments": ["turbosloth::util::FileDependency", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::Mutex::lock<std::collections::hash::set::HashSet<turbosloth::util::FileDependency,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::Mutex::lock", "arguments": ["std::collections::hash::set::HashSet<turbosloth::util::FileDependency,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<turbosloth::util::FileDependency,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["turbosloth::util::FileDependency", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<turbosloth::util::FileDependency,std::collections::hash::set::IntoIter<turbosloth::util::FileDependency,alloc::alloc::Global> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["turbosloth::util::FileDependency", "std::collections::hash::set::IntoIter<turbosloth::util::FileDependency,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::MutexGuard<std::collections::hash::set::HashSet<turbosloth::util::FileDependency,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::MutexGuard", "arguments": ["std::collections::hash::set::HashSet<turbosloth::util::FileDependency,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<std::collections::hash::set::HashSet<turbosloth::util::FileDependency,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::mutex::MutexGuard<std::collections::hash::set::HashSet<turbosloth::util::FileDependency,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<turbosloth::util::FileDependency,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["turbosloth::util::FileDependency", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >,ref$<alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >", "ref$<alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<turbosloth::util::FileDependency>", "constructor": "ref$", "arguments": ["turbosloth::util::FileDependency"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<turbosloth::util::FileDependency> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<turbosloth::util::FileDependency>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<turbosloth::util::FileDependency,tuple$<> >", "constructor": "tuple$", "arguments": ["turbosloth::util::FileDependency", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<turbosloth::util::FileDependency,tuple$<>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["turbosloth::util::FileDependency", "tuple$<>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<turbosloth::util::FileDependency,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::util::FileDependency,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<turbosloth::util::FileDependency,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<turbosloth::util::FileDependency,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<turbosloth::util::FileDependency,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::util::FileDependency,tuple$<>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<turbosloth::util::FileDependency,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::util::FileDependency,tuple$<>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<turbosloth::util::FileDependency,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::util::FileDependency,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<turbosloth::util::FileDependency,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<turbosloth::util::FileDependency,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<turbosloth::util::FileDependency,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["turbosloth::util::FileDependency", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<turbosloth::util::FileDependency,bincode::de::impl$3::deserialize_tuple::Access<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["turbosloth::util::FileDependency", "bincode::de::impl$3::deserialize_tuple::Access<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize<turbosloth::util::FileDependency,ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >", "constructor": "serde_core::de::impls::impl$19::deserialize", "arguments": ["turbosloth::util::FileDependency", "ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] turbosloth::util::FileDependencyState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] turbosloth::util::FileObservedState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] turbosloth::util::FileStamp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] turbosloth::util::SourceFileSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<turbosloth::util::SourceFileSnapshot>", "constructor": "core::ptr::drop_in_place", "arguments": ["turbosloth::util::SourceFileSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<turbosloth::util::SourceFileSnapshot,std::io::error::Error>", "constructor": "core::result::Result", "arguments": ["turbosloth::util::SourceFileSnapshot", "std::io::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<turbosloth::util::SourceFileSnapshot,std::io::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<turbosloth::util::SourceFileSnapshot,std::io::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<turbosloth::util::SourceFileSnapshot,std::io::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<turbosloth::util::SourceFileSnapshot,std::io::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: util.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: util.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: turbosloth::util::FileDependency::missing
// 信息 ID: 56a15067d2e8b1667302cda7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::util::SourceFileSnapshot::read; module=3339, inline_record=15824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::FileDependency::present
// 信息 ID: fc228b423037a511c8a00501
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::util::SourceFileSnapshot::read; module=3339, inline_record=15996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::FileDependencyState::content_matches
// 信息 ID: 5b6ba2424c37f51f2d889bb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=108664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::FileObservedState::from_dependency
// 信息 ID: cb75b0bd4793a6bff958ea44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=105920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::FileObservedState::matches_dependency
// 信息 ID: c825d27f4ff368b0a5d6559e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=107028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::FileObservedState::read
// 信息 ID: afbe6d1db1ff5882d185dc17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=105636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::FileStamp::from_metadata
// 信息 ID: 7036aa071e17fc60f5b071d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::store::PackagePass::install_closure @ 0x1b1afb7 -> 0x263de50 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::PackagePass::install_closure @ 0x1b1b26a -> 0x263de50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=105772; 不是独立调用边。
// [PDB 内联宿主] turbosloth::util::SourceFileSnapshot::read; module=3339, inline_record=14972; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::SourceFileSnapshot::read
// 信息 ID: 4442bdca55708802fd785a9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::store::SourceFileSlot::read @ 0x1b1c1aa -> 0x263d9d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::_::impl$0::deserialize
// 信息 ID: 2768f3029062ee613fb9add7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bincode::de::impl$3::deserialize_map<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite>,serde_core::de::impls::impl$204::deserialize::MapVisitor<turbosloth::store::AssetRecipeId,turbosloth::store::PackagedFileState,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher> > >; module=3343, inline_record=9960; 不是独立调用边。
// [PDB 内联宿主] bincode::de::impl$3::deserialize_struct<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite>,turbosloth::util::_::impl$0::deserialize::__Visitor>; module=3343, inline_record=15136; 不是独立调用边。
// [PDB 内联宿主] turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum<ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=3348, inline_record=9940; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<turbosloth::util::FileDependency,bincode::de::impl$3::deserialize_tuple::Access<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >; module=4478, inline_record=1852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 1a99f4f6d4cd91f985ae5203
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum<ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=3348, inline_record=9208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: d6ff562fe15fd6ada572d56f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum<ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=3348, inline_record=9096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: 93fbcca6e32ca89589ddf25c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum<ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=3348, inline_record=9352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum<ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >
// 信息 ID: b62bb63bbf4fae26b0e5a3c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] bincode::config::WithOtherIntEncoding -> ["bincode::config::DefaultOptions", "bincode::config::int::FixintEncoding"]; 字段布局 = unknown。
// [类型关系] bincode::config::WithOtherTrailing -> ["bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>", "bincode::config::trailing::AllowTrailing"]; 字段布局 = unknown。
// [类型关系] bincode::config::WithOtherLimit -> ["bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>", "bincode::config::limit::Infinite"]; 字段布局 = unknown。
// [类型关系] bincode::de::Deserializer -> ["bincode::de::read::SliceReader", "bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> >"]; 字段布局 = unknown。
// [类型关系] turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum -> ["ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bincode::de::impl$3::deserialize_struct<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite>,turbosloth::util::_::impl$0::deserialize::__Visitor> @ 0x263ff0d -> 0x2642a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 5c0b4fbb4d3a45d21ac353c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bincode::de::impl$3::deserialize_map<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite>,serde_core::de::impls::impl$204::deserialize::MapVisitor<turbosloth::store::AssetRecipeId,turbosloth::store::PackagedFileState,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher> > >; module=3343, inline_record=10044; 不是独立调用边。
// [PDB 内联宿主] bincode::de::impl$3::deserialize_struct<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite>,turbosloth::util::_::impl$0::deserialize::__Visitor>; module=3343, inline_record=14828; 不是独立调用边。
// [PDB 内联宿主] turbosloth::util::_::impl$0::deserialize::impl$2::visit_enum<ref_mut$<bincode::de::Deserializer<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=3348, inline_record=10020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::_::impl$0::serialize
// 信息 ID: 9eb81c0e1b53c6b67e83e2a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::util::_::impl$0::serialize<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=529, inline_record=4260; 不是独立调用边。
// [PDB 内联宿主] turbosloth::util::_::impl$0::serialize<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=529, inline_record=6256; 不是独立调用边。
// [PDB 内联宿主] bincode::internal::serialize<turbosloth::store::DiskBuildRecord,bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing> >; module=599, inline_record=80616; 不是独立调用边。
// [PDB 内联宿主] bincode::internal::serialize<turbosloth::store::DiskBuildRecord,bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing> >; module=599, inline_record=81388; 不是独立调用边。
// [PDB 内联宿主] turbosloth::util::_::impl$0::serialize<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=3348, inline_record=5372; 不是独立调用边。
// [PDB 内联宿主] turbosloth::util::_::impl$0::serialize<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >; module=3348, inline_record=7368; 不是独立调用边。
// [PDB 内联宿主] bincode::internal::serialize<turbosloth::store::PackageState,bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing> >; module=3350, inline_record=1820; 不是独立调用边。
// [PDB 内联宿主] bincode::internal::serialize<turbosloth::store::DiskBuildRecord,bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing> >; module=3350, inline_record=4624; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::_::impl$0::serialize<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > > >
// 信息 ID: 382982d4e7e04472f5a8c4a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] bincode::config::WithOtherIntEncoding -> ["bincode::config::DefaultOptions", "bincode::config::int::FixintEncoding"]; 字段布局 = unknown。
// [类型关系] bincode::config::WithOtherTrailing -> ["bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>", "bincode::config::trailing::AllowTrailing"]; 字段布局 = unknown。
// [类型关系] bincode::config::WithOtherLimit -> ["bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>", "bincode::config::limit::Infinite"]; 字段布局 = unknown。
// [类型关系] bincode::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> >"]; 字段布局 = unknown。
// [类型关系] turbosloth::util::_::impl$0::serialize -> ["ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >,ref$<alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global> > > @ 0x17b266c -> 0xa0a3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_map<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >,ref$<turbosloth::store::AssetRecipeId>,ref$<turbosloth::store::PackagedFileState>,ref$<std::collections::hash::map::HashMap<turbosloth::store::AssetRecipeId,turbosloth::store::PackagedFileState,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher>,alloc::alloc::Global> > > @ 0x2a66b78 -> 0x2642720 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<bincode::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite> > >,ref$<alloc::vec::Vec<turbosloth::util::FileDependency,alloc::alloc::Global> > > @ 0x2a66c7c -> 0x2642860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::hash_bytes
// 信息 ID: 98afdedeb6a1224d32780e38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::store::PackagePass::begin @ 0x1b1b714 -> 0x263c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::AssetContentId::from_bytes @ 0x1b1c130 -> 0x263c9d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::artifact_bytes @ 0x1b1d706 -> 0x263c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::util::SourceFileSnapshot::read @ 0x263da2b -> 0x263c9d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$10::eq
// 信息 ID: 65ac6f69b3c685596f0212f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::impl$0::equivalent<turbosloth::util::FileDependency,turbosloth::util::FileDependency>; module=529, inline_record=75896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$11::clone
// 信息 ID: e16267bba08eb45285611c82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<turbosloth::util::FileDependency,alloc::alloc::Global>; module=1643, inline_record=260892; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::SourceFileSlot::read; module=1777, inline_record=44048; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::SourceFileSlot::read; module=1777, inline_record=45200; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=107996; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=110360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$14::hash
// 信息 ID: 876f6e64cb0f4cc0e5c42e01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<turbosloth::util::FileDependency> >; module=1660, inline_record=55068; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<turbosloth::util::FileDependency> >; module=3394, inline_record=3144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$16::eq
// 信息 ID: fa7241435fa63fbaac228fb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::impl$0::equivalent<turbosloth::util::FileDependency,turbosloth::util::FileDependency>; module=529, inline_record=76284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$22::hash
// 信息 ID: 9ae8870435ee9b4231b94182
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<turbosloth::util::FileDependency> >; module=1660, inline_record=55404; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<turbosloth::util::FileDependency> >; module=3394, inline_record=3480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$24::eq
// 信息 ID: 3c36c81133acd8fa869a195e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::impl$0::equivalent<turbosloth::util::FileDependency,turbosloth::util::FileDependency>; module=529, inline_record=76396; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::PackagePass::install_closure; module=1777, inline_record=24860; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=107064; 不是独立调用边。
// [PDB 内联宿主] std::collections::hash::map::impl$6::eq<turbosloth::store::AssetRecipeId,turbosloth::store::PackagedFileState,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher>,alloc::alloc::Global>; module=3349, inline_record=2600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$25::clone
// 信息 ID: 69afa3b7f8217706e4f71d66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::store::SourceFileSlot::read; module=1777, inline_record=43160; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::SourceFileSlot::read; module=1777, inline_record=44328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$5::clone
// 信息 ID: b8beb88f30210ae5a5e1d350
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<turbosloth::util::FileDependency,alloc::alloc::Global>; module=1643, inline_record=260164; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::SourceFileSlot::read; module=1777, inline_record=43332; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::SourceFileSlot::read; module=1777, inline_record=44500; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=107156; 不是独立调用边。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=109632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::impl$8::hash
// 信息 ID: ed48adcbdfe1041558dc7df7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<turbosloth::util::FileDependency> >; module=1660, inline_record=54620; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<turbosloth::util::FileDependency> >; module=3394, inline_record=3116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::normalize_file_path<ref$<std::path::Path> >
// 信息 ID: a85e6f3c219bd6e4493c4825
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::Path"]; 字段布局 = unknown。
// [类型关系] turbosloth::util::normalize_file_path -> ["ref$<std::path::Path>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::store::PackagePass::begin @ 0x1b1b6ec -> 0x26425a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::normalize_file_path<ref$<std::path::PathBuf> >
// 信息 ID: c663b26bb40e3314f3b47467
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [类型关系] turbosloth::util::normalize_file_path -> ["ref$<std::path::PathBuf>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::impl$0::load::async_fn$0 @ 0x8f862e -> 0xa0a260 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$38::read_file::async_fn$0<ref$<std::path::PathBuf> > @ 0x1b25d05 -> 0xa0a260 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterGlbImport,anyhow::Error,clutter_pack_pipe::ImportClutterGlb> @ 0x257fc3f -> 0xa0a260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::system_time_ns
// 信息 ID: a9265b43e1a70de75b61b9b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state; module=1777, inline_record=105844; 不是独立调用边。
// [PDB 内联宿主] turbosloth::util::SourceFileSnapshot::read; module=3339, inline_record=15056; 不是独立调用边。
// [PDB 内联宿主] turbosloth::util::FileStamp::from_metadata; module=3339, inline_record=16752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::util::write_atomic
// 信息 ID: d50eb0bf45fe710f808663e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::store::DiskStore::store<asset_pipe::texture::TextureAssetList> @ 0x1b0f4be -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<clutter_pack_pipe::ClutterObjectList> @ 0x1b10377 -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<clutter_pack_pipe::ClutterMeshAsset> @ 0x1b11227 -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<turbosloth::asset::FileAsset> @ 0x1b120d0 -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<clutter_pack_pipe::ClutterGlbImport> @ 0x1b12f87 -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<asset::texture::blob::TextureAssetBlob> @ 0x1b13e3e -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<clutter_pack_pipe::ClutterIconImage> @ 0x1b14cf7 -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<asset::texture::SourceImageAsset> @ 0x1b15ba0 -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::PackagePass::install_closure @ 0x1b1aef1 -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::PackagePass::finish @ 0x1b1bd86 -> 0x263d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::write_package_manifest @ 0x1b1d10a -> 0x263d890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
