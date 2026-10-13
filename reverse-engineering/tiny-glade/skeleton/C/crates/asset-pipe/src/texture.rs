// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset-pipe/src/texture.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 283b7b4c6241a47828499680e51d5cb5245cfaf31e0a9e0fe3272af6621940b4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 501, 515, 687, 1253, 1643, 1647, 1651, 1654, 1663, 1666, 1670, 1671, 1749, 2061, 3252, 3257, 3529, 3579, 4413, 4565, 4570, 4581, 4582, 4972
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.0.1"}]
// use rkyv; // namespace candidate
//   package identities: [{"name":"rkyv","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.16"}]
// use std; // namespace candidate
//   package identities: []
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$10::check_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E86
// 首条位置: {"module":1670,"inlinee":28294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::blob::_::impl$1::check_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":1670,"record_offset":4324,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":28091,"source_line":5,"archive":"_pdb/C/modules/01670.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1670,"record_offset":4264,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01670.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$11::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F3E
// 首条位置: {"module":1663,"inlinee":28478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::blob::impl$27::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":1663,"record_offset":20216,"parent_record_offset":19584,"ancestor_record_offset":19584,"inlinee":28269,"source_line":197,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$72::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1663,"record_offset":20128,"parent_record_offset":19584,"ancestor_record_offset":19584,"inlinee":28265,"source_line":125,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::string::impl$1::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/string.rs
//     evidence={"module":1663,"record_offset":19616,"parent_record_offset":19584,"ancestor_record_offset":19584,"inlinee":28259,"source_line":27,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1663,"record_offset":20068,"parent_record_offset":19616,"ancestor_record_offset":19584,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1663,"record_offset":20092,"parent_record_offset":20068,"ancestor_record_offset":19584,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1663,"record_offset":19956,"parent_record_offset":19924,"ancestor_record_offset":19584,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1663,"record_offset":20180,"parent_record_offset":20156,"ancestor_record_offset":19584,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1663,"record_offset":19896,"parent_record_offset":19812,"ancestor_record_offset":19584,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1663,"record_offset":19856,"parent_record_offset":19812,"ancestor_record_offset":19584,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$20::serialize_unsized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1663,"record_offset":19684,"parent_record_offset":19644,"ancestor_record_offset":19584,"inlinee":28263,"source_line":391,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::option::impl$2::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/option.rs
//     evidence={"module":1663,"record_offset":20156,"parent_record_offset":20128,"ancestor_record_offset":19584,"inlinee":28267,"source_line":64,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::impl$1::pos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/mod.rs
//     evidence={"module":1663,"record_offset":20028,"parent_record_offset":20004,"ancestor_record_offset":19584,"inlinee":28186,"source_line":51,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::impl$2::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/mod.rs
//     evidence={"module":1663,"record_offset":19748,"parent_record_offset":19716,"ancestor_record_offset":19584,"inlinee":28175,"source_line":57,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::writer::alloc::impl$3::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/writer/alloc.rs
//     evidence={"module":1663,"record_offset":19780,"parent_record_offset":19748,"ancestor_record_offset":19584,"inlinee":28177,"source_line":29,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::writer::impl$2::pos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/writer/mod.rs
//     evidence={"module":1663,"record_offset":20004,"parent_record_offset":19684,"ancestor_record_offset":19584,"inlinee":28184,"source_line":45,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::writer::impl$4::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/writer/mod.rs
//     evidence={"module":1663,"record_offset":19716,"parent_record_offset":19684,"ancestor_record_offset":19584,"inlinee":28173,"source_line":77,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::string::ArchivedString::serialize_from_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/string/mod.rs
//     evidence={"module":1663,"record_offset":19644,"parent_record_offset":19616,"ancestor_record_offset":19584,"inlinee":28261,"source_line":81,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::alloc::aligned_vec::AlignedVec::extend_from_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/util/alloc/aligned_vec.rs
//     evidence={"module":1663,"record_offset":19812,"parent_record_offset":19780,"ancestor_record_offset":19584,"inlinee":28179,"source_line":567,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::alloc::aligned_vec::AlignedVec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/util/alloc/aligned_vec.rs
//     evidence={"module":1663,"record_offset":19924,"parent_record_offset":19812,"ancestor_record_offset":19584,"inlinee":28180,"source_line":398,"archive":"_pdb/C/modules/01663.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$12::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E1E
// 首条位置: {"module":501,"inlinee":28190}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::blob::impl$28::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":501,"record_offset":64276,"parent_record_offset":64168,"ancestor_record_offset":64168,"inlinee":28194,"source_line":197,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$73::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":64192,"parent_record_offset":64168,"ancestor_record_offset":64168,"inlinee":28192,"source_line":126,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":64300,"parent_record_offset":64168,"ancestor_record_offset":64168,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$24::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":64244,"parent_record_offset":64192,"ancestor_record_offset":64168,"inlinee":28198,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::option::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/option.rs
//     evidence={"module":501,"record_offset":64216,"parent_record_offset":64192,"ancestor_record_offset":64168,"inlinee":28196,"source_line":80,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$13::visit_asset_dependencies
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB246
// 首条位置: {"module":687,"inlinee":45638}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] turbosloth::asset::impl$15::visit_asset_dependencies
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":687,"record_offset":5732,"parent_record_offset":5712,"ancestor_record_offset":5712,"inlinee":42050,"source_line":267,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$6::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D79
// 首条位置: {"module":1643,"inlinee":81273}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1643,"record_offset":269860,"parent_record_offset":269824,"ancestor_record_offset":269824,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$7::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6DF3
// 首条位置: {"module":501,"inlinee":28147}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":501,"record_offset":60864,"parent_record_offset":60832,"ancestor_record_offset":60832,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::blob::impl$21::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":501,"record_offset":61088,"parent_record_offset":60832,"ancestor_record_offset":60832,"inlinee":28155,"source_line":197,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$19::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":60976,"parent_record_offset":60832,"ancestor_record_offset":60832,"inlinee":28149,"source_line":287,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":60912,"parent_record_offset":60888,"ancestor_record_offset":60832,"inlinee":12394,"source_line":550,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61024,"parent_record_offset":61000,"ancestor_record_offset":60832,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":60936,"parent_record_offset":60912,"ancestor_record_offset":60832,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":60888,"parent_record_offset":60864,"ancestor_record_offset":60832,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61108,"parent_record_offset":61088,"ancestor_record_offset":60832,"inlinee":14042,"source_line":812,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":61048,"parent_record_offset":61024,"ancestor_record_offset":60832,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$1::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":501,"record_offset":61000,"parent_record_offset":60976,"ancestor_record_offset":60832,"inlinee":28151,"source_line":78,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$8::resolve
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":4380,"rva":14250896,"va":5382960016}
// 编译实例 RVAs: 0xd97390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rancor..Panic$u20$as$u20$rancor..Source$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33892840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382960016,"raw_hex":"e8e8290502","size":5,"soft_fail":false,"target_va":5416852944,"va":5382960099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b258b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14214316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382960016,"raw_hex":"e8ace4d800","size":5,"soft_fail":false,"target_va":5397174448,"va":5382960127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db30e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16891066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382960016,"raw_hex":"e8babc0101","size":5,"soft_fail":false,"target_va":5399851232,"va":5382960161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34096990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382960016,"raw_hex":"e85e470802","size":5,"soft_fail":false,"target_va":5417057056,"va":5382960061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1417f9a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t10888674","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5382960016,"raw_hex":"e9e225a600","size":5,"soft_fail":false,"target_va":5393848880,"va":5382960201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rkyv::impls::alloc::string::impl$0::resolve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/string.rs
//     evidence={"module":687,"record_offset":4508,"parent_record_offset":4380,"ancestor_record_offset":4380,"inlinee":42174,"source_line":17,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] rkyv::place::Place::from_field_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":687,"record_offset":5020,"parent_record_offset":4380,"ancestor_record_offset":4380,"inlinee":42155,"source_line":47,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$75::resolve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":687,"record_offset":5076,"parent_record_offset":4380,"ancestor_record_offset":4380,"inlinee":42154,"source_line":213,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":687,"record_offset":4860,"parent_record_offset":4824,"ancestor_record_offset":4380,"inlinee":42164,"source_line":2927,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":687,"record_offset":4624,"parent_record_offset":4572,"ancestor_record_offset":4380,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":687,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":4380,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":687,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":4380,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":687,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":4380,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::place::Place::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":687,"record_offset":4712,"parent_record_offset":4680,"ancestor_record_offset":4380,"inlinee":42147,"source_line":89,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::place::Place::write_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":687,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":4380,"inlinee":42148,"source_line":79,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::signed_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":687,"record_offset":4824,"parent_record_offset":4680,"ancestor_record_offset":4380,"inlinee":42163,"source_line":144,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::string::ArchivedString::resolve_from_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/string/mod.rs
//     evidence={"module":687,"record_offset":4540,"parent_record_offset":4508,"ancestor_record_offset":4380,"inlinee":42175,"source_line":59,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::string::repr::ArchivedStringRepr::emplace_inline
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/string/repr.rs
//     evidence={"module":687,"record_offset":4572,"parent_record_offset":4540,"ancestor_record_offset":4380,"inlinee":42176,"source_line":173,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::string::repr::ArchivedStringRepr::emplace_out_of_line
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/string/repr.rs
//     evidence={"module":687,"record_offset":4656,"parent_record_offset":4540,"ancestor_record_offset":4380,"inlinee":42181,"source_line":241,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::string::repr::ArchivedStringRepr::try_emplace_out_of_line
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/string/repr.rs
//     evidence={"module":687,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4380,"inlinee":42182,"source_line":200,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$14::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F65
// 首条位置: {"module":1651,"inlinee":81765}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$15::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F29
// 首条位置: {"module":1651,"inlinee":81705}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":9948,"parent_record_offset":9924,"ancestor_record_offset":9924,"inlinee":14306,"source_line":3793,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":10000,"parent_record_offset":9972,"ancestor_record_offset":9924,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":9924,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":9924,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":10048,"parent_record_offset":10024,"ancestor_record_offset":9924,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$16::resolve
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":3672,"rva":14250832,"va":5382959952}
// 编译实例 RVAs: 0xd97350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rancor..Panic$u20$as$u20$rancor..Source$GT$::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t33892970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382959952,"raw_hex":"e86a2a0502","size":5,"soft_fail":false,"target_va":5416852944,"va":5382959969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$0::resolve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":687,"record_offset":3796,"parent_record_offset":3672,"ancestor_record_offset":3672,"inlinee":42157,"source_line":19,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":687,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":3672,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$17::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":687,"record_offset":4060,"parent_record_offset":4032,"ancestor_record_offset":3672,"inlinee":42168,"source_line":319,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":687,"record_offset":4004,"parent_record_offset":3972,"ancestor_record_offset":3672,"inlinee":42164,"source_line":2927,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":687,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":3672,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":687,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3672,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::primitive::impl$0::resolve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/primitive.rs
//     evidence={"module":687,"record_offset":4224,"parent_record_offset":3856,"ancestor_record_offset":3672,"inlinee":42170,"source_line":269,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::place::Place::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":687,"record_offset":4100,"parent_record_offset":3940,"ancestor_record_offset":3672,"inlinee":42147,"source_line":89,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::place::Place::write_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":687,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":3672,"inlinee":42148,"source_line":79,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::try_emplace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":687,"record_offset":3940,"parent_record_offset":3912,"ancestor_record_offset":3672,"inlinee":42161,"source_line":173,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::emplace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":687,"record_offset":3888,"parent_record_offset":3856,"ancestor_record_offset":3672,"inlinee":42159,"source_line":410,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::try_emplace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":687,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":3672,"inlinee":42160,"source_line":395,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$19::from_isize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":687,"record_offset":4032,"parent_record_offset":3940,"ancestor_record_offset":3672,"inlinee":42166,"source_line":70,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::signed_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":687,"record_offset":3972,"parent_record_offset":3940,"ancestor_record_offset":3672,"inlinee":42163,"source_line":144,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::resolve_from_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":687,"record_offset":3856,"parent_record_offset":3820,"ancestor_record_offset":3672,"inlinee":42158,"source_line":79,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$18::check_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F42
// 首条位置: {"module":1666,"inlinee":28482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rkyv::vec::impl$19::check_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":117568,"parent_record_offset":117544,"ancestor_record_offset":117544,"inlinee":28271,"source_line":28,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::metadata::from_raw_parts
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\metadata.rs
//     evidence={"module":1666,"record_offset":117848,"parent_record_offset":117824,"ancestor_record_offset":117544,"inlinee":28281,"source_line":116,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1666,"record_offset":117724,"parent_record_offset":117700,"ancestor_record_offset":117544,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1666,"record_offset":117784,"parent_record_offset":117672,"ancestor_record_offset":117544,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":117748,"parent_record_offset":117724,"ancestor_record_offset":117544,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::slice_from_raw_parts
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":117824,"parent_record_offset":117592,"ancestor_record_offset":117544,"inlinee":28280,"source_line":1174,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":117648,"parent_record_offset":117624,"ancestor_record_offset":117544,"inlinee":28277,"source_line":325,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":117672,"parent_record_offset":117648,"ancestor_record_offset":117544,"inlinee":28278,"source_line":236,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":117700,"parent_record_offset":117672,"ancestor_record_offset":117544,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":117624,"parent_record_offset":117592,"ancestor_record_offset":117544,"inlinee":28275,"source_line":606,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":117592,"parent_record_offset":117568,"ancestor_record_offset":117544,"inlinee":28273,"source_line":304,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$19::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E01
// 首条位置: {"module":501,"inlinee":28161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$1::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":501,"record_offset":62636,"parent_record_offset":62616,"ancestor_record_offset":62616,"inlinee":28163,"source_line":27,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":62708,"parent_record_offset":62684,"ancestor_record_offset":62616,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":62756,"parent_record_offset":62732,"ancestor_record_offset":62616,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":62732,"parent_record_offset":62708,"ancestor_record_offset":62616,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":62684,"parent_record_offset":62660,"ancestor_record_offset":62616,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":62660,"parent_record_offset":62636,"ancestor_record_offset":62616,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":62848,"parent_record_offset":62824,"ancestor_record_offset":62616,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$14::serialize_unsized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":501,"record_offset":62824,"parent_record_offset":62800,"ancestor_record_offset":62616,"inlinee":28167,"source_line":288,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::serialize_from_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":501,"record_offset":62800,"parent_record_offset":62636,"ancestor_record_offset":62616,"inlinee":28165,"source_line":90,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$20::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F00
// 首条位置: {"module":1666,"inlinee":28416}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1666,"record_offset":119080,"parent_record_offset":118680,"ancestor_record_offset":118680,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$2::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":1666,"record_offset":118704,"parent_record_offset":118680,"ancestor_record_offset":118680,"inlinee":28517,"source_line":45,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1666,"record_offset":118800,"parent_record_offset":118704,"ancestor_record_offset":118680,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1666,"record_offset":118996,"parent_record_offset":118964,"ancestor_record_offset":118680,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":119020,"parent_record_offset":118996,"ancestor_record_offset":118680,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":118916,"parent_record_offset":118892,"ancestor_record_offset":118680,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":118940,"parent_record_offset":118916,"ancestor_record_offset":118680,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":118964,"parent_record_offset":118940,"ancestor_record_offset":118680,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":118892,"parent_record_offset":118868,"ancestor_record_offset":118680,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":118868,"parent_record_offset":118832,"ancestor_record_offset":118680,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":118744,"parent_record_offset":118704,"ancestor_record_offset":118680,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":118768,"parent_record_offset":118744,"ancestor_record_offset":118680,"inlinee":28519,"source_line":45,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$21::visit_asset_dependencies
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":5484,"rva":14251312,"va":5382960432}
// 编译实例 RVAs: 0xd97530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14238303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382960432,"raw_hex":"e8a1bd26ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5382960490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r15","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5382960432,"raw_hex":"41ffd7","size":3,"soft_fail":false,"va":5382960506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] turbosloth::asset::impl$6::visit_asset_dependencies
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":687,"record_offset":5624,"parent_record_offset":5484,"ancestor_record_offset":5484,"inlinee":42184,"source_line":149,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$13::visit_asset_dependencies
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":687,"record_offset":5712,"parent_record_offset":5624,"ancestor_record_offset":5484,"inlinee":45638,"source_line":22,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":687,"record_offset":5680,"parent_record_offset":5652,"ancestor_record_offset":5484,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":687,"record_offset":5652,"parent_record_offset":5624,"ancestor_record_offset":5484,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$15::visit_asset_dependencies
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":687,"record_offset":5732,"parent_record_offset":5712,"ancestor_record_offset":5484,"inlinee":42050,"source_line":267,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$22::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E3E
// 首条位置: {"module":501,"inlinee":28222}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":501,"record_offset":66716,"parent_record_offset":66656,"ancestor_record_offset":66656,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":66688,"parent_record_offset":66656,"ancestor_record_offset":66656,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67180,"parent_record_offset":67156,"ancestor_record_offset":66656,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67204,"parent_record_offset":67180,"ancestor_record_offset":66656,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67228,"parent_record_offset":67204,"ancestor_record_offset":66656,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67156,"parent_record_offset":67004,"ancestor_record_offset":66656,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66948,"parent_record_offset":66924,"ancestor_record_offset":66656,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67004,"parent_record_offset":66972,"ancestor_record_offset":66656,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66972,"parent_record_offset":66948,"ancestor_record_offset":66656,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67044,"parent_record_offset":67004,"ancestor_record_offset":66656,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":66896,"parent_record_offset":66836,"ancestor_record_offset":66656,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66868,"parent_record_offset":66836,"ancestor_record_offset":66656,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66924,"parent_record_offset":66896,"ancestor_record_offset":66656,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66812,"parent_record_offset":66788,"ancestor_record_offset":66656,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66836,"parent_record_offset":66812,"ancestor_record_offset":66656,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":501,"record_offset":66788,"parent_record_offset":66764,"ancestor_record_offset":66656,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":67068,"parent_record_offset":67044,"ancestor_record_offset":66656,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":67116,"parent_record_offset":67092,"ancestor_record_offset":66656,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":67092,"parent_record_offset":67068,"ancestor_record_offset":66656,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":67284,"parent_record_offset":66896,"ancestor_record_offset":66656,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":67308,"parent_record_offset":67284,"ancestor_record_offset":66656,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":501,"record_offset":66740,"parent_record_offset":66716,"ancestor_record_offset":66656,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":501,"record_offset":66764,"parent_record_offset":66740,"ancestor_record_offset":66656,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$23::hash
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6DFF
// 首条位置: {"module":501,"inlinee":28159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61600,"parent_record_offset":61456,"ancestor_record_offset":61456,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$19::hash
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":61488,"parent_record_offset":61456,"ancestor_record_offset":61456,"inlinee":28149,"source_line":287,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61624,"parent_record_offset":61600,"ancestor_record_offset":61456,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61536,"parent_record_offset":61512,"ancestor_record_offset":61456,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61648,"parent_record_offset":61624,"ancestor_record_offset":61456,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":61560,"parent_record_offset":61536,"ancestor_record_offset":61456,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":61672,"parent_record_offset":61648,"ancestor_record_offset":61456,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$1::hash
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":501,"record_offset":61512,"parent_record_offset":61488,"ancestor_record_offset":61456,"inlinee":28151,"source_line":78,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::TextureSourceImage::load
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":1856,"rva":14250208,"va":5382959328}
// 编译实例 RVAs: 0xd970e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$0::load::async_fn$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":501,"record_offset":1652,"rva":9405120,"va":5378114240}
// 编译实例 RVAs: 0x8f82c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2436511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e861d2daff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378115194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2435999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e861d4daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378114698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9392992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8a0ac70ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378115179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=3
//     target_aliases=66 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8808043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8959979ff","size":5,"soft_fail":false,"target_va":5369308864,"va":5378116902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8778488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8080d7aff","size":5,"soft_fail":false,"target_va":5369338848,"va":5378117331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0a260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1121325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e82d1c1100","size":5,"soft_fail":false,"target_va":5379236448,"va":5378115118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b0830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9143541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8f5848b00","size":5,"soft_fail":false,"target_va":5387257904,"va":5378114358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14170d010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14763359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e85f45e100","size":5,"soft_fail":false,"target_va":5392879632,"va":5378116268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14170e5e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14770866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8b262e100","size":5,"soft_fail":false,"target_va":5392885216,"va":5378114345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141711160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14780836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8a489e100","size":5,"soft_fail":false,"target_va":5392896352,"va":5378115511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417406c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14974360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8987de400","size":5,"soft_fail":false,"target_va":5393090240,"va":5378115875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd5290
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38653392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8d0cd4d02","size":5,"soft_fail":false,"target_va":5416768144,"va":5378114747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38940825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e899305202","size":5,"soft_fail":false,"target_va":5417057056,"va":5378116226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38504419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8e3874b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378116345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6678900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e874e96500","size":5,"soft_fail":false,"target_va":5384794864,"va":5378115959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38504887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8b7894b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378115902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t14967726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8ae63e400","size":5,"soft_fail":false,"target_va":5393082512,"va":5378114781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::source_image_format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4844544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e800ec4900","size":5,"soft_fail":false,"target_va":5382959360,"va":5378114811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_async_fn_resumed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38506255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e80f8f4b02","size":5,"soft_fail":false,"target_va":5416622656,"va":5378116396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_async_fn_resumed_panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38506299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e83b8f4b02","size":5,"soft_fail":false,"target_va":5416622688,"va":5378116384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$38::read_file::async_fn_env$0<ref$<std::path::PathBuf> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8fb2c0000","size":5,"soft_fail":false,"target_va":5378129584,"va":5378118064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$38::read_file_asset::async_fn_env$0<ref$<std::path::PathBuf> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e8332e0000","size":5,"soft_fail":false,"target_va":5378129760,"va":5378117928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::error::Error::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4974985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378114240,"raw_hex":"e889e94b00","size":5,"soft_fail":false,"target_va":5383090992,"va":5378116002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*24(%r9)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378114240,"raw_hex":"41ff5118","size":4,"soft_fail":false,"va":5378114559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":5656,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":501,"record_offset":4408,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":4492,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":5052,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":5080,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$38::read_file::async_fn$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":1804,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":28093,"source_line":706,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":1652,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":1652,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1652,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":1652,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":1652,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":1652,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":2424,"parent_record_offset":2392,"ancestor_record_offset":1652,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":2776,"parent_record_offset":2644,"ancestor_record_offset":1652,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":2392,"parent_record_offset":2348,"ancestor_record_offset":1652,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":2348,"parent_record_offset":2312,"ancestor_record_offset":1652,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":2644,"parent_record_offset":2616,"ancestor_record_offset":1652,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":501,"record_offset":3648,"parent_record_offset":1804,"ancestor_record_offset":1652,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":501,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":1652,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":1652,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":1652,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":3316,"parent_record_offset":3292,"ancestor_record_offset":1652,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":5836,"parent_record_offset":5800,"ancestor_record_offset":1652,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":5800,"parent_record_offset":5772,"ancestor_record_offset":1652,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":3292,"parent_record_offset":3268,"ancestor_record_offset":1652,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":5876,"parent_record_offset":5836,"ancestor_record_offset":1652,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":5684,"parent_record_offset":5656,"ancestor_record_offset":1652,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":5712,"parent_record_offset":5684,"ancestor_record_offset":1652,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":4088,"parent_record_offset":4052,"ancestor_record_offset":1652,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":2312,"parent_record_offset":2276,"ancestor_record_offset":1652,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":3620,"parent_record_offset":1804,"ancestor_record_offset":1652,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":1652,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":5744,"parent_record_offset":5712,"ancestor_record_offset":1652,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":2676,"parent_record_offset":2644,"ancestor_record_offset":1652,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":1652,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":501,"record_offset":2140,"parent_record_offset":2028,"ancestor_record_offset":1652,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":501,"record_offset":3732,"parent_record_offset":1804,"ancestor_record_offset":1652,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":5948,"parent_record_offset":5924,"ancestor_record_offset":1652,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":2740,"parent_record_offset":2704,"ancestor_record_offset":1652,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":1652,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":1652,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":501,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":1652,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":501,"record_offset":2168,"parent_record_offset":2140,"ancestor_record_offset":1652,"inlinee":5375,"source_line":1428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":501,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":1652,"inlinee":5376,"source_line":1409,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":6116,"parent_record_offset":5712,"ancestor_record_offset":1652,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":1652,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":2544,"parent_record_offset":2348,"ancestor_record_offset":1652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":4120,"parent_record_offset":4088,"ancestor_record_offset":1652,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":3520,"parent_record_offset":1928,"ancestor_record_offset":1652,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":2228,"parent_record_offset":2028,"ancestor_record_offset":1652,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":3592,"parent_record_offset":1804,"ancestor_record_offset":1652,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":4148,"parent_record_offset":4088,"ancestor_record_offset":1652,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":4180,"parent_record_offset":4052,"ancestor_record_offset":1652,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$38::read_file_asset::async_fn$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":1928,"parent_record_offset":1804,"ancestor_record_offset":1652,"inlinee":28095,"source_line":696,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$38::require_file_asset::async_fn$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":2028,"parent_record_offset":1928,"ancestor_record_offset":1652,"inlinee":28097,"source_line":683,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$38::require_file_asset::async_fn$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":2276,"parent_record_offset":2228,"ancestor_record_offset":1652,"inlinee":28099,"source_line":684,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$0::load::async_fn_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":501,"record_offset":58724,"rva":9422864,"va":5378131984}
// 编译实例 RVAs: 0x8fc810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2453377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378131984,"raw_hex":"e87f90daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378132076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14950153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378131984,"raw_hex":"e8091fe400","size":5,"soft_fail":false,"target_va":5393082512,"va":5378132354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t14950403","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378131984,"raw_hex":"e90320e400","size":5,"soft_fail":false,"target_va":5393082512,"va":5378132104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378131984,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378132052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":58892,"parent_record_offset":58724,"ancestor_record_offset":58724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":59096,"parent_record_offset":59072,"ancestor_record_offset":58724,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":59120,"parent_record_offset":59096,"ancestor_record_offset":58724,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":59144,"parent_record_offset":59120,"ancestor_record_offset":58724,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":59072,"parent_record_offset":58948,"ancestor_record_offset":58724,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":58948,"parent_record_offset":58920,"ancestor_record_offset":58724,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":59528,"parent_record_offset":59492,"ancestor_record_offset":58724,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":59492,"parent_record_offset":59468,"ancestor_record_offset":58724,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":58976,"parent_record_offset":58948,"ancestor_record_offset":58724,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":59036,"parent_record_offset":59004,"ancestor_record_offset":58724,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":59004,"parent_record_offset":58976,"ancestor_record_offset":58724,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":58920,"parent_record_offset":58892,"ancestor_record_offset":58724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":59560,"parent_record_offset":59528,"ancestor_record_offset":58724,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":59588,"parent_record_offset":59528,"ancestor_record_offset":58724,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":59620,"parent_record_offset":59492,"ancestor_record_offset":58724,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$0::load::async_fn$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112E2
// 首条位置: {"module":1253,"inlinee":70370}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":41852,"parent_record_offset":41832,"ancestor_record_offset":41832,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":41900,"parent_record_offset":41876,"ancestor_record_offset":41832,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":41876,"parent_record_offset":41852,"ancestor_record_offset":41832,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$24::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E3A
// 首条位置: {"module":501,"inlinee":28218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66616,"parent_record_offset":66584,"ancestor_record_offset":66584,"inlinee":28220,"source_line":136,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67180,"parent_record_offset":67156,"ancestor_record_offset":66584,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67204,"parent_record_offset":67180,"ancestor_record_offset":66584,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67576,"parent_record_offset":67552,"ancestor_record_offset":66584,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67600,"parent_record_offset":67576,"ancestor_record_offset":66584,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67228,"parent_record_offset":67204,"ancestor_record_offset":66584,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67624,"parent_record_offset":67600,"ancestor_record_offset":66584,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67156,"parent_record_offset":67004,"ancestor_record_offset":66584,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67552,"parent_record_offset":67496,"ancestor_record_offset":66584,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66948,"parent_record_offset":66924,"ancestor_record_offset":66584,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67524,"parent_record_offset":67496,"ancestor_record_offset":66584,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67496,"parent_record_offset":67472,"ancestor_record_offset":66584,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67004,"parent_record_offset":66972,"ancestor_record_offset":66584,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66972,"parent_record_offset":66948,"ancestor_record_offset":66584,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67472,"parent_record_offset":67448,"ancestor_record_offset":66584,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67044,"parent_record_offset":67004,"ancestor_record_offset":66584,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":66896,"parent_record_offset":66836,"ancestor_record_offset":66584,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66868,"parent_record_offset":66836,"ancestor_record_offset":66584,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66924,"parent_record_offset":66896,"ancestor_record_offset":66584,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66812,"parent_record_offset":66788,"ancestor_record_offset":66584,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66836,"parent_record_offset":66812,"ancestor_record_offset":66584,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":501,"record_offset":66788,"parent_record_offset":66764,"ancestor_record_offset":66584,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$62::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":67372,"parent_record_offset":66616,"ancestor_record_offset":66584,"inlinee":28226,"source_line":116,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66656,"parent_record_offset":66616,"ancestor_record_offset":66584,"inlinee":28222,"source_line":39,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":67068,"parent_record_offset":67044,"ancestor_record_offset":66584,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":67116,"parent_record_offset":67092,"ancestor_record_offset":66584,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":67092,"parent_record_offset":67068,"ancestor_record_offset":66584,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":67284,"parent_record_offset":66896,"ancestor_record_offset":66584,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":67308,"parent_record_offset":67284,"ancestor_record_offset":66584,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":67400,"parent_record_offset":66616,"ancestor_record_offset":66584,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":501,"record_offset":66740,"parent_record_offset":66716,"ancestor_record_offset":66584,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":501,"record_offset":66716,"parent_record_offset":66656,"ancestor_record_offset":66584,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":501,"record_offset":66764,"parent_record_offset":66740,"ancestor_record_offset":66584,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":66688,"parent_record_offset":66656,"ancestor_record_offset":66584,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$25::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F39
// 首条位置: {"module":1651,"inlinee":81721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$29::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":42148,"parent_record_offset":42124,"ancestor_record_offset":42124,"inlinee":81723,"source_line":136,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1651,"record_offset":42180,"parent_record_offset":42148,"ancestor_record_offset":42124,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$23::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":42296,"parent_record_offset":42148,"ancestor_record_offset":42124,"inlinee":28159,"source_line":39,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42352,"parent_record_offset":42328,"ancestor_record_offset":42124,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42228,"parent_record_offset":42204,"ancestor_record_offset":42124,"inlinee":12394,"source_line":550,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42488,"parent_record_offset":42464,"ancestor_record_offset":42124,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42256,"parent_record_offset":42228,"ancestor_record_offset":42124,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42376,"parent_record_offset":42352,"ancestor_record_offset":42124,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42328,"parent_record_offset":42296,"ancestor_record_offset":42124,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42204,"parent_record_offset":42180,"ancestor_record_offset":42124,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":42512,"parent_record_offset":42488,"ancestor_record_offset":42124,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":42400,"parent_record_offset":42376,"ancestor_record_offset":42124,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$19::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1651,"record_offset":42440,"parent_record_offset":42296,"ancestor_record_offset":42124,"inlinee":28149,"source_line":287,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$1::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1651,"record_offset":42464,"parent_record_offset":42440,"ancestor_record_offset":42124,"inlinee":28151,"source_line":78,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$1::run::async_fn$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E53
// 首条位置: {"module":501,"inlinee":28243}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::future::into_future::impl$0::into_future
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\into_future.rs
//     evidence={"module":501,"record_offset":80472,"parent_record_offset":80344,"ancestor_record_offset":80344,"inlinee":20661,"source_line":142,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":80500,"parent_record_offset":80344,"ancestor_record_offset":80344,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":82500,"parent_record_offset":80344,"ancestor_record_offset":80344,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":80896,"parent_record_offset":80872,"ancestor_record_offset":80344,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":80920,"parent_record_offset":80896,"ancestor_record_offset":80344,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":80944,"parent_record_offset":80920,"ancestor_record_offset":80344,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":80872,"parent_record_offset":80740,"ancestor_record_offset":80344,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":80740,"parent_record_offset":80712,"ancestor_record_offset":80344,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":81868,"parent_record_offset":81840,"ancestor_record_offset":80344,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":81840,"parent_record_offset":81816,"ancestor_record_offset":80344,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":81816,"parent_record_offset":81792,"ancestor_record_offset":80344,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":81336,"parent_record_offset":81300,"ancestor_record_offset":80344,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":81300,"parent_record_offset":81272,"ancestor_record_offset":80344,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":80772,"parent_record_offset":80740,"ancestor_record_offset":80344,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":80836,"parent_record_offset":80800,"ancestor_record_offset":80344,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":80800,"parent_record_offset":80772,"ancestor_record_offset":80344,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":80560,"parent_record_offset":80500,"ancestor_record_offset":80344,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":81368,"parent_record_offset":81336,"ancestor_record_offset":80344,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":81396,"parent_record_offset":81336,"ancestor_record_offset":80344,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":81428,"parent_record_offset":81300,"ancestor_record_offset":80344,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$1::run::async_fn_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":501,"record_offset":50480,"rva":9421536,"va":5378130656}
// 编译实例 RVAs: 0x8fc2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2452131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e85d95daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378130830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8821927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e8596379ff","size":5,"soft_fail":false,"target_va":5369308864,"va":5378130786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t605160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e8e83b0900","size":5,"soft_fail":false,"target_va":5378736160,"va":5378130995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140992780
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t614980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e844620900","size":5,"soft_fail":false,"target_va":5378746240,"va":5378131255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14951529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e86924e400","size":5,"soft_fail":false,"target_va":5393082512,"va":5378130978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-2452366","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378130656,"raw_hex":"e97294daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378131065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t605273","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378130656,"raw_hex":"e9593c0900","size":5,"soft_fail":false,"target_va":5378736160,"va":5378130882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378130656,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378130921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":50648,"parent_record_offset":50480,"ancestor_record_offset":50480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":51040,"parent_record_offset":51016,"ancestor_record_offset":50480,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":51064,"parent_record_offset":51040,"ancestor_record_offset":50480,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":51088,"parent_record_offset":51064,"ancestor_record_offset":50480,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":51016,"parent_record_offset":50884,"ancestor_record_offset":50480,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":50884,"parent_record_offset":50856,"ancestor_record_offset":50480,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":52012,"parent_record_offset":51984,"ancestor_record_offset":50480,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":51984,"parent_record_offset":51960,"ancestor_record_offset":50480,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":51960,"parent_record_offset":51936,"ancestor_record_offset":50480,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":51480,"parent_record_offset":51444,"ancestor_record_offset":50480,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":51444,"parent_record_offset":51416,"ancestor_record_offset":50480,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":50916,"parent_record_offset":50884,"ancestor_record_offset":50480,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":50980,"parent_record_offset":50944,"ancestor_record_offset":50480,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":50944,"parent_record_offset":50916,"ancestor_record_offset":50480,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":50704,"parent_record_offset":50648,"ancestor_record_offset":50480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":51512,"parent_record_offset":51480,"ancestor_record_offset":50480,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":51540,"parent_record_offset":51480,"ancestor_record_offset":50480,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":51572,"parent_record_offset":51444,"ancestor_record_offset":50480,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$3::run::async_fn_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":501,"record_offset":47644,"rva":9421536,"va":5378130656}
// 编译实例 RVAs: 0x8fc2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2452131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e85d95daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378130830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8821927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e8596379ff","size":5,"soft_fail":false,"target_va":5369308864,"va":5378130786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t605160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e8e83b0900","size":5,"soft_fail":false,"target_va":5378736160,"va":5378130995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140992780
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t614980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e844620900","size":5,"soft_fail":false,"target_va":5378746240,"va":5378131255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14951529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130656,"raw_hex":"e86924e400","size":5,"soft_fail":false,"target_va":5393082512,"va":5378130978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-2452366","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378130656,"raw_hex":"e97294daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378131065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t605273","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378130656,"raw_hex":"e9593c0900","size":5,"soft_fail":false,"target_va":5378736160,"va":5378130882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378130656,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378130921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":47812,"parent_record_offset":47644,"ancestor_record_offset":47644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":48204,"parent_record_offset":48180,"ancestor_record_offset":47644,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":48228,"parent_record_offset":48204,"ancestor_record_offset":47644,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":48252,"parent_record_offset":48228,"ancestor_record_offset":47644,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":48180,"parent_record_offset":48048,"ancestor_record_offset":47644,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":48048,"parent_record_offset":48020,"ancestor_record_offset":47644,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":49176,"parent_record_offset":49148,"ancestor_record_offset":47644,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":49148,"parent_record_offset":49124,"ancestor_record_offset":47644,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":49124,"parent_record_offset":49100,"ancestor_record_offset":47644,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":48644,"parent_record_offset":48608,"ancestor_record_offset":47644,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":48608,"parent_record_offset":48580,"ancestor_record_offset":47644,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":48080,"parent_record_offset":48048,"ancestor_record_offset":47644,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":48144,"parent_record_offset":48108,"ancestor_record_offset":47644,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":48108,"parent_record_offset":48080,"ancestor_record_offset":47644,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":47868,"parent_record_offset":47812,"ancestor_record_offset":47644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":48676,"parent_record_offset":48644,"ancestor_record_offset":47644,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":48704,"parent_record_offset":48644,"ancestor_record_offset":47644,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":48736,"parent_record_offset":48608,"ancestor_record_offset":47644,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$26::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E36
// 首条位置: {"module":501,"inlinee":28214}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$32::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66256,"parent_record_offset":66228,"ancestor_record_offset":66228,"inlinee":28216,"source_line":150,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66468,"parent_record_offset":66444,"ancestor_record_offset":66228,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66492,"parent_record_offset":66468,"ancestor_record_offset":66228,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66516,"parent_record_offset":66492,"ancestor_record_offset":66228,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66444,"parent_record_offset":66388,"ancestor_record_offset":66228,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66416,"parent_record_offset":66388,"ancestor_record_offset":66228,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66388,"parent_record_offset":66364,"ancestor_record_offset":66228,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66364,"parent_record_offset":66340,"ancestor_record_offset":66228,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":66292,"parent_record_offset":66256,"ancestor_record_offset":66228,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$27::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F35
// 首条位置: {"module":1651,"inlinee":81717}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$33::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":41828,"parent_record_offset":41804,"ancestor_record_offset":41804,"inlinee":81719,"source_line":150,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1651,"record_offset":41856,"parent_record_offset":41828,"ancestor_record_offset":41804,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":41972,"parent_record_offset":41828,"ancestor_record_offset":41804,"inlinee":14306,"source_line":3793,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42024,"parent_record_offset":41996,"ancestor_record_offset":41804,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41904,"parent_record_offset":41880,"ancestor_record_offset":41804,"inlinee":12394,"source_line":550,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41932,"parent_record_offset":41904,"ancestor_record_offset":41804,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42048,"parent_record_offset":42024,"ancestor_record_offset":41804,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41880,"parent_record_offset":41856,"ancestor_record_offset":41804,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41996,"parent_record_offset":41972,"ancestor_record_offset":41804,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":42072,"parent_record_offset":42048,"ancestor_record_offset":41804,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$2::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":5160,"rva":14251088,"va":5382960208}
// 编译实例 RVAs: 0xd97450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$2::run::async_fn$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E51
// 首条位置: {"module":501,"inlinee":28241}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77372,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":76876,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":79108,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77260,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":501,"record_offset":79164,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":77672,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":28230,"source_line":143,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":501,"record_offset":77172,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":79328,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":79888,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":80312,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":501,"record_offset":77616,"parent_record_offset":76660,"ancestor_record_offset":76660,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":77064,"parent_record_offset":77040,"ancestor_record_offset":76660,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":77088,"parent_record_offset":77064,"ancestor_record_offset":76660,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":78708,"parent_record_offset":78684,"ancestor_record_offset":76660,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":78732,"parent_record_offset":78708,"ancestor_record_offset":76660,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":77112,"parent_record_offset":77088,"ancestor_record_offset":76660,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":78756,"parent_record_offset":78732,"ancestor_record_offset":76660,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":77040,"parent_record_offset":77008,"ancestor_record_offset":76660,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":78684,"parent_record_offset":78628,"ancestor_record_offset":76660,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":79384,"parent_record_offset":79356,"ancestor_record_offset":76660,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":77492,"parent_record_offset":77460,"ancestor_record_offset":76660,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":76948,"parent_record_offset":76912,"ancestor_record_offset":76660,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":78656,"parent_record_offset":78628,"ancestor_record_offset":76660,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":78628,"parent_record_offset":78604,"ancestor_record_offset":76660,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":77540,"parent_record_offset":77516,"ancestor_record_offset":76660,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":77516,"parent_record_offset":77492,"ancestor_record_offset":76660,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":77008,"parent_record_offset":76976,"ancestor_record_offset":76660,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":76976,"parent_record_offset":76948,"ancestor_record_offset":76660,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":78604,"parent_record_offset":78580,"ancestor_record_offset":76660,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":78176,"parent_record_offset":78136,"ancestor_record_offset":76660,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":77996,"parent_record_offset":77928,"ancestor_record_offset":76660,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":80176,"parent_record_offset":80152,"ancestor_record_offset":76660,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77460,"parent_record_offset":77408,"ancestor_record_offset":76660,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77968,"parent_record_offset":77928,"ancestor_record_offset":76660,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77408,"parent_record_offset":77372,"ancestor_record_offset":76660,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":76912,"parent_record_offset":76876,"ancestor_record_offset":76660,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77900,"parent_record_offset":77872,"ancestor_record_offset":76660,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77928,"parent_record_offset":77900,"ancestor_record_offset":76660,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":501,"record_offset":77872,"parent_record_offset":77844,"ancestor_record_offset":76660,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$62::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":78504,"parent_record_offset":77672,"ancestor_record_offset":76660,"inlinee":28226,"source_line":116,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":77724,"parent_record_offset":77672,"ancestor_record_offset":76660,"inlinee":28222,"source_line":39,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":79416,"parent_record_offset":79384,"ancestor_record_offset":76660,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":78200,"parent_record_offset":78176,"ancestor_record_offset":76660,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":78248,"parent_record_offset":78224,"ancestor_record_offset":76660,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":79480,"parent_record_offset":79444,"ancestor_record_offset":76660,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":79444,"parent_record_offset":79416,"ancestor_record_offset":76660,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":78224,"parent_record_offset":78200,"ancestor_record_offset":76660,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":501,"record_offset":77200,"parent_record_offset":77172,"ancestor_record_offset":76660,"inlinee":5375,"source_line":1428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":501,"record_offset":77224,"parent_record_offset":77200,"ancestor_record_offset":76660,"inlinee":5376,"source_line":1409,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":78416,"parent_record_offset":77996,"ancestor_record_offset":76660,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":78440,"parent_record_offset":78416,"ancestor_record_offset":76660,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":78532,"parent_record_offset":77672,"ancestor_record_offset":76660,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":501,"record_offset":77332,"parent_record_offset":77308,"ancestor_record_offset":76660,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":79240,"parent_record_offset":79164,"ancestor_record_offset":76660,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":79268,"parent_record_offset":79164,"ancestor_record_offset":76660,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":77640,"parent_record_offset":77616,"ancestor_record_offset":76660,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":79292,"parent_record_offset":79268,"ancestor_record_offset":76660,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":77580,"parent_record_offset":77408,"ancestor_record_offset":76660,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":501,"record_offset":77284,"parent_record_offset":77260,"ancestor_record_offset":76660,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":501,"record_offset":77308,"parent_record_offset":77284,"ancestor_record_offset":76660,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":80212,"parent_record_offset":80176,"ancestor_record_offset":76660,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":80236,"parent_record_offset":80212,"ancestor_record_offset":76660,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":80268,"parent_record_offset":80176,"ancestor_record_offset":76660,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":501,"record_offset":77816,"parent_record_offset":77788,"ancestor_record_offset":76660,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":501,"record_offset":77788,"parent_record_offset":77724,"ancestor_record_offset":76660,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":501,"record_offset":77844,"parent_record_offset":77816,"ancestor_record_offset":76660,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":77760,"parent_record_offset":77724,"ancestor_record_offset":76660,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$2::run::async_fn_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":501,"record_offset":46008,"rva":9420816,"va":5378129936}
// 编译实例 RVAs: 0x8fc010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2451341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378129936,"raw_hex":"e87398daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378130040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093970
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8816413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378129936,"raw_hex":"e8e37879ff","size":5,"soft_fail":false,"target_va":5369313648,"va":5378130056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e840
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t468898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378129936,"raw_hex":"e8a2270700","size":5,"soft_fail":false,"target_va":5378598976,"va":5378130073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t606056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378129936,"raw_hex":"e8683f0900","size":5,"soft_fail":false,"target_va":5378736160,"va":5378130099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409925d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t615676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378129936,"raw_hex":"e8fc640900","size":5,"soft_fail":false,"target_va":5378745808,"va":5378130127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t606009","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378129936,"raw_hex":"e9393f0900","size":5,"soft_fail":false,"target_va":5378736160,"va":5378130146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1409925d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t615685","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378129936,"raw_hex":"e905650900","size":5,"soft_fail":false,"target_va":5378745808,"va":5378130118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378129936,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378130016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":46176,"parent_record_offset":46008,"ancestor_record_offset":46008,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":46380,"parent_record_offset":46356,"ancestor_record_offset":46008,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":46404,"parent_record_offset":46380,"ancestor_record_offset":46008,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":46428,"parent_record_offset":46404,"ancestor_record_offset":46008,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":46356,"parent_record_offset":46232,"ancestor_record_offset":46008,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":46232,"parent_record_offset":46204,"ancestor_record_offset":46008,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":46260,"parent_record_offset":46232,"ancestor_record_offset":46008,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":46320,"parent_record_offset":46288,"ancestor_record_offset":46008,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":46288,"parent_record_offset":46260,"ancestor_record_offset":46008,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":46204,"parent_record_offset":46176,"ancestor_record_offset":46008,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$28::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E3C
// 首条位置: {"module":501,"inlinee":28220}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::desc::impl$62::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":67372,"parent_record_offset":66616,"ancestor_record_offset":66616,"inlinee":28226,"source_line":116,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66656,"parent_record_offset":66616,"ancestor_record_offset":66616,"inlinee":28222,"source_line":39,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":67400,"parent_record_offset":66616,"ancestor_record_offset":66616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67180,"parent_record_offset":67156,"ancestor_record_offset":66616,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67204,"parent_record_offset":67180,"ancestor_record_offset":66616,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67576,"parent_record_offset":67552,"ancestor_record_offset":66616,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67600,"parent_record_offset":67576,"ancestor_record_offset":66616,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67228,"parent_record_offset":67204,"ancestor_record_offset":66616,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67624,"parent_record_offset":67600,"ancestor_record_offset":66616,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67156,"parent_record_offset":67004,"ancestor_record_offset":66616,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67552,"parent_record_offset":67496,"ancestor_record_offset":66616,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66948,"parent_record_offset":66924,"ancestor_record_offset":66616,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67524,"parent_record_offset":67496,"ancestor_record_offset":66616,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67496,"parent_record_offset":67472,"ancestor_record_offset":66616,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67004,"parent_record_offset":66972,"ancestor_record_offset":66616,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66972,"parent_record_offset":66948,"ancestor_record_offset":66616,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67472,"parent_record_offset":67448,"ancestor_record_offset":66616,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67044,"parent_record_offset":67004,"ancestor_record_offset":66616,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":66896,"parent_record_offset":66836,"ancestor_record_offset":66616,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66868,"parent_record_offset":66836,"ancestor_record_offset":66616,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66924,"parent_record_offset":66896,"ancestor_record_offset":66616,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66812,"parent_record_offset":66788,"ancestor_record_offset":66616,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66836,"parent_record_offset":66812,"ancestor_record_offset":66616,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":501,"record_offset":66788,"parent_record_offset":66764,"ancestor_record_offset":66616,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":67068,"parent_record_offset":67044,"ancestor_record_offset":66616,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":67116,"parent_record_offset":67092,"ancestor_record_offset":66616,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":67092,"parent_record_offset":67068,"ancestor_record_offset":66616,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":67284,"parent_record_offset":66896,"ancestor_record_offset":66616,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":67308,"parent_record_offset":67284,"ancestor_record_offset":66616,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":67424,"parent_record_offset":67400,"ancestor_record_offset":66616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":501,"record_offset":66740,"parent_record_offset":66716,"ancestor_record_offset":66616,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":501,"record_offset":66716,"parent_record_offset":66656,"ancestor_record_offset":66616,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":501,"record_offset":66764,"parent_record_offset":66740,"ancestor_record_offset":66616,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":66688,"parent_record_offset":66656,"ancestor_record_offset":66616,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$29::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F3B
// 首条位置: {"module":1651,"inlinee":81723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1651,"record_offset":42180,"parent_record_offset":42148,"ancestor_record_offset":42148,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$23::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":42296,"parent_record_offset":42148,"ancestor_record_offset":42148,"inlinee":28159,"source_line":39,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42352,"parent_record_offset":42328,"ancestor_record_offset":42148,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42228,"parent_record_offset":42204,"ancestor_record_offset":42148,"inlinee":12394,"source_line":550,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42488,"parent_record_offset":42464,"ancestor_record_offset":42148,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42256,"parent_record_offset":42228,"ancestor_record_offset":42148,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42376,"parent_record_offset":42352,"ancestor_record_offset":42148,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42328,"parent_record_offset":42296,"ancestor_record_offset":42148,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42204,"parent_record_offset":42180,"ancestor_record_offset":42148,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":42512,"parent_record_offset":42488,"ancestor_record_offset":42148,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":42400,"parent_record_offset":42376,"ancestor_record_offset":42148,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$19::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1651,"record_offset":42440,"parent_record_offset":42296,"ancestor_record_offset":42148,"inlinee":28149,"source_line":287,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$1::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1651,"record_offset":42464,"parent_record_offset":42440,"ancestor_record_offset":42148,"inlinee":28151,"source_line":78,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$30::clone
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E46
// 首条位置: {"module":501,"inlinee":28230}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::desc::impl$62::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":68900,"parent_record_offset":68156,"ancestor_record_offset":68156,"inlinee":28226,"source_line":116,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$22::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":68196,"parent_record_offset":68156,"ancestor_record_offset":68156,"inlinee":28222,"source_line":39,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":68928,"parent_record_offset":68156,"ancestor_record_offset":68156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":68708,"parent_record_offset":68684,"ancestor_record_offset":68156,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":68732,"parent_record_offset":68708,"ancestor_record_offset":68156,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":69104,"parent_record_offset":69080,"ancestor_record_offset":68156,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":69128,"parent_record_offset":69104,"ancestor_record_offset":68156,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":68756,"parent_record_offset":68732,"ancestor_record_offset":68156,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":69152,"parent_record_offset":69128,"ancestor_record_offset":68156,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":68684,"parent_record_offset":68536,"ancestor_record_offset":68156,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":69080,"parent_record_offset":69024,"ancestor_record_offset":68156,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":68480,"parent_record_offset":68456,"ancestor_record_offset":68156,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":69052,"parent_record_offset":69024,"ancestor_record_offset":68156,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":69024,"parent_record_offset":69000,"ancestor_record_offset":68156,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":68536,"parent_record_offset":68504,"ancestor_record_offset":68156,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":68504,"parent_record_offset":68480,"ancestor_record_offset":68156,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":69000,"parent_record_offset":68976,"ancestor_record_offset":68156,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":68572,"parent_record_offset":68536,"ancestor_record_offset":68156,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":68428,"parent_record_offset":68368,"ancestor_record_offset":68156,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":68400,"parent_record_offset":68368,"ancestor_record_offset":68156,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":68456,"parent_record_offset":68428,"ancestor_record_offset":68156,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":68344,"parent_record_offset":68324,"ancestor_record_offset":68156,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":68368,"parent_record_offset":68344,"ancestor_record_offset":68156,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":501,"record_offset":68324,"parent_record_offset":68300,"ancestor_record_offset":68156,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":68596,"parent_record_offset":68572,"ancestor_record_offset":68156,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":68644,"parent_record_offset":68620,"ancestor_record_offset":68156,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":68620,"parent_record_offset":68596,"ancestor_record_offset":68156,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":68812,"parent_record_offset":68428,"ancestor_record_offset":68156,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":68836,"parent_record_offset":68812,"ancestor_record_offset":68156,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":68952,"parent_record_offset":68928,"ancestor_record_offset":68156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":501,"record_offset":68276,"parent_record_offset":68252,"ancestor_record_offset":68156,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":501,"record_offset":68252,"parent_record_offset":68196,"ancestor_record_offset":68156,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":501,"record_offset":68300,"parent_record_offset":68276,"ancestor_record_offset":68156,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":68224,"parent_record_offset":68196,"ancestor_record_offset":68156,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$31::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6DFD
// 首条位置: {"module":501,"inlinee":28157}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":501,"record_offset":61716,"parent_record_offset":61420,"ancestor_record_offset":61420,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$23::hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":61456,"parent_record_offset":61420,"ancestor_record_offset":61420,"inlinee":28159,"source_line":39,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61624,"parent_record_offset":61600,"ancestor_record_offset":61420,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61764,"parent_record_offset":61740,"ancestor_record_offset":61420,"inlinee":12394,"source_line":550,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61536,"parent_record_offset":61512,"ancestor_record_offset":61420,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61788,"parent_record_offset":61764,"ancestor_record_offset":61420,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61648,"parent_record_offset":61624,"ancestor_record_offset":61420,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61600,"parent_record_offset":61456,"ancestor_record_offset":61420,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":501,"record_offset":61740,"parent_record_offset":61716,"ancestor_record_offset":61420,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":61560,"parent_record_offset":61536,"ancestor_record_offset":61420,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":61672,"parent_record_offset":61648,"ancestor_record_offset":61420,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$19::hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":61488,"parent_record_offset":61456,"ancestor_record_offset":61420,"inlinee":28149,"source_line":287,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$1::hash
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":501,"record_offset":61512,"parent_record_offset":61488,"ancestor_record_offset":61420,"inlinee":28151,"source_line":78,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$32::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E38
// 首条位置: {"module":501,"inlinee":28216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":66292,"parent_record_offset":66256,"ancestor_record_offset":66256,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66468,"parent_record_offset":66444,"ancestor_record_offset":66256,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66492,"parent_record_offset":66468,"ancestor_record_offset":66256,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66516,"parent_record_offset":66492,"ancestor_record_offset":66256,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66444,"parent_record_offset":66388,"ancestor_record_offset":66256,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66416,"parent_record_offset":66388,"ancestor_record_offset":66256,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66388,"parent_record_offset":66364,"ancestor_record_offset":66256,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66364,"parent_record_offset":66340,"ancestor_record_offset":66256,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":66316,"parent_record_offset":66292,"ancestor_record_offset":66256,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$33::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F37
// 首条位置: {"module":1651,"inlinee":81719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1651,"record_offset":41856,"parent_record_offset":41828,"ancestor_record_offset":41828,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":41972,"parent_record_offset":41828,"ancestor_record_offset":41828,"inlinee":14306,"source_line":3793,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42024,"parent_record_offset":41996,"ancestor_record_offset":41828,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41904,"parent_record_offset":41880,"ancestor_record_offset":41828,"inlinee":12394,"source_line":550,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41932,"parent_record_offset":41904,"ancestor_record_offset":41828,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42048,"parent_record_offset":42024,"ancestor_record_offset":41828,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41880,"parent_record_offset":41856,"ancestor_record_offset":41828,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41996,"parent_record_offset":41972,"ancestor_record_offset":41828,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":42072,"parent_record_offset":42048,"ancestor_record_offset":41828,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$34::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E44
// 首条位置: {"module":501,"inlinee":28228}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":68156,"parent_record_offset":68120,"ancestor_record_offset":68120,"inlinee":28230,"source_line":143,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":68708,"parent_record_offset":68684,"ancestor_record_offset":68120,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":68732,"parent_record_offset":68708,"ancestor_record_offset":68120,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":69104,"parent_record_offset":69080,"ancestor_record_offset":68120,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":69128,"parent_record_offset":69104,"ancestor_record_offset":68120,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":68756,"parent_record_offset":68732,"ancestor_record_offset":68120,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":69152,"parent_record_offset":69128,"ancestor_record_offset":68120,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":68684,"parent_record_offset":68536,"ancestor_record_offset":68120,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":69080,"parent_record_offset":69024,"ancestor_record_offset":68120,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":68480,"parent_record_offset":68456,"ancestor_record_offset":68120,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":69052,"parent_record_offset":69024,"ancestor_record_offset":68120,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":69024,"parent_record_offset":69000,"ancestor_record_offset":68120,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":68536,"parent_record_offset":68504,"ancestor_record_offset":68120,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":68504,"parent_record_offset":68480,"ancestor_record_offset":68120,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":69000,"parent_record_offset":68976,"ancestor_record_offset":68120,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":68572,"parent_record_offset":68536,"ancestor_record_offset":68120,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":68428,"parent_record_offset":68368,"ancestor_record_offset":68120,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":68400,"parent_record_offset":68368,"ancestor_record_offset":68120,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":68456,"parent_record_offset":68428,"ancestor_record_offset":68120,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":68344,"parent_record_offset":68324,"ancestor_record_offset":68120,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":68368,"parent_record_offset":68344,"ancestor_record_offset":68120,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":501,"record_offset":68324,"parent_record_offset":68300,"ancestor_record_offset":68120,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$62::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":68900,"parent_record_offset":68156,"ancestor_record_offset":68120,"inlinee":28226,"source_line":116,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":68196,"parent_record_offset":68156,"ancestor_record_offset":68120,"inlinee":28222,"source_line":39,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":68596,"parent_record_offset":68572,"ancestor_record_offset":68120,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":68644,"parent_record_offset":68620,"ancestor_record_offset":68120,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":68620,"parent_record_offset":68596,"ancestor_record_offset":68120,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":68812,"parent_record_offset":68428,"ancestor_record_offset":68120,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":68836,"parent_record_offset":68812,"ancestor_record_offset":68120,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":68928,"parent_record_offset":68156,"ancestor_record_offset":68120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":501,"record_offset":68276,"parent_record_offset":68252,"ancestor_record_offset":68120,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":501,"record_offset":68252,"parent_record_offset":68196,"ancestor_record_offset":68120,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":501,"record_offset":68300,"parent_record_offset":68276,"ancestor_record_offset":68120,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":68224,"parent_record_offset":68196,"ancestor_record_offset":68120,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$35::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F2F
// 首条位置: {"module":1651,"inlinee":81711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$31::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":30976,"parent_record_offset":30952,"ancestor_record_offset":30952,"inlinee":28157,"source_line":143,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1651,"record_offset":31008,"parent_record_offset":30976,"ancestor_record_offset":30952,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$23::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":31124,"parent_record_offset":30976,"ancestor_record_offset":30952,"inlinee":28159,"source_line":39,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":31180,"parent_record_offset":31156,"ancestor_record_offset":30952,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":31056,"parent_record_offset":31032,"ancestor_record_offset":30952,"inlinee":12394,"source_line":550,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":30952,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":31084,"parent_record_offset":31056,"ancestor_record_offset":30952,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":31204,"parent_record_offset":31180,"ancestor_record_offset":30952,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":31156,"parent_record_offset":31124,"ancestor_record_offset":30952,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":31032,"parent_record_offset":31008,"ancestor_record_offset":30952,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":31340,"parent_record_offset":31316,"ancestor_record_offset":30952,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":31228,"parent_record_offset":31204,"ancestor_record_offset":30952,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$19::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1651,"record_offset":31268,"parent_record_offset":31124,"ancestor_record_offset":30952,"inlinee":28149,"source_line":287,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$1::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1651,"record_offset":31292,"parent_record_offset":31268,"ancestor_record_offset":30952,"inlinee":28151,"source_line":78,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$3::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":5376,"rva":14251232,"va":5382960352}
// 编译实例 RVAs: 0xd974e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$3::run::async_fn$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E4A
// 首条位置: {"module":501,"inlinee":28234}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::future::into_future::impl$0::into_future
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\into_future.rs
//     evidence={"module":501,"record_offset":71728,"parent_record_offset":71644,"ancestor_record_offset":71644,"inlinee":20661,"source_line":142,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":71756,"parent_record_offset":71644,"ancestor_record_offset":71644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":72148,"parent_record_offset":72124,"ancestor_record_offset":71644,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":72172,"parent_record_offset":72148,"ancestor_record_offset":71644,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":72196,"parent_record_offset":72172,"ancestor_record_offset":71644,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":72124,"parent_record_offset":71992,"ancestor_record_offset":71644,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":71992,"parent_record_offset":71964,"ancestor_record_offset":71644,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":73692,"parent_record_offset":73664,"ancestor_record_offset":71644,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":73664,"parent_record_offset":73640,"ancestor_record_offset":71644,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":73640,"parent_record_offset":73616,"ancestor_record_offset":71644,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":73160,"parent_record_offset":73124,"ancestor_record_offset":71644,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":73124,"parent_record_offset":73096,"ancestor_record_offset":71644,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":72024,"parent_record_offset":71992,"ancestor_record_offset":71644,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":72088,"parent_record_offset":72052,"ancestor_record_offset":71644,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":72052,"parent_record_offset":72024,"ancestor_record_offset":71644,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":71812,"parent_record_offset":71756,"ancestor_record_offset":71644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":73192,"parent_record_offset":73160,"ancestor_record_offset":71644,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":74316,"parent_record_offset":74280,"ancestor_record_offset":71644,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":73220,"parent_record_offset":73160,"ancestor_record_offset":71644,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":73252,"parent_record_offset":73124,"ancestor_record_offset":71644,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$36::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E34
// 首条位置: {"module":501,"inlinee":28212}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$24::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66584,"parent_record_offset":66180,"ancestor_record_offset":66180,"inlinee":28218,"source_line":75,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66228,"parent_record_offset":66180,"ancestor_record_offset":66180,"inlinee":28214,"source_line":106,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67180,"parent_record_offset":67156,"ancestor_record_offset":66180,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67204,"parent_record_offset":67180,"ancestor_record_offset":66180,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66468,"parent_record_offset":66444,"ancestor_record_offset":66180,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66492,"parent_record_offset":66468,"ancestor_record_offset":66180,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67228,"parent_record_offset":67204,"ancestor_record_offset":66180,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66516,"parent_record_offset":66492,"ancestor_record_offset":66180,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":67156,"parent_record_offset":67004,"ancestor_record_offset":66180,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":66444,"parent_record_offset":66388,"ancestor_record_offset":66180,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66948,"parent_record_offset":66924,"ancestor_record_offset":66180,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66416,"parent_record_offset":66388,"ancestor_record_offset":66180,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66388,"parent_record_offset":66364,"ancestor_record_offset":66180,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67004,"parent_record_offset":66972,"ancestor_record_offset":66180,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66972,"parent_record_offset":66948,"ancestor_record_offset":66180,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":66364,"parent_record_offset":66340,"ancestor_record_offset":66180,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":67044,"parent_record_offset":67004,"ancestor_record_offset":66180,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":66896,"parent_record_offset":66836,"ancestor_record_offset":66180,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66868,"parent_record_offset":66836,"ancestor_record_offset":66180,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66924,"parent_record_offset":66896,"ancestor_record_offset":66180,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66812,"parent_record_offset":66788,"ancestor_record_offset":66180,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":66836,"parent_record_offset":66812,"ancestor_record_offset":66180,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":501,"record_offset":66788,"parent_record_offset":66764,"ancestor_record_offset":66180,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$62::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":67372,"parent_record_offset":66616,"ancestor_record_offset":66180,"inlinee":28226,"source_line":116,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66656,"parent_record_offset":66616,"ancestor_record_offset":66180,"inlinee":28222,"source_line":39,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66616,"parent_record_offset":66584,"ancestor_record_offset":66180,"inlinee":28220,"source_line":136,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$32::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":66256,"parent_record_offset":66228,"ancestor_record_offset":66180,"inlinee":28216,"source_line":150,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":67068,"parent_record_offset":67044,"ancestor_record_offset":66180,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":67116,"parent_record_offset":67092,"ancestor_record_offset":66180,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":67092,"parent_record_offset":67068,"ancestor_record_offset":66180,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":67284,"parent_record_offset":66896,"ancestor_record_offset":66180,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":67308,"parent_record_offset":67284,"ancestor_record_offset":66180,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":66292,"parent_record_offset":66256,"ancestor_record_offset":66180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":501,"record_offset":66740,"parent_record_offset":66716,"ancestor_record_offset":66180,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":501,"record_offset":66716,"parent_record_offset":66656,"ancestor_record_offset":66180,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":501,"record_offset":66764,"parent_record_offset":66740,"ancestor_record_offset":66180,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":66688,"parent_record_offset":66656,"ancestor_record_offset":66180,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$37::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F33
// 首条位置: {"module":1651,"inlinee":81715}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$25::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":42124,"parent_record_offset":41660,"ancestor_record_offset":41660,"inlinee":81721,"source_line":75,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$27::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":41804,"parent_record_offset":41660,"ancestor_record_offset":41660,"inlinee":81717,"source_line":106,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41692,"parent_record_offset":41660,"ancestor_record_offset":41660,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1651,"record_offset":41856,"parent_record_offset":41828,"ancestor_record_offset":41660,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":41972,"parent_record_offset":41828,"ancestor_record_offset":41660,"inlinee":14306,"source_line":3793,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$23::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":42296,"parent_record_offset":42148,"ancestor_record_offset":41660,"inlinee":28159,"source_line":39,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$29::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":42148,"parent_record_offset":42124,"ancestor_record_offset":41660,"inlinee":81723,"source_line":136,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$33::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1651,"record_offset":41828,"parent_record_offset":41804,"ancestor_record_offset":41660,"inlinee":81719,"source_line":150,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41716,"parent_record_offset":41692,"ancestor_record_offset":41660,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42024,"parent_record_offset":41996,"ancestor_record_offset":41660,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41904,"parent_record_offset":41880,"ancestor_record_offset":41660,"inlinee":12394,"source_line":550,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42488,"parent_record_offset":42464,"ancestor_record_offset":41660,"inlinee":28152,"source_line":386,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41932,"parent_record_offset":41904,"ancestor_record_offset":41660,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41740,"parent_record_offset":41716,"ancestor_record_offset":41660,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":42328,"parent_record_offset":42296,"ancestor_record_offset":41660,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41880,"parent_record_offset":41856,"ancestor_record_offset":41660,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":41996,"parent_record_offset":41972,"ancestor_record_offset":41660,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":42512,"parent_record_offset":42488,"ancestor_record_offset":41660,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":41764,"parent_record_offset":41740,"ancestor_record_offset":41660,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$19::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1651,"record_offset":42440,"parent_record_offset":42296,"ancestor_record_offset":41660,"inlinee":28149,"source_line":287,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$1::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1651,"record_offset":42464,"parent_record_offset":42440,"ancestor_record_offset":41660,"inlinee":28151,"source_line":78,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::strip_debug_name
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB240
// 首条位置: {"module":687,"inlinee":45632}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::String::clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":687,"record_offset":1364,"parent_record_offset":1344,"ancestor_record_offset":1344,"inlinee":18025,"source_line":1936,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":687,"record_offset":1384,"parent_record_offset":1364,"ancestor_record_offset":1344,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::BuildTextureAsset::new_single
// 记录形式: compiled_procedure; instances=1
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":1208,"rva":14249856,"va":5382958976}
// 编译实例 RVAs: 0xd96f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::strip_debug_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":687,"record_offset":1344,"parent_record_offset":1208,"ancestor_record_offset":1208,"inlinee":45632,"source_line":193,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":687,"record_offset":1364,"parent_record_offset":1344,"ancestor_record_offset":1208,"inlinee":18025,"source_line":1936,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":687,"record_offset":1384,"parent_record_offset":1364,"ancestor_record_offset":1208,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::BuildTextureAsset::new_array
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB244
// 首条位置: {"module":687,"inlinee":45636}
// 编译实例 RVAs: 0xd96fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::strip_debug_name
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":687,"record_offset":1564,"parent_record_offset":1424,"ancestor_record_offset":1424,"inlinee":45632,"source_line":193,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":687,"record_offset":1640,"parent_record_offset":1424,"ancestor_record_offset":1424,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":687,"record_offset":1584,"parent_record_offset":1564,"ancestor_record_offset":1424,"inlinee":18025,"source_line":1936,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":687,"record_offset":1604,"parent_record_offset":1584,"ancestor_record_offset":1424,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":687,"record_offset":1720,"parent_record_offset":1640,"ancestor_record_offset":1424,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":687,"record_offset":1688,"parent_record_offset":1640,"ancestor_record_offset":1424,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$5::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":5268,"rva":14251152,"va":5382960272}
// 编译实例 RVAs: 0xd97490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$5::run::async_fn$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E4F
// 首条位置: {"module":501,"inlinee":28239}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$1::run::async_fn$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":80344,"parent_record_offset":76512,"ancestor_record_offset":76512,"inlinee":28243,"source_line":84,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$2::run::async_fn$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":76660,"parent_record_offset":76512,"ancestor_record_offset":76512,"inlinee":28241,"source_line":115,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] core::future::into_future::impl$0::into_future
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\into_future.rs
//     evidence={"module":501,"record_offset":76632,"parent_record_offset":76512,"ancestor_record_offset":76512,"inlinee":20661,"source_line":142,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":77064,"parent_record_offset":77040,"ancestor_record_offset":76512,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":77088,"parent_record_offset":77064,"ancestor_record_offset":76512,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":78708,"parent_record_offset":78684,"ancestor_record_offset":76512,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":78732,"parent_record_offset":78708,"ancestor_record_offset":76512,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":77112,"parent_record_offset":77088,"ancestor_record_offset":76512,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":78756,"parent_record_offset":78732,"ancestor_record_offset":76512,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":77040,"parent_record_offset":77008,"ancestor_record_offset":76512,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":78684,"parent_record_offset":78628,"ancestor_record_offset":76512,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":501,"record_offset":79384,"parent_record_offset":79356,"ancestor_record_offset":76512,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":77492,"parent_record_offset":77460,"ancestor_record_offset":76512,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":76948,"parent_record_offset":76912,"ancestor_record_offset":76512,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":78656,"parent_record_offset":78628,"ancestor_record_offset":76512,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":78628,"parent_record_offset":78604,"ancestor_record_offset":76512,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":77540,"parent_record_offset":77516,"ancestor_record_offset":76512,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":77516,"parent_record_offset":77492,"ancestor_record_offset":76512,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":77008,"parent_record_offset":76976,"ancestor_record_offset":76512,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":76976,"parent_record_offset":76948,"ancestor_record_offset":76512,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":78604,"parent_record_offset":78580,"ancestor_record_offset":76512,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":78176,"parent_record_offset":78136,"ancestor_record_offset":76512,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":501,"record_offset":77996,"parent_record_offset":77928,"ancestor_record_offset":76512,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":81336,"parent_record_offset":81300,"ancestor_record_offset":76512,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":501,"record_offset":80176,"parent_record_offset":80152,"ancestor_record_offset":76512,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77460,"parent_record_offset":77408,"ancestor_record_offset":76512,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77968,"parent_record_offset":77928,"ancestor_record_offset":76512,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77372,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77408,"parent_record_offset":77372,"ancestor_record_offset":76512,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":76876,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":76912,"parent_record_offset":76876,"ancestor_record_offset":76512,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77900,"parent_record_offset":77872,"ancestor_record_offset":76512,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":79108,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77260,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":501,"record_offset":77928,"parent_record_offset":77900,"ancestor_record_offset":76512,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":501,"record_offset":79164,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":501,"record_offset":77872,"parent_record_offset":77844,"ancestor_record_offset":76512,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$62::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":501,"record_offset":78504,"parent_record_offset":77672,"ancestor_record_offset":76512,"inlinee":28226,"source_line":116,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":77724,"parent_record_offset":77672,"ancestor_record_offset":76512,"inlinee":28222,"source_line":39,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":501,"record_offset":77672,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":28230,"source_line":143,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":79416,"parent_record_offset":79384,"ancestor_record_offset":76512,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":501,"record_offset":78200,"parent_record_offset":78176,"ancestor_record_offset":76512,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":501,"record_offset":77172,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::future::into_future::impl$0::into_future
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\into_future.rs
//     evidence={"module":501,"record_offset":80472,"parent_record_offset":80344,"ancestor_record_offset":76512,"inlinee":20661,"source_line":142,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":501,"record_offset":78248,"parent_record_offset":78224,"ancestor_record_offset":76512,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":79480,"parent_record_offset":79444,"ancestor_record_offset":76512,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":501,"record_offset":79444,"parent_record_offset":79416,"ancestor_record_offset":76512,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":501,"record_offset":78224,"parent_record_offset":78200,"ancestor_record_offset":76512,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":501,"record_offset":77200,"parent_record_offset":77172,"ancestor_record_offset":76512,"inlinee":5375,"source_line":1428,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":501,"record_offset":77224,"parent_record_offset":77200,"ancestor_record_offset":76512,"inlinee":5376,"source_line":1409,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":501,"record_offset":78416,"parent_record_offset":77996,"ancestor_record_offset":76512,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":78440,"parent_record_offset":78416,"ancestor_record_offset":76512,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":78532,"parent_record_offset":77672,"ancestor_record_offset":76512,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":501,"record_offset":77332,"parent_record_offset":77308,"ancestor_record_offset":76512,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":79240,"parent_record_offset":79164,"ancestor_record_offset":76512,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":81368,"parent_record_offset":81336,"ancestor_record_offset":76512,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":79268,"parent_record_offset":79164,"ancestor_record_offset":76512,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":501,"record_offset":77640,"parent_record_offset":77616,"ancestor_record_offset":76512,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":79292,"parent_record_offset":79268,"ancestor_record_offset":76512,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":77580,"parent_record_offset":77408,"ancestor_record_offset":76512,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":79888,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":501,"record_offset":80312,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":501,"record_offset":77284,"parent_record_offset":77260,"ancestor_record_offset":76512,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":501,"record_offset":77308,"parent_record_offset":77284,"ancestor_record_offset":76512,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":501,"record_offset":77616,"parent_record_offset":76660,"ancestor_record_offset":76512,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":80212,"parent_record_offset":80176,"ancestor_record_offset":76512,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":80236,"parent_record_offset":80212,"ancestor_record_offset":76512,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":501,"record_offset":80268,"parent_record_offset":80176,"ancestor_record_offset":76512,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":501,"record_offset":77816,"parent_record_offset":77788,"ancestor_record_offset":76512,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":501,"record_offset":77788,"parent_record_offset":77724,"ancestor_record_offset":76512,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":501,"record_offset":77844,"parent_record_offset":77816,"ancestor_record_offset":76512,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":501,"record_offset":77760,"parent_record_offset":77724,"ancestor_record_offset":76512,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$5::run::async_fn_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":501,"record_offset":46732,"rva":9421312,"va":5378130432}
// 编译实例 RVAs: 0x8fc200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2451865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130432,"raw_hex":"e86796daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378130564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t605512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130432,"raw_hex":"e8483d0900","size":5,"soft_fail":false,"target_va":5378736160,"va":5378130643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409925d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t615269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378130432,"raw_hex":"e865630900","size":5,"soft_fail":false,"target_va":5378745808,"va":5378130534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1408fc2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t148","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378130432,"raw_hex":"e994000000","size":5,"soft_fail":false,"target_va":5378130656,"va":5378130503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t605544","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378130432,"raw_hex":"e9683d0900","size":5,"soft_fail":false,"target_va":5378736160,"va":5378130611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$2::run::async_fn_env$0> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-554","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378130432,"raw_hex":"e9d6fdffff","size":5,"soft_fail":false,"target_va":5378129936,"va":5378130485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":46900,"parent_record_offset":46732,"ancestor_record_offset":46732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":47124,"parent_record_offset":47100,"ancestor_record_offset":46732,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":47148,"parent_record_offset":47124,"ancestor_record_offset":46732,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":47172,"parent_record_offset":47148,"ancestor_record_offset":46732,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":501,"record_offset":47100,"parent_record_offset":47048,"ancestor_record_offset":46732,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":47072,"parent_record_offset":47048,"ancestor_record_offset":46732,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":47048,"parent_record_offset":47024,"ancestor_record_offset":46732,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":501,"record_offset":47024,"parent_record_offset":47004,"ancestor_record_offset":46732,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":46924,"parent_record_offset":46900,"ancestor_record_offset":46732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::build_texture_assets
// 记录形式: compiled_procedure; instances=1
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":2992,"rva":14250736,"va":5382959856}
// 编译实例 RVAs: 0xd972f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::build_texture_assets::async_fn$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 228, 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13ECC
// 首条位置: {"module":1647,"record_offset":2268,"rva":24057296,"va":5392766416}
// 编译实例 RVAs: 0x16f15d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <str as core::fmt::Display>::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7897200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8907f87ff","size":5,"soft_fail":false,"target_va":5384871088,"va":5392768283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8157351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8598783ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5392766754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17088209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e82f41fbfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5392766892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t-17089098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8b63dfbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5392767797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24044692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e86c1b91fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5392766879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=2
//     target_aliases=66 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23461676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8d4009afe","size":5,"soft_fail":false,"target_va":5369308864,"va":5392770535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d120
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14178082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8dea827ff","size":5,"soft_fail":false,"target_va":5378593056,"va":5392771133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972c40
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14155099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8a50228ff","size":5,"soft_fail":false,"target_va":5378616384,"va":5392771478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409797b0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14123636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e88c7d28ff","size":5,"soft_fail":false,"target_va":5378643888,"va":5392767519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098bb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14051103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8e19829ff","size":5,"soft_fail":false,"target_va":5378718608,"va":5392769706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098bd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14049758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8229e29ff","size":5,"soft_fail":false,"target_va":5378718992,"va":5392768745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098c3f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14050546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e80e9b29ff","size":5,"soft_fail":false,"target_va":5378720752,"va":5392771293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098cce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14048026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8e6a429ff","size":5,"soft_fail":false,"target_va":5378723040,"va":5392771061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409925d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14021974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8aa0a2aff","size":5,"soft_fail":false,"target_va":5378745808,"va":5392767777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14170bf40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t106494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8fe9f0100","size":5,"soft_fail":false,"target_va":5392875328,"va":5392768829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14170fd80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t123854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8cee30100","size":5,"soft_fail":false,"target_va":5392891264,"va":5392767405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23583995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8fbdc6701","size":5,"soft_fail":false,"target_va":5416351504,"va":5392767504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fb00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23584104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e868dd6701","size":5,"soft_fail":false,"target_va":5416352512,"va":5392768403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd39c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23994042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8ba1e6e01","size":5,"soft_fail":false,"target_va":5416761792,"va":5392767745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd5290
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23999066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e85a326e01","size":5,"soft_fail":false,"target_va":5416768144,"va":5392769073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7972831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8215886ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5392767690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23853229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8adf86b01","size":5,"soft_fail":false,"target_va":5416620794,"va":5392767560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t314423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e837cc0400","size":5,"soft_fail":false,"target_va":5393082944,"va":5392768516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8145557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e86bb583ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5392769632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::resolve_opts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9808848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e830546aff","size":5,"soft_fail":false,"target_va":5382958432,"va":5392767275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_async_fn_resumed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23854660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e844fe6b01","size":5,"soft_fail":false,"target_va":5416622656,"va":5392767991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_async_fn_resumed_panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23854704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e870fe6b01","size":5,"soft_fail":false,"target_va":5416622688,"va":5392767979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<asset_pipe::texture::BuildTextureAsset> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14637998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e852a420ff","size":5,"soft_fail":false,"target_va":5378131888,"va":5392769881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23852062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e81ef46b01","size":5,"soft_fail":false,"target_va":5416621632,"va":5392769565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_print
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8045687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392766416,"raw_hex":"e8893b85ff","size":5,"soft_fail":false,"target_va":5384721168,"va":5392766850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392766416,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5392768030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1647,"record_offset":6004,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":3236,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":2892,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":3180,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":3480,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1647,"record_offset":3640,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":17362,"source_line":58,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1647,"record_offset":2548,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":1647,"record_offset":2716,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":4284,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1647,"record_offset":5976,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1647,"record_offset":9340,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1647,"record_offset":2604,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1647,"record_offset":2464,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::BuiltAsset::asset_ref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1647,"record_offset":6532,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":81565,"source_line":555,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$33::deref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1647,"record_offset":6560,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":81567,"source_line":574,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":2268,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":2268,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=67
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":2268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=67
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":2268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":2268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=67
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":2268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":3048,"parent_record_offset":3016,"ancestor_record_offset":2268,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=67
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":4508,"parent_record_offset":4452,"ancestor_record_offset":2268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1647,"record_offset":5472,"parent_record_offset":5444,"ancestor_record_offset":2268,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1647,"record_offset":3704,"parent_record_offset":3680,"ancestor_record_offset":2268,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1647,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":2268,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":3356,"parent_record_offset":3324,"ancestor_record_offset":2268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":2956,"parent_record_offset":2924,"ancestor_record_offset":2268,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":4480,"parent_record_offset":4452,"ancestor_record_offset":2268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":2268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":2268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":2268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":3016,"parent_record_offset":2984,"ancestor_record_offset":2268,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2268,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":2268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":4040,"parent_record_offset":4000,"ancestor_record_offset":2268,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1647,"record_offset":6092,"parent_record_offset":6040,"ancestor_record_offset":2268,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1647,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":2268,"inlinee":4606,"source_line":2711,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1647,"record_offset":6040,"parent_record_offset":6004,"ancestor_record_offset":2268,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":7212,"parent_record_offset":7176,"ancestor_record_offset":2268,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":2268,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":3324,"parent_record_offset":3272,"ancestor_record_offset":2268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":3272,"parent_record_offset":3236,"ancestor_record_offset":2268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":2924,"parent_record_offset":2892,"ancestor_record_offset":2268,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":2268,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":7884,"parent_record_offset":7852,"ancestor_record_offset":2268,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":7852,"parent_record_offset":7824,"ancestor_record_offset":2268,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":2268,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":2268,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1647,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":2268,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1647,"record_offset":3792,"parent_record_offset":3640,"ancestor_record_offset":2268,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::build_texture_assets::async_fn$0::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":1647,"record_offset":3680,"parent_record_offset":3640,"ancestor_record_offset":2268,"inlinee":70353,"source_line":239,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1647,"record_offset":5504,"parent_record_offset":5472,"ancestor_record_offset":2268,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1647,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":2268,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":1647,"record_offset":6120,"parent_record_offset":6040,"ancestor_record_offset":2268,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1647,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":2268,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1647,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":2268,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1647,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":2268,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1647,"record_offset":5568,"parent_record_offset":5532,"ancestor_record_offset":2268,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1647,"record_offset":5532,"parent_record_offset":5504,"ancestor_record_offset":2268,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1647,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":2268,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1647,"record_offset":8248,"parent_record_offset":8216,"ancestor_record_offset":2268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":2268,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":3728,"parent_record_offset":3704,"ancestor_record_offset":2268,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1647,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":2268,"inlinee":5375,"source_line":1428,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1647,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":2268,"inlinee":5376,"source_line":1409,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1647,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":2268,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=175
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":3856,"parent_record_offset":3640,"ancestor_record_offset":2268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1647,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":2268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1647,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":2268,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":3552,"parent_record_offset":3480,"ancestor_record_offset":2268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":7244,"parent_record_offset":7212,"ancestor_record_offset":2268,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":2268,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":3580,"parent_record_offset":3480,"ancestor_record_offset":2268,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":3520,"parent_record_offset":3480,"ancestor_record_offset":2268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":2268,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":3444,"parent_record_offset":3272,"ancestor_record_offset":2268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1647,"record_offset":6204,"parent_record_offset":6040,"ancestor_record_offset":2268,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":2268,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":2268,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":7272,"parent_record_offset":7212,"ancestor_record_offset":2268,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":7304,"parent_record_offset":7176,"ancestor_record_offset":2268,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1647,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2268,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1647,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2268,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1647,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":2268,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::relative_path_buf::impl$19::fmt
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/relative_path_buf.rs
//     evidence={"module":1647,"record_offset":6148,"parent_record_offset":6040,"ancestor_record_offset":2268,"inlinee":70443,"source_line":464,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::build_texture_assets::async_fn$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112D1
// 首条位置: {"module":1253,"inlinee":70353}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1647,"record_offset":3704,"parent_record_offset":3680,"ancestor_record_offset":3680,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1647,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":3680,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":3728,"parent_record_offset":3704,"ancestor_record_offset":3680,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<asset_pipe::texture::build_texture_assets::async_fn_env$0> >
// 记录形式: compiled_procedure; instances=6
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":515,"record_offset":1752572,"rva":10119328,"va":5378828448}
// 编译实例 RVAs: 0x9a68a0, 0x17042b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=55
//     evidence={"assembly":"\tcallq\t-3149932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378828448,"raw_hex":"e894efcfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378828631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d120
//     owner=unresolved_namespace; count=6
//     target_aliases=6 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-236266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378828448,"raw_hex":"e81665fcff","size":5,"soft_fail":false,"target_va":5378593056,"va":5378829317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098bd10
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-109701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378828448,"raw_hex":"e87b53feff","size":5,"soft_fail":false,"target_va":5378718992,"va":5378828688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098cce0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-106036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378828448,"raw_hex":"e8cc61feff","size":5,"soft_fail":false,"target_va":5378723040,"va":5378829071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140992690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-82840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378828448,"raw_hex":"e868bcfeff","size":5,"soft_fail":false,"target_va":5378746000,"va":5378828835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-3150207","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378828448,"raw_hex":"e981eecfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378828906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140972c40
//     owner=unresolved_namespace; count=5
//     target_aliases=4 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-14227425","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5392843440,"raw_hex":"e91fe826ff","size":5,"soft_fail":false,"target_va":5378616384,"va":5392843804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378828448,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378828532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":515,"record_offset":1752748,"parent_record_offset":1752572,"ancestor_record_offset":1752572,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=62
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":515,"record_offset":1752952,"parent_record_offset":1752928,"ancestor_record_offset":1752572,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=62
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":515,"record_offset":1752976,"parent_record_offset":1752952,"ancestor_record_offset":1752572,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=62
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":515,"record_offset":1753000,"parent_record_offset":1752976,"ancestor_record_offset":1752572,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=62
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":515,"record_offset":1752928,"parent_record_offset":1752804,"ancestor_record_offset":1752572,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":515,"record_offset":1752804,"parent_record_offset":1752776,"ancestor_record_offset":1752572,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":1754008,"parent_record_offset":1753980,"ancestor_record_offset":1752572,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":1753980,"parent_record_offset":1753956,"ancestor_record_offset":1752572,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":1753956,"parent_record_offset":1753932,"ancestor_record_offset":1752572,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":1755128,"parent_record_offset":1755100,"ancestor_record_offset":1752572,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":515,"record_offset":1754232,"parent_record_offset":1754200,"ancestor_record_offset":1752572,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":515,"record_offset":1754200,"parent_record_offset":1754172,"ancestor_record_offset":1752572,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":515,"record_offset":1754452,"parent_record_offset":1754428,"ancestor_record_offset":1752572,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":515,"record_offset":1754304,"parent_record_offset":1754280,"ancestor_record_offset":1752572,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":515,"record_offset":1752832,"parent_record_offset":1752804,"ancestor_record_offset":1752572,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":515,"record_offset":1754280,"parent_record_offset":1754256,"ancestor_record_offset":1752572,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":515,"record_offset":1754256,"parent_record_offset":1754232,"ancestor_record_offset":1752572,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":515,"record_offset":1752892,"parent_record_offset":1752860,"ancestor_record_offset":1752572,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":515,"record_offset":1752860,"parent_record_offset":1752832,"ancestor_record_offset":1752572,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":515,"record_offset":1754596,"parent_record_offset":1754564,"ancestor_record_offset":1752572,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":515,"record_offset":1754376,"parent_record_offset":1754352,"ancestor_record_offset":1752572,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=115
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":515,"record_offset":1752776,"parent_record_offset":1752748,"ancestor_record_offset":1752572,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":515,"record_offset":1754352,"parent_record_offset":1754328,"ancestor_record_offset":1752572,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":515,"record_offset":1754328,"parent_record_offset":1754304,"ancestor_record_offset":1752572,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::resolve_opts
// 记录形式: compiled_procedure; instances=1
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":264,"rva":14249312,"va":5382958432}
// 编译实例 RVAs: 0xd96d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33892767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382958432,"raw_hex":"e89f290502","size":5,"soft_fail":false,"target_va":5416851728,"va":5382958956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":687,"record_offset":1176,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":687,"record_offset":424,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":687,"record_offset":396,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":687,"record_offset":516,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":687,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":264,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":924,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38718,"source_line":34,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1040,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38720,"source_line":55,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1120,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38724,"source_line":82,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1080,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38722,"source_line":103,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":884,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38716,"source_line":9,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$67::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":748,"parent_record_offset":696,"ancestor_record_offset":264,"inlinee":38714,"source_line":122,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::resolve_opts::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":687,"record_offset":456,"parent_record_offset":424,"ancestor_record_offset":264,"inlinee":45630,"source_line":268,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::resolve_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":687,"record_offset":648,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":45629,"source_line":269,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":696,"parent_record_offset":648,"ancestor_record_offset":264,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":264,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":972,"parent_record_offset":924,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":687,"record_offset":844,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":687,"record_offset":604,"parent_record_offset":568,"ancestor_record_offset":264,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":687,"record_offset":568,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::resolve_opts::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB23E
// 首条位置: {"module":687,"inlinee":45630}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":687,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":456,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::resolve_opts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB23D
// 首条位置: {"module":687,"inlinee":45629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":696,"parent_record_offset":648,"ancestor_record_offset":648,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":924,"parent_record_offset":748,"ancestor_record_offset":648,"inlinee":38718,"source_line":34,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1040,"parent_record_offset":748,"ancestor_record_offset":648,"inlinee":38720,"source_line":55,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1120,"parent_record_offset":748,"ancestor_record_offset":648,"inlinee":38724,"source_line":82,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1080,"parent_record_offset":748,"ancestor_record_offset":648,"inlinee":38722,"source_line":103,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":884,"parent_record_offset":748,"ancestor_record_offset":648,"inlinee":38716,"source_line":9,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$67::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":748,"parent_record_offset":696,"ancestor_record_offset":648,"inlinee":38714,"source_line":122,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":648,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":972,"parent_record_offset":924,"ancestor_record_offset":648,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":687,"record_offset":844,"parent_record_offset":748,"ancestor_record_offset":648,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::discover_texture_asset_recipes::closure$0
// 记录形式: inlinee_declaration; instances=6
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1751F
// 首条位置: {"module":2061,"inlinee":95519}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::source_image_format
// 记录形式: compiled_procedure; instances=1
// 源码行: 313
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":1980,"rva":14250240,"va":5382959360}
// 编译实例 RVAs: 0xd97100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1697523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382959360,"raw_hex":"e8f3e61900","size":5,"soft_fail":false,"target_va":5384656912,"va":5382959384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417f9ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10889621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382959360,"raw_hex":"e89529a600","size":5,"soft_fail":false,"target_va":5393849056,"va":5382959430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::source_image_format::closure$1
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t33592834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382959360,"raw_hex":"e802960002","size":5,"soft_fail":false,"target_va":5416552288,"va":5382959449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1870670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382959360,"raw_hex":"e84e8b1c00","size":5,"soft_fail":false,"target_va":5384830080,"va":5382959405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":687,"record_offset":2116,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":687,"record_offset":2280,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":687,"record_offset":2308,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::parse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":687,"record_offset":2252,"parent_record_offset":1980,"ancestor_record_offset":1980,"inlinee":6807,"source_line":2765,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::source_image_format::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":687,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":1980,"inlinee":45634,"source_line":315,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":687,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":1980,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":687,"record_offset":2160,"parent_record_offset":2140,"ancestor_record_offset":1980,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":687,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":1980,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::source_image_format::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB242
// 首条位置: {"module":687,"inlinee":45634}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":687,"record_offset":2160,"parent_record_offset":2140,"ancestor_record_offset":2140,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":687,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":2140,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":687,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":2140,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::source_image_format::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":2340,"rva":47843168,"va":5416552288}
// 编译实例 RVAs: 0x2da0760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31757471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416552288,"raw_hex":"e8616b1bfe","size":5,"soft_fail":false,"target_va":5384794864,"va":5416552330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t299209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416552288,"raw_hex":"e8c9900400","size":5,"soft_fail":false,"target_va":5416851552,"va":5416552338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":687,"record_offset":2476,"parent_record_offset":2340,"ancestor_record_offset":2340,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":687,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":2340,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":687,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":2340,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00687.jsonl.gz"}
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
// 完整结构及原始类型记录: texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::ArchivedTextureAssetListEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$15::deserialize_unsized<asset_pipe::texture::ArchivedTextureAssetListEntry,asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >", "constructor": "rkyv::impls::core::impl$15::deserialize_unsized", "arguments": ["asset_pipe::texture::ArchivedTextureAssetListEntry", "asset_pipe::texture::TextureAssetListEntry", "rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<asset_pipe::texture::ArchivedTextureAssetListEntry>", "constructor": "slice2$", "arguments": ["asset_pipe::texture::ArchivedTextureAssetListEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::vec::verify::impl$0::verify::closure_env$0<asset_pipe::texture::ArchivedTextureAssetListEntry,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >", "constructor": "rkyv::vec::verify::impl$0::verify::closure_env$0", "arguments": ["asset_pipe::texture::ArchivedTextureAssetListEntry", "rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::validation::archive::impl$1::in_subtree<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>,rancor::Error,slice2$<asset_pipe::texture::ArchivedTextureAssetListEntry>,tuple$<>,rkyv::vec::verify::impl$0::verify::closure_env$0<asset_pipe::texture::ArchivedTextureAssetListEntry,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > >", "constructor": "rkyv::validation::archive::impl$1::in_subtree", "arguments": ["rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>", "rancor::Error", "slice2$<asset_pipe::texture::ArchivedTextureAssetListEntry>", "tuple$<>", "rkyv::vec::verify::impl$0::verify::closure_env$0<asset_pipe::texture::ArchivedTextureAssetListEntry,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::BuildSingleTexture
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::any::type_name<asset_pipe::texture::BuildSingleTexture>", "constructor": "core::any::type_name", "arguments": ["asset_pipe::texture::BuildSingleTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::BuildTextureArray
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset_pipe::texture::BuildTextureArray>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset_pipe::texture::BuildTextureArray"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::type_name<asset_pipe::texture::BuildTextureArray>", "constructor": "core::any::type_name", "arguments": ["asset_pipe::texture::BuildTextureArray"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::BuildTextureAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 27
// [PDB 泛型关系，非字段关系] {"type": "enum2$<asset_pipe::texture::BuildTextureAsset>", "constructor": "enum2$", "arguments": ["asset_pipe::texture::BuildTextureAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "turbosloth::asset::impl$24::run::async_block_env$0", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::debug_name<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "turbosloth::asset::impl$25::debug_name", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "turbosloth::asset::impl$25::clone_boxed", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::version_registry_exempt<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "turbosloth::asset::impl$25::version_registry_exempt", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::run_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "turbosloth::asset::impl$25::run_boxed", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "turbosloth::asset::impl$24::run::async_block$0", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "tuple$", "arguments": ["alloc::string::String", "enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "tuple$", "arguments": ["relative_path::relative_path_buf::RelativePathBuf", "enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<alloc::string::String,enum2$<asset_pipe::texture::BuildTextureAsset> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::type_name<enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "core::any::type_name", "arguments": ["enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::BuildContext::require<enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "turbosloth::asset::BuildContext::require", "arguments": ["enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$38::require::async_block$0<enum2$<asset_pipe::texture::BuildTextureAsset> >", "constructor": "turbosloth::asset::impl$38::require::async_block$0", "arguments": ["enum2$<asset_pipe::texture::BuildTextureAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<relative_path::relative_path_buf::RelativePathBuf,enum2$<asset_pipe::texture::BuildTextureAsset> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::BuildTextureLayer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>", "constructor": "turbosloth::asset::impl$24::run::async_block_env$0", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::debug_name<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>", "constructor": "turbosloth::asset::impl$25::debug_name", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>", "constructor": "turbosloth::asset::impl$25::clone_boxed", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::version<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>", "constructor": "turbosloth::asset::impl$25::version", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::run_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>", "constructor": "turbosloth::asset::impl$25::run_boxed", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>", "constructor": "turbosloth::asset::impl$24::run::async_block$0", "arguments": ["asset::texture::blob::TextureAssetBlob", "anyhow::Error", "asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset_pipe::texture::BuildTextureLayer>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<asset_pipe::texture::BuildTextureLayer>", "constructor": "alloc::sync::ArcInner", "arguments": ["asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<asset_pipe::texture::BuildTextureLayer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<asset_pipe::texture::BuildTextureLayer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::type_name<asset_pipe::texture::BuildTextureLayer>", "constructor": "core::any::type_name", "arguments": ["asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::BuildContext::build<asset_pipe::texture::BuildTextureLayer>", "constructor": "turbosloth::asset::BuildContext::build", "arguments": ["asset_pipe::texture::BuildTextureLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetList
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 50
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$5::serialize_unsized<asset_pipe::texture::TextureAssetList,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >", "constructor": "rkyv::impls::core::impl$5::serialize_unsized", "arguments": ["asset_pipe::texture::TextureAssetList", "rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$3::to_archive_bytes<asset_pipe::texture::TextureAssetList>", "constructor": "turbosloth::asset::impl$3::to_archive_bytes", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>", "constructor": "turbosloth::asset::StoreAssetRecipe", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >", "constructor": "core::ptr::drop_in_place", "arguments": ["turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >", "constructor": "alloc::sync::ArcInner", "arguments": ["turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["asset_pipe::texture::TextureAssetList", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError>", "constructor": "core::result::Result", "arguments": ["alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>", "turbosloth::asset::AssetBuildError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::Send<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "async_channel::Send", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_channel::Send<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_channel::Send<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::Sender<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "async_channel::Sender", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_channel::Sender<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_channel::Sender<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::Receiver<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "async_channel::Receiver", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_channel::Receiver<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_channel::Receiver<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "async_channel::Channel", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "alloc::sync::ArcInner", "arguments": ["async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset_pipe::texture::TextureAssetList>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<asset_pipe::texture::TextureAssetList>", "constructor": "alloc::sync::ArcInner", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<asset_pipe::texture::TextureAssetList> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::impl$30::poll_with_strategy<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >,event_listener_strategy::NonBlocking>", "constructor": "async_channel::impl$30::poll_with_strategy", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "event_listener_strategy::NonBlocking"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::impl$29::poll_with_strategy<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >,event_listener_strategy::NonBlocking>", "constructor": "async_channel::impl$29::poll_with_strategy", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "event_listener_strategy::NonBlocking"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset_pipe::texture::TextureAssetList>", "constructor": "rkyv::api::high::to_bytes_in::closure_env$0", "arguments": ["rkyv::util::alloc::aligned_vec::AlignedVec<16>", "rancor::Error", "asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::util::alloc::arena::detail::with_arena::closure_env$0<enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >,rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset_pipe::texture::TextureAssetList> >", "constructor": "rkyv::util::alloc::arena::detail::with_arena::closure_env$0", "arguments": ["enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >", "rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::local::LocalKey::with<core::cell::Cell<enum2$<core::option::Option<rkyv::ser::allocator::alloc::Arena> > >,rkyv::util::alloc::arena::detail::with_arena::closure_env$0<enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >,rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset_pipe::texture::TextureAssetList> >,enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> > >", "constructor": "std::thread::local::LocalKey::with", "arguments": ["core::cell::Cell<enum2$<core::option::Option<rkyv::ser::allocator::alloc::Arena> > >", "rkyv::util::alloc::arena::detail::with_arena::closure_env$0<enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >,rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset_pipe::texture::TextureAssetList> >", "enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::BuildContext::store_asset<asset_pipe::texture::TextureAssetList>", "constructor": "turbosloth::asset::BuildContext::store_asset", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$38::require::async_block$0<turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >", "constructor": "turbosloth::asset::impl$38::require::async_block$0", "arguments": ["turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block$1<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block$1", "arguments": ["asset_pipe::texture::TextureAssetList", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block$2<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block$2", "arguments": ["asset_pipe::texture::TextureAssetList", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block$3<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block$3", "arguments": ["asset_pipe::texture::TextureAssetList", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block$1::closure$3<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block$1::closure$3", "arguments": ["asset_pipe::texture::TextureAssetList", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block$1::closure$8<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block$1::closure$8", "arguments": ["asset_pipe::texture::TextureAssetList", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::downcast_build_artifact<asset_pipe::texture::TextureAssetList>", "constructor": "turbosloth::asset::downcast_build_artifact", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<asset_pipe::texture::TextureAssetList>", "constructor": "core::any::impl$0::type_id", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block_env$2<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block_env$2", "arguments": ["asset_pipe::texture::TextureAssetList", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$44::build::async_block_env$2<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext> >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$44::build::async_block_env$2<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$44::build::async_block_env$2<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$44::build::async_block_env$2<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block_env$3<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block_env$3", "arguments": ["asset_pipe::texture::TextureAssetList", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$44::build::async_block_env$3<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext> >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$44::build::async_block_env$3<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$44::build::async_block_env$3<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$44::build::async_block_env$3<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$22::run::async_fn_env$0<asset_pipe::texture::TextureAssetList>", "constructor": "turbosloth::asset::impl$22::run::async_fn_env$0", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$22::run::async_fn_env$0<asset_pipe::texture::TextureAssetList> >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$22::run::async_fn_env$0<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$22::run::async_fn_env$0<asset_pipe::texture::TextureAssetList> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$22::run::async_fn_env$0<asset_pipe::texture::TextureAssetList> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block_env$0<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >", "constructor": "turbosloth::asset::impl$24::run::async_block_env$0", "arguments": ["asset_pipe::texture::TextureAssetList", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> > >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$24::run::async_block_env$0<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::debug_name<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >", "constructor": "turbosloth::asset::impl$25::debug_name", "arguments": ["asset_pipe::texture::TextureAssetList", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::clone_boxed<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >", "constructor": "turbosloth::asset::impl$25::clone_boxed", "arguments": ["asset_pipe::texture::TextureAssetList", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::run_boxed<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >", "constructor": "turbosloth::asset::impl$25::run_boxed", "arguments": ["asset_pipe::texture::TextureAssetList", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block$0<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >", "constructor": "turbosloth::asset::impl$24::run::async_block$0", "arguments": ["asset_pipe::texture::TextureAssetList", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["asset_pipe::texture::TextureAssetList", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::high::to_bytes_in_with_alloc<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rancor::Error,asset_pipe::texture::TextureAssetList>", "constructor": "rkyv::api::high::to_bytes_in_with_alloc", "arguments": ["rkyv::util::alloc::aligned_vec::AlignedVec<16>", "rkyv::ser::allocator::alloc::ArenaHandle", "rancor::Error", "asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::high::checked::from_bytes<asset_pipe::texture::TextureAssetList,rancor::Error>", "constructor": "rkyv::api::high::checked::from_bytes", "arguments": ["asset_pipe::texture::TextureAssetList", "rancor::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "concurrent_queue::bounded::impl$0::push::closure_env$0", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "concurrent_queue::bounded::Bounded::push_or_else<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >,concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "concurrent_queue::bounded::Bounded::push_or_else", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "concurrent_queue::bounded::Bounded::pop<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "concurrent_queue::bounded::Bounded::pop", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::closure_env$0<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::closure_env$0", "arguments": ["asset_pipe::texture::TextureAssetList", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::AssetStore::get_or_insert_with<turbosloth::asset::impl$44::build::closure_env$0<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext> >", "constructor": "turbosloth::store::AssetStore::get_or_insert_with", "arguments": ["turbosloth::asset::impl$44::build::closure_env$0<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::read_packaged_asset<asset_pipe::texture::TextureAssetList,ref$<std::path::Path> >", "constructor": "turbosloth::store::read_packaged_asset", "arguments": ["asset_pipe::texture::TextureAssetList", "ref$<std::path::Path>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::read_packaged_asset<asset_pipe::texture::TextureAssetList,ref$<std::path::PathBuf> >", "constructor": "turbosloth::store::read_packaged_asset", "arguments": ["asset_pipe::texture::TextureAssetList", "ref$<std::path::PathBuf>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::DiskStore::store<asset_pipe::texture::TextureAssetList>", "constructor": "turbosloth::store::DiskStore::store", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::DiskStore::try_load<asset_pipe::texture::TextureAssetList>", "constructor": "turbosloth::store::DiskStore::try_load", "arguments": ["asset_pipe::texture::TextureAssetList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "concurrent_queue::unbounded::Unbounded::push<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "concurrent_queue::unbounded::Unbounded::push", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset_pipe::texture::TextureAssetList,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetListEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 65
// [PDB 泛型关系，非字段关系] {"type": "core::hash::Hash::hash_slice<asset_pipe::texture::TextureAssetListEntry,turbosloth::id::Hash128>", "constructor": "core::hash::Hash::hash_slice", "arguments": ["asset_pipe::texture::TextureAssetListEntry", "turbosloth::id::Hash128"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$15::deserialize_unsized<asset_pipe::texture::ArchivedTextureAssetListEntry,asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >", "constructor": "rkyv::impls::core::impl$15::deserialize_unsized", "arguments": ["asset_pipe::texture::ArchivedTextureAssetListEntry", "asset_pipe::texture::TextureAssetListEntry", "rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset_pipe::texture::TextureAssetListEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "asset_pipe::texture::TextureAssetListEntry", "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "asset_pipe::texture::TextureAssetListEntry", "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::async_block_env$0", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::impl$3::run::Guard<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::impl$3::run::Guard", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_task::raw::impl$3::run::Guard<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_task::raw::impl$3::run::Guard<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["asset_pipe::texture::TextureAssetListEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>", "constructor": "core::slice::iter::Iter", "arguments": ["asset_pipe::texture::TextureAssetListEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>", "country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<core::option::Option<rhapsody::raw::texture::Texture> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>", "country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<core::option::Option<rhapsody::raw::texture::Texture> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "asset_pipe::texture::TextureAssetListEntry", "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "futures_lite::future::block_on::closure_env$0", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >,alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "std::thread::local::LocalKey::with", "arguments": ["core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >", "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "futures_lite::future::block_on::closure_env$0", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >,alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "std::thread::local::LocalKey::with", "arguments": ["core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >", "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::closure_env$1", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "asset_pipe::texture::TextureAssetListEntry", "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::closure_env$1", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >", "constructor": "rkyv::impls::core::impl$14::serialize_unsized::closure_env$0", "arguments": ["asset_pipe::texture::TextureAssetListEntry", "rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::util::ser_vec::SerVec::with_capacity<asset_pipe::texture::TextureAssetListEntryResolver,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>,enum2$<core::result::Result<usize,rancor::Error> >,rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> > >", "constructor": "rkyv::util::ser_vec::SerVec::with_capacity", "arguments": ["asset_pipe::texture::TextureAssetListEntryResolver", "rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>", "enum2$<core::result::Result<usize,rancor::Error> >", "rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>", "country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$78::from_iter<ref$<str$>,turbosloth::asset::AssetRef<asset::texture::blob::TextureAssetBlob>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$0> >", "constructor": "std::collections::hash::map::impl$78::from_iter", "arguments": ["ref$<str$>", "turbosloth::asset::AssetRef<asset::texture::blob::TextureAssetBlob>", "std::hash::random::RandomState", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_mod_textures::packaged_textures::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>", "country_core::startup::load_mod_textures::packaged_textures::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$78::from_iter<alloc::string::String,country_core::startup::load_mod_textures::PackagedTexture,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_mod_textures::packaged_textures::closure_env$2> >", "constructor": "std::collections::hash::map::impl$78::from_iter", "arguments": ["alloc::string::String", "country_core::startup::load_mod_textures::PackagedTexture", "std::hash::random::RandomState", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_mod_textures::packaged_textures::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn$0::async_block_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >,enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn$0::async_block_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >,enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$9::poll<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "futures_lite::future::impl$9::poll", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$9::poll<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "futures_lite::future::impl$9::poll", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::impl$1::next<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,core::iter::adapters::flatten::Flatten<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::zip::impl$1::next", "arguments": ["core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>", "core::iter::adapters::flatten::Flatten<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::async_block_env$0", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::clone_waker<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::clone_waker", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::clone_waker<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::clone_waker", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::drop_future<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::drop_future", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::drop_future<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::drop_future", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::wake_by_ref<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::wake_by_ref", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::run<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::run", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::run<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::run", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::schedule<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::schedule", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetListEntryResolver
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "rkyv::util::ser_vec::SerVec::with_capacity<asset_pipe::texture::TextureAssetListEntryResolver,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>,enum2$<core::result::Result<usize,rancor::Error> >,rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> > >", "constructor": "rkyv::util::ser_vec::SerVec::with_capacity", "arguments": ["asset_pipe::texture::TextureAssetListEntryResolver", "rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>", "enum2$<core::result::Result<usize,rancor::Error> >", "rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureLayerAssetDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::hash::Hash::hash_slice<asset_pipe::texture::TextureLayerAssetDesc,turbosloth::id::Hash128>", "constructor": "core::hash::Hash::hash_slice", "arguments": ["asset_pipe::texture::TextureLayerAssetDesc", "turbosloth::id::Hash128"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset_pipe::texture::TextureLayerAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<asset_pipe::texture::TextureLayerAssetDesc,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["asset_pipe::texture::TextureLayerAssetDesc", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["asset_pipe::texture::TextureLayerAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["asset_pipe::texture::TextureLayerAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["asset_pipe::texture::TextureLayerAssetDesc", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["asset_pipe::texture::TextureLayerAssetDesc", "alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,asset_pipe::texture::TextureLayerAssetDesc,tuple$<>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["std::path::PathBuf", "asset_pipe::texture::TextureLayerAssetDesc", "tuple$<>", "country_core::startup::bake_mod_textures::bake_recipe::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<std::path::PathBuf,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,asset_pipe::texture::TextureLayerAssetDesc,tuple$<>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> > > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["std::path::PathBuf", "alloc::alloc::Global", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,asset_pipe::texture::TextureLayerAssetDesc,tuple$<>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureSourceImage
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 103; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::BuildTextureAsset::new_array
// 信息 ID: 25397e29d6a4ed7429f22617
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources @ 0xc7fde8 -> 0xd96fc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::BuildTextureAsset::new_single
// 信息 ID: 41fd06684912a7ee35fad9eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources @ 0xc7fe5d -> 0xd96f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::icon_texture_recipe @ 0xcee820 -> 0xd96f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_pack_pipe::build_object_textures::async_fn$0 @ 0x16f30d1 -> 0xd96f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > @ 0x258e8bb -> 0xd96f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a45f2b -> 0xd96f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::TextureSourceImage::load
// 信息 ID: 3f20162e048cacfc5581d869
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> @ 0x8fd75b -> 0xd970e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe29b -> 0xd970e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::build_texture_assets
// 信息 ID: dcc80be4620c77a5e3b4e611
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::load_clutter_packs::load_clutter_packs_at_startup::closure_env$1> > >,enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> > > @ 0x16fa229 -> 0xd972f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> >,enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> > >,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > @ 0x16feb59 -> 0xd972f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7> > >,enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> > > @ 0x1700299 -> 0xd972f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > @ 0x258cf13 -> 0xd972f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a42f02 -> 0xd972f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::build_texture_assets::async_fn$0
// 信息 ID: 8090b5cc02b04cebf1f7134f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::load_clutter_packs::load_clutter_packs_at_startup::closure_env$1> > >,enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> > > @ 0x16fab6d -> 0x16f15d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> >,enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> > >,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > @ 0x16fef68 -> 0x16f15d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7> > >,enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> > > @ 0x1700bdd -> 0x16f15d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> >; module=3252, inline_record=117348; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0; module=4413, inline_record=4756; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::build_texture_assets::async_fn$0::closure$0
// 信息 ID: 3a330b534bc901f6c86bc2f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::build_texture_assets::async_fn$0", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::build_texture_assets::async_fn$0; module=1647, inline_record=3680; 不是独立调用边。
// [PDB 内联宿主] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> >; module=3252, inline_record=118892; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0; module=4413, inline_record=6308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::discover_texture_asset_recipes::closure$0
// 信息 ID: ad8f6454d4866c40a77acd0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::discover_texture_asset_recipes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$0::load::async_fn$0
// 信息 ID: 8b4f21f623a4d5868d4f4cb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> @ 0x8fd7b3 -> 0x8f82c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe2f3 -> 0x8f82c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$0::load::async_fn$0::closure$0
// 信息 ID: 8c3b61674eac0b308e66a2ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::impl$0::load::async_fn$0", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::impl$0::load::async_fn$0::closure_env$0 <- anyhow::context::impl$0::with_context<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::texture::impl$0::load::async_fn$0::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::texture::impl$0::load::async_fn$0::closure_env$0>; module=1253, inline_record=41832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$10::check_bytes
// 信息 ID: d7af59d030ca2dcddd845b9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::validation::archive::impl$1::in_subtree<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>,rancor::Error,slice2$<asset_pipe::texture::ArchivedTextureAssetListEntry>,tuple$<>,rkyv::vec::verify::impl$0::verify::closure_env$0<asset_pipe::texture::ArchivedTextureAssetListEntry,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > >; module=1670, inline_record=4244; 不是独立调用边。
// [PDB 内联宿主] rkyv::validation::archive::impl$1::in_subtree<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>,rancor::Error,slice2$<asset_pipe::texture::ArchivedTextureAssetListEntry>,tuple$<>,rkyv::vec::verify::impl$0::verify::closure_env$0<asset_pipe::texture::ArchivedTextureAssetListEntry,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > >; module=4565, inline_record=4280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$11::serialize
// 信息 ID: 05cf565fb1f2bc074c127630
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::util::ser_vec::SerVec::with_capacity<asset_pipe::texture::TextureAssetListEntryResolver,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>,enum2$<core::result::Result<usize,rancor::Error> >,rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> > >; module=1663, inline_record=19584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$12::deserialize
// 信息 ID: dd12ebcceca5122b920ee80d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset_pipe::texture::ArchivedTextureAssetListEntry,asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=501, inline_record=64168; 不是独立调用边。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset_pipe::texture::ArchivedTextureAssetListEntry,asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=4570, inline_record=676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$13::visit_asset_dependencies
// 信息 ID: 1b12148639c95feac6835f20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::impl$21::visit_asset_dependencies; module=687, inline_record=5712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$14::clone
// 信息 ID: d26cbd21e606b67cc746a9ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset_pipe::texture::TextureAssetList,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset_pipe::texture::TextureAssetList> >; module=1651, inline_record=350400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$15::hash
// 信息 ID: 37befbe784f2886d20aaf6fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::store_asset<asset_pipe::texture::TextureAssetList>; module=1651, inline_record=9924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$16::resolve
// 信息 ID: 0509c14ab9bb413a39392923
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rkyv::impls::core::impl$5::serialize_unsized<asset_pipe::texture::TextureAssetList,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> > @ 0x8fcbc3 -> 0xd97350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$18::check_bytes
// 信息 ID: 2681e50340784dfde69ab423
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset_pipe::texture::TextureAssetList,rancor::Error>; module=1666, inline_record=117544; 不是独立调用边。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset_pipe::texture::TextureAssetList,rancor::Error>; module=3529, inline_record=1224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$19::serialize
// 信息 ID: 16c12c9123ee250edd8c9841
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$5::serialize_unsized<asset_pipe::texture::TextureAssetList,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >; module=501, inline_record=62616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$1::run::async_fn$0
// 信息 ID: cf2598e269a7b8ff33fb3cbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=80344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$20::deserialize
// 信息 ID: 82e03b1b25c0b95298a5fa03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset_pipe::texture::TextureAssetList,rancor::Error>; module=1666, inline_record=118680; 不是独立调用边。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset_pipe::texture::TextureAssetList,rancor::Error>; module=3529, inline_record=2360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$21::visit_asset_dependencies
// 信息 ID: 85712c57a0c5cf07ff0437bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$44::build::async_block$1<asset_pipe::texture::TextureAssetList,turbosloth::asset::BuildContext> @ 0x1715a8a -> 0xd97530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$22::clone
// 信息 ID: ba51fcb7e5730c832c66c445
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=66656; 不是独立调用边。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>; module=501, inline_record=68196; 不是独立调用边。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=77724; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$12::clone<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global>; module=1643, inline_record=264532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$23::hash
// 信息 ID: 5f9c38faff1cb47344577751
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::Hash::hash_slice<asset_pipe::texture::TextureLayerAssetDesc,turbosloth::id::Hash128>; module=501, inline_record=61456; 不是独立调用边。
// [PDB 内联宿主] turbosloth::asset::BuildContext::build<asset_pipe::texture::BuildTextureLayer>; module=1651, inline_record=31124; 不是独立调用边。
// [PDB 内联宿主] turbosloth::asset::BuildContext::require<enum2$<asset_pipe::texture::BuildTextureAsset> >; module=1651, inline_record=42296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$24::clone
// 信息 ID: 45b321a7db5ac18e7420f91e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=66584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$25::hash
// 信息 ID: c47e8386e03383cf5c12313c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::require<enum2$<asset_pipe::texture::BuildTextureAsset> >; module=1651, inline_record=42124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$26::clone
// 信息 ID: ddb5a19caec4906ec3478fa7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=66228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$27::hash
// 信息 ID: aeb0bc627d70f560e7aea6d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::require<enum2$<asset_pipe::texture::BuildTextureAsset> >; module=1651, inline_record=41804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$28::clone
// 信息 ID: cf6e1fbe1631a555e6b471bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=66616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$29::hash
// 信息 ID: c030e8c3236dfe9471b5d200
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::require<enum2$<asset_pipe::texture::BuildTextureAsset> >; module=1651, inline_record=42148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$2::run
// 信息 ID: 823ebb618577482f426ec8df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe027 -> 0xd97450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$2::run::async_fn$0
// 信息 ID: 360a9764163816f5d50d1720
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=76660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$30::clone
// 信息 ID: b8aed8b6b19b1dbe5bf15612
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>; module=501, inline_record=68156; 不是独立调用边。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=77672; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$12::clone<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global>; module=1643, inline_record=264488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$31::hash
// 信息 ID: 26b69414c177145f8b43a541
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::Hash::hash_slice<asset_pipe::texture::TextureLayerAssetDesc,turbosloth::id::Hash128>; module=501, inline_record=61420; 不是独立调用边。
// [PDB 内联宿主] turbosloth::asset::BuildContext::build<asset_pipe::texture::BuildTextureLayer>; module=1651, inline_record=30976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$32::clone
// 信息 ID: 31b3739fdc61e04928fe84bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=66256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$33::hash
// 信息 ID: 1f7e2754564f4b8624e28d35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::require<enum2$<asset_pipe::texture::BuildTextureAsset> >; module=1651, inline_record=41828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$34::clone
// 信息 ID: 1358b8efa3a104a635e02395
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>; module=501, inline_record=68120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$35::hash
// 信息 ID: 111e75d4095deebb9b1dd8d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::build<asset_pipe::texture::BuildTextureLayer>; module=1651, inline_record=30952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$36::clone
// 信息 ID: bb22a4e49254a37d2bb270ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=66180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$37::hash
// 信息 ID: 2eb3edfdb0522dadc741dce8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::require<enum2$<asset_pipe::texture::BuildTextureAsset> >; module=1651, inline_record=41660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$3::run
// 信息 ID: c0ed968fb364ab7b0d6aba36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> @ 0x8fd660 -> 0xd974e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe1c2 -> 0xd974e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$3::run::async_fn$0
// 信息 ID: 1014d90796d844333a2e7c0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer>; module=501, inline_record=71644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$5::run
// 信息 ID: cbce328e7288af8b9e73f808
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fdf34 -> 0xd97490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$5::run::async_fn$0
// 信息 ID: b15ba64cc2e837de634947fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> >; module=501, inline_record=76512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$6::clone
// 信息 ID: 4ca4b2fbf87de961ba17a93f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>; module=1643, inline_record=269824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$7::hash
// 信息 ID: e56a74f8a9d79f8c595651d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::Hash::hash_slice<asset_pipe::texture::TextureAssetListEntry,turbosloth::id::Hash128>; module=501, inline_record=60832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$8::resolve
// 信息 ID: 55077931ac2811de43845180
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rkyv::util::ser_vec::SerVec::with_capacity<asset_pipe::texture::TextureAssetListEntryResolver,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>,enum2$<core::result::Result<usize,rancor::Error> >,rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<asset_pipe::texture::TextureAssetListEntry,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> > > @ 0x17bf764 -> 0xd97390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::resolve_opts
// 信息 ID: 2446cd25daab6f18f56def64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::build_texture_assets::async_fn$0 @ 0x16f192b -> 0xd96d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > @ 0x258d315 -> 0xd96d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a432c4 -> 0xd96d60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::resolve_opts::closure$0
// 信息 ID: ec174eac8276316bee54c648
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::resolve_opts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::resolve_opts; module=687, inline_record=456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::resolve_opts::closure$1
// 信息 ID: 678022e66e838f8c784162f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::resolve_opts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::resolve_opts; module=687, inline_record=648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::source_image_format
// 信息 ID: b350e90db8644c641c24062f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::impl$0::load::async_fn$0 @ 0x8f84fb -> 0xd97100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::source_image_format::closure$0
// 信息 ID: 561f1f598b7442010cff8470
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::source_image_format", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::source_image_format; module=687, inline_record=2140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::source_image_format::closure$1
// 信息 ID: cbd9b276136f9dbac1d3acc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::source_image_format", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::source_image_format @ 0xd97159 -> 0x2da0760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::strip_debug_name
// 信息 ID: 8244c76ff16a9d752edb576d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::BuildTextureAsset::new_single; module=687, inline_record=1344; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::BuildTextureAsset::new_array; module=687, inline_record=1564; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::BuildTextureAsset::new_array; module=687, inline_record=1756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<asset_pipe::texture::build_texture_assets::async_fn_env$0> >
// 信息 ID: 938cbac9d3e6fdf7c930e072
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["asset_pipe::texture::build_texture_assets::async_fn_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<asset_pipe::texture::build_texture_assets::async_fn_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::load_clutter_packs::load_clutter_packs_at_startup::closure_env$1> > >,enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> > > @ 0x16fab9f -> 0x17042b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> >,enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> > >,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > @ 0x16fefb5 -> 0x17042b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7> > >,enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> > > @ 0x1700c0f -> 0x17042b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> > > @ 0x1a86a7a -> 0x17042b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > @ 0x258dcac -> 0x17042b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a44e4f -> 0x17042b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$0::load::async_fn_env$0> >
// 信息 ID: 174806eb8f96d9c1177fbb33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["asset_pipe::texture::impl$0::load::async_fn_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<asset_pipe::texture::impl$0::load::async_fn_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$1::run::async_fn_env$0> >
// 信息 ID: 30f37dd5655ab7f69a938f76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["asset_pipe::texture::impl$1::run::async_fn_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<asset_pipe::texture::impl$1::run::async_fn_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$5::run::async_fn_env$0> > @ 0x8fc247 -> 0x8fc2e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > > > @ 0x8fc66f -> 0x8fc2e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> > > @ 0x8fc73e -> 0x8fc2e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> @ 0x8fdab8 -> 0x8fc2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe694 -> 0x8fc2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$2::run::async_fn_env$0> >
// 信息 ID: 2e36bb603d7a288d173f4de0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["asset_pipe::texture::impl$2::run::async_fn_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<asset_pipe::texture::impl$2::run::async_fn_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$5::run::async_fn_env$0> > @ 0x8fc235 -> 0x8fc010 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > > > @ 0x8fc681 -> 0x8fc010 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fed3f -> 0x8fc010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$3::run::async_fn_env$0> >
// 信息 ID: c31fca1db3fbb6d9c4a45c65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["asset_pipe::texture::impl$3::run::async_fn_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<asset_pipe::texture::impl$3::run::async_fn_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$5::run::async_fn_env$0> > @ 0x8fc247 -> 0x8fc2e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > > > @ 0x8fc66f -> 0x8fc2e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> > > @ 0x8fc73e -> 0x8fc2e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,asset_pipe::texture::BuildTextureLayer> @ 0x8fdab8 -> 0x8fc2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe694 -> 0x8fc2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<asset_pipe::texture::impl$5::run::async_fn_env$0> >
// 信息 ID: efac47cffda452e40a28b117
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["asset_pipe::texture::impl$5::run::async_fn_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<asset_pipe::texture::impl$5::run::async_fn_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
