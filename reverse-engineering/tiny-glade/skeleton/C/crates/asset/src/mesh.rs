// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/mesh.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 393cc85e1dd71ee13c11c73b44ebb93d6e06a0e8fa729e32292ed97fa5c5f11d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mesh.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 727, 1650, 2144, 2146, 2148, 2150, 3620, 3625, 4600, 4603, 4977
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use bytecheck; // namespace candidate
//   package identities: [{"name":"bytecheck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.2"}]
// use core; // namespace candidate
//   package identities: []
// use rkyv; // namespace candidate
//   package identities: [{"name":"rkyv","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.16"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::mesh::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 10, 24, 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1803F
// 首条位置: {"module":2150,"inlinee":98367}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$7::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1650,"record_offset":20124,"rva":24161392,"va":5392870512}
// 编译实例 RVAs: 0x170ac70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0a5f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13633221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392870512,"raw_hex":"e83bf92fff","size":5,"soft_fail":false,"target_va":5379237360,"va":5392870576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14170adb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t97","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392870512,"raw_hex":"e861000000","size":5,"soft_fail":false,"target_va":5392870832,"va":5392870730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rkyv::vec::impl$19::check_bytes
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":20412,"parent_record_offset":20124,"ancestor_record_offset":20124,"inlinee":28271,"source_line":28,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] bytecheck::impl$14::check_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytecheck","version":"0.8.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytecheck-0.8.2/src/lib.rs
//     evidence={"module":1650,"record_offset":21652,"parent_record_offset":21628,"ancestor_record_offset":20124,"inlinee":28505,"source_line":704,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1650,"record_offset":20708,"parent_record_offset":20684,"ancestor_record_offset":20124,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1650,"record_offset":20732,"parent_record_offset":20708,"ancestor_record_offset":20124,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1650,"record_offset":21740,"parent_record_offset":21712,"ancestor_record_offset":20124,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1650,"record_offset":21712,"parent_record_offset":21684,"ancestor_record_offset":20124,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1650,"record_offset":21684,"parent_record_offset":21652,"ancestor_record_offset":20124,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1650,"record_offset":21932,"parent_record_offset":21908,"ancestor_record_offset":20124,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1650,"record_offset":21908,"parent_record_offset":21872,"ancestor_record_offset":20124,"inlinee":28509,"source_line":1645,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1650,"record_offset":20884,"parent_record_offset":20768,"ancestor_record_offset":20124,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":20124,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":20616,"parent_record_offset":20508,"ancestor_record_offset":20124,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":20124,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1650,"record_offset":20856,"parent_record_offset":20768,"ancestor_record_offset":20124,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1650,"record_offset":20684,"parent_record_offset":20656,"ancestor_record_offset":20124,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":20484,"parent_record_offset":20460,"ancestor_record_offset":20124,"inlinee":28277,"source_line":325,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping_raw
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":20508,"parent_record_offset":20484,"ancestor_record_offset":20124,"inlinee":28278,"source_line":236,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":20532,"parent_record_offset":20508,"ancestor_record_offset":20124,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":20460,"parent_record_offset":20432,"ancestor_record_offset":20124,"inlinee":28275,"source_line":606,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::check_subtree_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":1650,"record_offset":20800,"parent_record_offset":20768,"ancestor_record_offset":20124,"inlinee":28495,"source_line":62,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::pop_subtree_range
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":1650,"record_offset":20912,"parent_record_offset":20768,"ancestor_record_offset":20124,"inlinee":28506,"source_line":80,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":1650,"record_offset":21508,"parent_record_offset":21384,"ancestor_record_offset":20124,"inlinee":28498,"source_line":70,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":1650,"record_offset":20656,"parent_record_offset":20432,"ancestor_record_offset":20124,"inlinee":28490,"source_line":138,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree_raw
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":1650,"record_offset":20768,"parent_record_offset":20656,"ancestor_record_offset":20124,"inlinee":28493,"source_line":114,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::validator::impl$12::pop_subtree_range
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/validator.rs
//     evidence={"module":1650,"record_offset":20960,"parent_record_offset":20936,"ancestor_record_offset":20124,"inlinee":28508,"source_line":176,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::validator::impl$12::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/validator.rs
//     evidence={"module":1650,"record_offset":21556,"parent_record_offset":21532,"ancestor_record_offset":20124,"inlinee":28501,"source_line":158,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::check_subtree_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":1650,"record_offset":20824,"parent_record_offset":20800,"ancestor_record_offset":20124,"inlinee":28497,"source_line":32,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::pop_subtree_range
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":1650,"record_offset":20936,"parent_record_offset":20912,"ancestor_record_offset":20124,"inlinee":28507,"source_line":50,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":1650,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":20124,"inlinee":28499,"source_line":40,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":20432,"parent_record_offset":20412,"ancestor_record_offset":20124,"inlinee":28273,"source_line":304,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":21628,"parent_record_offset":21384,"ancestor_record_offset":20124,"inlinee":28503,"source_line":310,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$9::deserialize<rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1650,"record_offset":268,"rva":24155264,"va":5392864384}
// 编译实例 RVAs: 0x1709480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-17185771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392864384,"raw_hex":"e815c4f9fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5392864454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-17185939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392864384,"raw_hex":"e86dc3f9fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5392864638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24142254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392864384,"raw_hex":"e8529e8ffe","size":5,"soft_fail":false,"target_va":5368722192,"va":5392864441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141709dd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392864384,"raw_hex":"e878080000","size":5,"soft_fail":false,"target_va":5392866768,"va":5392864595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24192567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392864384,"raw_hex":"e837267101","size":5,"soft_fail":false,"target_va":5417057056,"va":5392864484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23756224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392864384,"raw_hex":"e8c07d6a01","size":5,"soft_fail":false,"target_va":5416620769,"va":5392864540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":1432,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1650,"record_offset":1688,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1650,"record_offset":1404,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$2::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":1650,"record_offset":464,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":28517,"source_line":45,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":668,"parent_record_offset":464,"ancestor_record_offset":268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":1560,"parent_record_offset":1504,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1650,"record_offset":1532,"parent_record_offset":1504,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1650,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1650,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1650,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1650,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":268,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":848,"parent_record_offset":816,"ancestor_record_offset":268,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":268,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":268,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$15::deserialize_unsized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1650,"record_offset":928,"parent_record_offset":464,"ancestor_record_offset":268,"inlinee":28442,"source_line":335,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1650,"record_offset":584,"parent_record_offset":464,"ancestor_record_offset":268,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":268,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":268,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":268,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":268,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":520,"parent_record_offset":464,"ancestor_record_offset":268,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":548,"parent_record_offset":520,"ancestor_record_offset":268,"inlinee":28519,"source_line":45,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$15::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1650,"record_offset":21996,"rva":24161712,"va":5392870832}
// 编译实例 RVAs: 0x170adb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0a5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13633623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392870832,"raw_hex":"e8a9f72fff","size":5,"soft_fail":false,"target_va":5379237360,"va":5392870978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11207844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392870832,"raw_hex":"e85cfb54ff","size":5,"soft_fail":false,"target_va":5381663008,"va":5392870847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::mesh::_::impl$1::check_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/mesh.rs
//     evidence={"module":1650,"record_offset":22316,"parent_record_offset":21996,"ancestor_record_offset":21996,"inlinee":81669,"source_line":34,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1650,"record_offset":22288,"parent_record_offset":21996,"ancestor_record_offset":21996,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1650,"record_offset":22632,"parent_record_offset":22608,"ancestor_record_offset":21996,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1650,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":21996,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":21996,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":22544,"parent_record_offset":22436,"ancestor_record_offset":21996,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":21996,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1650,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":21996,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":21996,"inlinee":28277,"source_line":325,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":22436,"parent_record_offset":22412,"ancestor_record_offset":21996,"inlinee":28278,"source_line":236,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":21996,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":22388,"parent_record_offset":22360,"ancestor_record_offset":21996,"inlinee":28275,"source_line":606,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":1650,"record_offset":22584,"parent_record_offset":22360,"ancestor_record_offset":21996,"inlinee":28490,"source_line":138,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::impl$19::check_bytes
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":22340,"parent_record_offset":22316,"ancestor_record_offset":21996,"inlinee":28271,"source_line":28,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":22360,"parent_record_offset":22340,"ancestor_record_offset":21996,"inlinee":28273,"source_line":304,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$17::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13EFE
// 首条位置: {"module":1650,"inlinee":81662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::mesh::impl$22::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/mesh.rs
//     evidence={"module":1650,"record_offset":10128,"parent_record_offset":10068,"ancestor_record_offset":10068,"inlinee":81664,"source_line":36,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":11564,"parent_record_offset":10068,"ancestor_record_offset":10068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1650,"record_offset":10096,"parent_record_offset":10068,"ancestor_record_offset":10068,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":10068,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":11764,"parent_record_offset":11740,"ancestor_record_offset":10068,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":10260,"parent_record_offset":10156,"ancestor_record_offset":10068,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":11788,"parent_record_offset":11764,"ancestor_record_offset":10068,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":11716,"parent_record_offset":11660,"ancestor_record_offset":10068,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1650,"record_offset":11688,"parent_record_offset":11660,"ancestor_record_offset":10068,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1650,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":10068,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1650,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":10068,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1650,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":10068,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":10068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":10436,"parent_record_offset":10412,"ancestor_record_offset":10068,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":10548,"parent_record_offset":10516,"ancestor_record_offset":10068,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":10068,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":10068,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1650,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":10068,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::alloc::vec::impl$2::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":1650,"record_offset":10156,"parent_record_offset":10128,"ancestor_record_offset":10068,"inlinee":28517,"source_line":45,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$15::deserialize_unsized
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1650,"record_offset":10516,"parent_record_offset":10156,"ancestor_record_offset":10068,"inlinee":28442,"source_line":335,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":10068,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":10068,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":10068,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":10340,"parent_record_offset":10316,"ancestor_record_offset":10068,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":10068,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":10204,"parent_record_offset":10156,"ancestor_record_offset":10068,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":10228,"parent_record_offset":10204,"ancestor_record_offset":10068,"inlinee":28519,"source_line":45,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::_::impl$0::resolve
// 记录形式: inlinee_declaration; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA75
// 首条位置: {"module":727,"inlinee":47733}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::_::impl$1::check_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F05
// 首条位置: {"module":1650,"inlinee":81669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rkyv::vec::impl$19::check_bytes
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":22340,"parent_record_offset":22316,"ancestor_record_offset":22316,"inlinee":28271,"source_line":28,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1650,"record_offset":22632,"parent_record_offset":22608,"ancestor_record_offset":22316,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1650,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":22316,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":22316,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":22544,"parent_record_offset":22436,"ancestor_record_offset":22316,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":22316,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1650,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":22316,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":22316,"inlinee":28277,"source_line":325,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":22436,"parent_record_offset":22412,"ancestor_record_offset":22316,"inlinee":28278,"source_line":236,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":22316,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":22388,"parent_record_offset":22360,"ancestor_record_offset":22316,"inlinee":28275,"source_line":606,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":1650,"record_offset":22584,"parent_record_offset":22360,"ancestor_record_offset":22316,"inlinee":28490,"source_line":138,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":22360,"parent_record_offset":22340,"ancestor_record_offset":22316,"inlinee":28273,"source_line":304,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$22::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F00
// 首条位置: {"module":1650,"inlinee":81664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$2::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":1650,"record_offset":10156,"parent_record_offset":10128,"ancestor_record_offset":10128,"inlinee":28517,"source_line":45,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1650,"record_offset":10260,"parent_record_offset":10156,"ancestor_record_offset":10128,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1650,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":10128,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":10436,"parent_record_offset":10412,"ancestor_record_offset":10128,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1650,"record_offset":10548,"parent_record_offset":10516,"ancestor_record_offset":10128,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":10128,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1650,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":10128,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1650,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":10128,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$15::deserialize_unsized
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1650,"record_offset":10516,"parent_record_offset":10156,"ancestor_record_offset":10128,"inlinee":28442,"source_line":335,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":10128,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":10128,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":10128,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1650,"record_offset":10340,"parent_record_offset":10316,"ancestor_record_offset":10128,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":10128,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":10204,"parent_record_offset":10156,"ancestor_record_offset":10128,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1650,"record_offset":10228,"parent_record_offset":10204,"ancestor_record_offset":10128,"inlinee":28519,"source_line":45,"archive":"_pdb/C/modules/01650.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$23::visit_asset_dependencies
// 记录形式: inlinee_declaration; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA77
// 首条位置: {"module":727,"inlinee":47735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$0::vertex_count::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA69
// 首条位置: {"module":727,"inlinee":47721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::MeshData::attribute
// 记录形式: inlinee_declaration; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA6B
// 首条位置: {"module":727,"inlinee":47723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$0::attribute::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA6D
// 首条位置: {"module":727,"inlinee":47725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$0::attribute::closure$2
// 记录形式: inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA6E
// 首条位置: {"module":727,"inlinee":47726}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$0::optional_attribute::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA70
// 首条位置: {"module":727,"inlinee":47728}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$0::insert_attribute::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1803E
// 首条位置: {"module":2148,"inlinee":98366}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$0::insert_default_attribute::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA72
// 首条位置: {"module":727,"inlinee":47730}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::MeshData::sort_attributes
// 记录形式: inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA73
// 首条位置: {"module":727,"inlinee":47731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::mesh::impl$0::sort_attributes::closure$0
// 记录形式: inlinee_declaration; instances=5
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18038
// 首条位置: {"module":2146,"inlinee":98360}
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
// 完整结构及原始类型记录: mesh.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::mesh::ArchivedMeshAttribute
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$15::deserialize_unsized<asset::mesh::ArchivedMeshAttribute,asset::mesh::MeshAttribute,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >", "constructor": "rkyv::impls::core::impl$15::deserialize_unsized", "arguments": ["asset::mesh::ArchivedMeshAttribute", "asset::mesh::MeshAttribute", "rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::mesh::ArchivedMeshData
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::checked::check_pos_with_context::closure_env$0<asset::mesh::ArchivedMeshData,rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>", "constructor": "rkyv::api::checked::check_pos_with_context::closure_env$0", "arguments": ["asset::mesh::ArchivedMeshData", "rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>", "rancor::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::validation::archive::impl$1::in_subtree<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>,rancor::Error,asset::mesh::ArchivedMeshData,tuple$<>,rkyv::api::checked::check_pos_with_context::closure_env$0<asset::mesh::ArchivedMeshData,rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >", "constructor": "rkyv::validation::archive::impl$1::in_subtree", "arguments": ["rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>", "rancor::Error", "asset::mesh::ArchivedMeshData", "tuple$<>", "rkyv::api::checked::check_pos_with_context::closure_env$0<asset::mesh::ArchivedMeshData,rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::mesh::MeshAttribute
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$15::deserialize_unsized<asset::mesh::ArchivedMeshAttribute,asset::mesh::MeshAttribute,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >", "constructor": "rkyv::impls::core::impl$15::deserialize_unsized", "arguments": ["asset::mesh::ArchivedMeshAttribute", "asset::mesh::MeshAttribute", "rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::mesh::MeshAttribute", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::mesh::MeshAttributeValues
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum2$<asset::mesh::MeshAttributeValues>", "constructor": "enum2$", "arguments": ["asset::mesh::MeshAttributeValues"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<asset::mesh::MeshAttributeValues> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<asset::mesh::MeshAttributeValues>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::mesh::MeshData
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::mesh::MeshData>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::mesh::MeshData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::type_name<asset::mesh::MeshData>", "constructor": "core::any::type_name", "arguments": ["asset::mesh::MeshData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::high::checked::from_bytes<asset::mesh::MeshData,rancor::Error>", "constructor": "rkyv::api::high::checked::from_bytes", "arguments": ["asset::mesh::MeshData", "rancor::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::read_packaged_asset<asset::mesh::MeshData,ref$<std::path::PathBuf> >", "constructor": "turbosloth::store::read_packaged_asset", "arguments": ["asset::mesh::MeshData", "ref$<std::path::PathBuf>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::export::to_json_value<asset::mesh::MeshData>", "constructor": "turbosloth::export::to_json_value", "arguments": ["asset::mesh::MeshData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::AssetRef<asset::mesh::MeshData>", "constructor": "turbosloth::asset::AssetRef", "arguments": ["asset::mesh::MeshData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<turbosloth::asset::AssetRef<asset::mesh::MeshData> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["turbosloth::asset::AssetRef<asset::mesh::MeshData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mesh.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mesh.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::history::HistoryChainSnapshot (*)(country_core::resources::history::ResolvedHistoryChainSnapshot)
// 候选参数类型: ["country_core::resources::history::ResolvedHistoryChainSnapshot"]

// ------------------------------------------------------------------
// 函数: asset::mesh::MeshData::attribute
// 信息 ID: eb4cb5570a76676f65d35854
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::MeshData::sort_attributes
// 信息 ID: 850daa81a5dbc19096af0690
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::_::impl$0::resolve
// 信息 ID: 608ce7e9ba96145d64dab943
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::_::impl$0::serialize
// 信息 ID: bc936c92aa495cb4136ce460
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::export::to_json_value<asset::mesh::MeshData>; module=2150, inline_record=568; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> > >; module=3625, inline_record=5280; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<enum2$<asset::mesh::MeshAttributeValues> >; module=3625, inline_record=76320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::_::impl$1::check_bytes
// 信息 ID: a165b9bfaa8161f2dd6eb9f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::mesh::impl$15::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >; module=1650, inline_record=22316; 不是独立调用边。
// [PDB 内联宿主] asset::mesh::impl$15::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >; module=2148, inline_record=8588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$0::attribute::closure$0
// 信息 ID: 4484baca8a43db18d955afe3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::mesh::impl$0::attribute", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$0::attribute::closure$2
// 信息 ID: db77679e89588afb792d4f99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset::mesh::impl$0::attribute", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$0::insert_attribute::closure$0
// 信息 ID: fc3ae3b709810d15c51b3b08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::mesh::impl$0::insert_attribute", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$0::insert_default_attribute::closure$0
// 信息 ID: ca1ad44cd5edffa5567382c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::mesh::impl$0::insert_default_attribute", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$0::optional_attribute::closure$0
// 信息 ID: 1ba46cda4b3cb7058f8d6751
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::mesh::impl$0::optional_attribute", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$0::sort_attributes::closure$0
// 信息 ID: 79a64b93ed409b80d4de9177
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::mesh::impl$0::sort_attributes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$0::vertex_count::closure$0
// 信息 ID: b3888bee6dc3b38d69b73650
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::mesh::impl$0::vertex_count", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$15::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 信息 ID: 165fb4d2fc83d666a7cb962a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rkyv::validation::Validator -> ["rkyv::validation::archive::validator::ArchiveValidator", "rkyv::validation::shared::validator::SharedValidator"]; 字段布局 = unknown。
// [类型关系] rancor::Strategy -> ["rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>", "rancor::Error"]; 字段布局 = unknown。
// [类型关系] asset::mesh::impl$15::check_bytes -> ["rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::mesh::impl$7::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > | asset::mesh::impl$7::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > @ 0x170ad4a -> 0x170adb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$17::deserialize
// 信息 ID: bd77c16cb183ba4cb717fc67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::mesh::ArchivedMeshAttribute,asset::mesh::MeshAttribute,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=1650, inline_record=10068; 不是独立调用边。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::mesh::ArchivedMeshAttribute,asset::mesh::MeshAttribute,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=2148, inline_record=4608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$22::deserialize
// 信息 ID: ffce1b44c1c1300a2a225759
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::mesh::ArchivedMeshAttribute,asset::mesh::MeshAttribute,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=1650, inline_record=10128; 不是独立调用边。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::mesh::ArchivedMeshAttribute,asset::mesh::MeshAttribute,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=2148, inline_record=4668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$23::visit_asset_dependencies
// 信息 ID: 786aeb57ef00c084957b8fa2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$7::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 信息 ID: 9105c4e5037f3fe97925902e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rkyv::validation::Validator -> ["rkyv::validation::archive::validator::ArchiveValidator", "rkyv::validation::shared::validator::SharedValidator"]; 字段布局 = unknown。
// [类型关系] rancor::Strategy -> ["rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>", "rancor::Error"]; 字段布局 = unknown。
// [类型关系] asset::mesh::impl$7::check_bytes -> ["rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rkyv::validation::archive::impl$1::in_subtree<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>,rancor::Error,asset::mesh::ArchivedMeshData,tuple$<>,rkyv::api::checked::check_pos_with_context::closure_env$0<asset::mesh::ArchivedMeshData,rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > | rkyv::validation::archive::impl$1::in_subtree<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>,rancor::Error,asset::mesh::ArchivedMeshData,tuple$<>,rkyv::api::checked::check_pos_with_context::closure_env$0<asset::mesh::ArchivedMeshData,rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > @ 0x17dbee8 -> 0x170ac70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::mesh::impl$9::deserialize<rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >
// 信息 ID: a2520217d9d7602f3fdf1362
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rancor::Strategy -> ["rkyv::de::pooling::alloc::Pool", "rancor::Error"]; 字段布局 = unknown。
// [类型关系] asset::mesh::impl$9::deserialize -> ["rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rkyv::api::high::checked::from_bytes<asset::mesh::MeshData,rancor::Error> @ 0x17d1765 -> 0x1709480 (direct_call); 同地址别名未裁定。
// [静态直接引用] rkyv::api::high::checked::from_bytes<asset::mesh::MeshData,rancor::Error> @ 0x2728535 -> 0x1709480 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
