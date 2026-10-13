// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/save/shadow_file.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 24782b2394db3be66260201fbbd22a6893613f5c812762133b285204054b977a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shadow_file.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 485, 505, 545, 565, 1585, 1686
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bitvec; // namespace candidate
//   package identities: [{"name":"bitvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use radium; // namespace candidate
//   package identities: [{"name":"radium","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.0"}]
// use std; // namespace candidate
//   package identities: []
// use tap; // namespace candidate
//   package identities: [{"name":"tap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use tempfile; // namespace candidate
//   package identities: [{"name":"tempfile","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.20.0"}]
// use wyz; // namespace candidate
//   package identities: [{"name":"wyz","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::create
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":505,"record_offset":33844,"rva":9536720,"va":5378245840}
// 编译实例 RVAs: 0x9184d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4398190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e86e1c4300","size":5,"soft_fail":false,"target_va":5382644528,"va":5378246333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tempfile..file..TempPath$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2536630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e8b6b42600","size":5,"soft_fail":false,"target_va":5380783824,"va":5378247189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8808723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e8ed9679ff","size":5,"soft_fail":false,"target_va":5369438848,"va":5378247566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402860b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6893932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e894ce96ff","size":5,"soft_fail":false,"target_va":5371355312,"va":5378249239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=7
//     target_aliases=92 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3347608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e868ebccff","size":5,"soft_fail":false,"target_va":5374900736,"va":5378248339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ecc10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3326685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e8233dcdff","size":5,"soft_fail":false,"target_va":5374921744,"va":5378248424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3165720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e8e8b1cfff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378247219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2730114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e87e57d6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5378246141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140914c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e8fcc4ffff","size":5,"soft_fail":false,"target_va":5378231440,"va":5378246543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac5e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1759423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e8bfd81a00","size":5,"soft_fail":false,"target_va":5380005488,"va":5378246060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d597e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4460443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e89b0f4400","size":5,"soft_fail":false,"target_va":5382707168,"va":5378246720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4494109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e81d934400","size":5,"soft_fail":false,"target_va":5382740736,"va":5378246622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a4cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13157609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e8e9c4c800","size":5,"soft_fail":false,"target_va":5391404208,"va":5378246594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8e80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33097523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e83307f901","size":5,"soft_fail":false,"target_va":5411344000,"va":5378246472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4405518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e80e394300","size":5,"soft_fail":false,"target_va":5382655024,"va":5378249501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bitvec::vec::BitVec::resize<usize,bitvec::order::Lsb0>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e89ed1ffff","size":5,"soft_fail":false,"target_va":5378235424,"va":5378247293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4448279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e817e04300","size":5,"soft_fail":false,"target_va":5382694656,"va":5378246372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::error::Error::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2534468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e844ac2600","size":5,"soft_fail":false,"target_va":5380781152,"va":5378246679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4498145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378245840,"raw_hex":"e8e1a24400","size":5,"soft_fail":false,"target_va":5382744080,"va":5378245930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*42441131(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378245840,"raw_hex":"ff15ab998702","size":6,"soft_fail":false,"va":5378247231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":34440,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::traits::impl$24::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/traits.rs
//     evidence={"module":505,"record_offset":35892,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":28674,"source_line":346,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":34232,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":505,"record_offset":35948,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":6245,"source_line":3197,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":35252,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":34792,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":34264,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":34148,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":34036,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":35980,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] tempfile::Builder::make_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":34120,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":28670,"source_line":742,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":34064,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":7537,"source_line":314,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] tempfile::file::impl$22::write_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/file/mod.rs
//     evidence={"module":505,"record_offset":34552,"parent_record_offset":33844,"ancestor_record_offset":33844,"inlinee":7555,"source_line":983,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":505,"record_offset":35388,"parent_record_offset":35364,"ancestor_record_offset":33844,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":505,"record_offset":35364,"parent_record_offset":35336,"ancestor_record_offset":33844,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":505,"record_offset":35336,"parent_record_offset":35308,"ancestor_record_offset":33844,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":505,"record_offset":37112,"parent_record_offset":37084,"ancestor_record_offset":33844,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":505,"record_offset":37084,"parent_record_offset":37060,"ancestor_record_offset":33844,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":505,"record_offset":37060,"parent_record_offset":37036,"ancestor_record_offset":33844,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":505,"record_offset":34512,"parent_record_offset":34472,"ancestor_record_offset":33844,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":34472,"parent_record_offset":34440,"ancestor_record_offset":33844,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":505,"record_offset":35916,"parent_record_offset":35892,"ancestor_record_offset":33844,"inlinee":28675,"source_line":64,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":505,"record_offset":34692,"parent_record_offset":34648,"ancestor_record_offset":33844,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":34320,"parent_record_offset":34288,"ancestor_record_offset":33844,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":34348,"parent_record_offset":34288,"ancestor_record_offset":33844,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":35280,"parent_record_offset":35252,"ancestor_record_offset":33844,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":34616,"parent_record_offset":34584,"ancestor_record_offset":33844,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":34172,"parent_record_offset":34148,"ancestor_record_offset":33844,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":34196,"parent_record_offset":34172,"ancestor_record_offset":33844,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::impl$1::create::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":505,"record_offset":34816,"parent_record_offset":34792,"ancestor_record_offset":33844,"inlinee":28672,"source_line":48,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":34376,"parent_record_offset":34288,"ancestor_record_offset":33844,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":34288,"parent_record_offset":34264,"ancestor_record_offset":33844,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":34400,"parent_record_offset":34376,"ancestor_record_offset":33844,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":34740,"parent_record_offset":34716,"ancestor_record_offset":33844,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":505,"record_offset":35676,"parent_record_offset":35652,"ancestor_record_offset":33844,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":34716,"parent_record_offset":34692,"ancestor_record_offset":33844,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::error::impl$2::with_err_path
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/error.rs
//     evidence={"module":505,"record_offset":34584,"parent_record_offset":34552,"ancestor_record_offset":33844,"inlinee":7527,"source_line":30,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::error::impl$2::with_err_path::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/error.rs
//     evidence={"module":505,"record_offset":34648,"parent_record_offset":34616,"ancestor_record_offset":33844,"inlinee":7529,"source_line":35,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":34088,"parent_record_offset":34064,"ancestor_record_offset":33844,"inlinee":7539,"source_line":235,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowBackendFile::unwrap_open_mut
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x700E
// 首条位置: {"module":505,"inlinee":28686}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":505,"record_offset":42740,"parent_record_offset":42708,"ancestor_record_offset":42708,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::impl$1::create::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7000
// 首条位置: {"module":505,"inlinee":28672}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::write_at
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7026
// 首条位置: {"module":1686,"inlinee":28710}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":75272,"parent_record_offset":75072,"ancestor_record_offset":75072,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":75112,"parent_record_offset":75072,"ancestor_record_offset":75072,"inlinee":14156,"source_line":293,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":75232,"parent_record_offset":75140,"ancestor_record_offset":75072,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75140,"parent_record_offset":75112,"ancestor_record_offset":75072,"inlinee":14148,"source_line":593,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75196,"parent_record_offset":75168,"ancestor_record_offset":75072,"inlinee":14150,"source_line":2350,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75168,"parent_record_offset":75140,"ancestor_record_offset":75072,"inlinee":14149,"source_line":2431,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::truncate
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7027
// 首条位置: {"module":1686,"inlinee":28711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1686,"record_offset":78368,"parent_record_offset":78148,"ancestor_record_offset":78148,"inlinee":28645,"source_line":359,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":78204,"parent_record_offset":78148,"ancestor_record_offset":78148,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":78332,"parent_record_offset":78148,"ancestor_record_offset":78148,"inlinee":28676,"source_line":3197,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":78532,"parent_record_offset":78148,"ancestor_record_offset":78148,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":78696,"parent_record_offset":78668,"ancestor_record_offset":78148,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":78668,"parent_record_offset":78608,"ancestor_record_offset":78148,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":78636,"parent_record_offset":78608,"ancestor_record_offset":78148,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":78608,"parent_record_offset":78580,"ancestor_record_offset":78148,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":78580,"parent_record_offset":78556,"ancestor_record_offset":78148,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1686,"record_offset":78428,"parent_record_offset":78400,"ancestor_record_offset":78148,"inlinee":11197,"source_line":333,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1686,"record_offset":78492,"parent_record_offset":78464,"ancestor_record_offset":78148,"inlinee":25488,"source_line":350,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1686,"record_offset":78400,"parent_record_offset":78368,"ancestor_record_offset":78148,"inlinee":25242,"source_line":774,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":1686,"record_offset":78464,"parent_record_offset":78368,"ancestor_record_offset":78148,"inlinee":25487,"source_line":585,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":78232,"parent_record_offset":78204,"ancestor_record_offset":78148,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":78556,"parent_record_offset":78532,"ancestor_record_offset":78148,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":78260,"parent_record_offset":78232,"ancestor_record_offset":78148,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":78292,"parent_record_offset":78232,"ancestor_record_offset":78148,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::data
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7028
// 首条位置: {"module":1686,"inlinee":28712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::flush
// 记录形式: compiled_procedure; instances=1
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":505,"record_offset":38100,"rva":9540560,"va":5378249680}
// 编译实例 RVAs: 0x9193d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4393517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e82d0a4300","size":5,"soft_fail":false,"target_va":5382644528,"va":5378251006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..File$u20$as$u20$std..io..Seek$GT$::seek
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4458614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e876084400","size":5,"soft_fail":false,"target_va":5382708576,"va":5378249957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tempfile..file..TempPath$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2532459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e86ba42600","size":5,"soft_fail":false,"target_va":5380783824,"va":5378251360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8813108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e8cc8579ff","size":5,"soft_fail":false,"target_va":5369438848,"va":5378251951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4622664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e8b876b9ff","size":5,"soft_fail":false,"target_va":5373629488,"va":5378252147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=9
//     target_aliases=92 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3351806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e802dbccff","size":5,"soft_fail":false,"target_va":5374900736,"va":5378252537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebb30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3335028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e88c1ccdff","size":5,"soft_fail":false,"target_va":5374917424,"va":5378252447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ecc10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3331602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e8ee29cdff","size":5,"soft_fail":false,"target_va":5374921744,"va":5378253341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3169891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e89da1cfff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378251390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2734822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e81a45d6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5378250849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140914c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e86eb7ffff","size":5,"soft_fail":false,"target_va":5378231440,"va":5378250013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac5e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1755318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e8b6c81a00","size":5,"soft_fail":false,"target_va":5380005488,"va":5378250165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d597e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4455929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e8f9fd4300","size":5,"soft_fail":false,"target_va":5382707168,"va":5378251234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4490296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e838844400","size":5,"soft_fail":false,"target_va":5382740736,"va":5378250435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4577621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e855d94500","size":5,"soft_fail":false,"target_va":5382827728,"va":5378250102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fe920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9327857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e8f1548e00","size":5,"soft_fail":false,"target_va":5387577632,"va":5378249770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a4cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13153795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e803b6c800","size":5,"soft_fail":false,"target_va":5391404208,"va":5378250408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33089272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e8f8e6f801","size":5,"soft_fail":false,"target_va":5411341616,"va":5378252339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8e80
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33093705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e849f8f801","size":5,"soft_fail":false,"target_va":5411344000,"va":5378250290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4403585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e881314300","size":5,"soft_fail":false,"target_va":5382655024,"va":5378251434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::save::shadow_file::ShadowBackendFile> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e831b2ffff","size":5,"soft_fail":false,"target_va":5378232064,"va":5378251978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33290742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e8f6f9fb01","size":5,"soft_fail":false,"target_va":5411543120,"va":5378252373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t4444448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e820d14300","size":5,"soft_fail":false,"target_va":5382694656,"va":5378250203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::File::set_len
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4456206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e80eff4300","size":5,"soft_fail":false,"target_va":5382707232,"va":5378251021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::error::Error::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2530668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378249680,"raw_hex":"e86c9d2600","size":5,"soft_fail":false,"target_va":5380781152,"va":5378250479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*42436960(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378249680,"raw_hex":"ff1560898702","size":6,"soft_fail":false,"va":5378251402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":42152,"parent_record_offset":38100,"ancestor_record_offset":38100,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":42220,"parent_record_offset":38100,"ancestor_record_offset":38100,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::flush_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":505,"record_offset":38252,"parent_record_offset":38100,"ancestor_record_offset":38100,"inlinee":28678,"source_line":121,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":505,"record_offset":41252,"parent_record_offset":41228,"ancestor_record_offset":38100,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":505,"record_offset":41228,"parent_record_offset":41200,"ancestor_record_offset":38100,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":505,"record_offset":41200,"parent_record_offset":41172,"ancestor_record_offset":38100,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":505,"record_offset":38748,"parent_record_offset":38700,"ancestor_record_offset":38100,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":38700,"parent_record_offset":38664,"ancestor_record_offset":38100,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":38664,"parent_record_offset":38520,"ancestor_record_offset":38100,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":40904,"parent_record_offset":40880,"ancestor_record_offset":38100,"inlinee":11197,"source_line":333,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":40960,"parent_record_offset":40936,"ancestor_record_offset":38100,"inlinee":25488,"source_line":350,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::iter::impl$15::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/iter.rs
//     evidence={"module":505,"record_offset":38480,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":28680,"source_line":2158,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":505,"record_offset":40828,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":28684,"source_line":759,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":505,"record_offset":40880,"parent_record_offset":40852,"ancestor_record_offset":38100,"inlinee":25242,"source_line":774,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":505,"record_offset":40936,"parent_record_offset":40852,"ancestor_record_offset":38100,"inlinee":25487,"source_line":585,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":505,"record_offset":40852,"parent_record_offset":40828,"ancestor_record_offset":38100,"inlinee":28645,"source_line":359,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":38788,"parent_record_offset":38520,"ancestor_record_offset":38100,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":40496,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":38632,"parent_record_offset":38608,"ancestor_record_offset":38100,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$1::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":505,"record_offset":39184,"parent_record_offset":39156,"ancestor_record_offset":38100,"inlinee":14147,"source_line":715,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":505,"record_offset":39640,"parent_record_offset":39596,"ancestor_record_offset":38100,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":505,"record_offset":41936,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":505,"record_offset":38880,"parent_record_offset":38844,"ancestor_record_offset":38100,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":505,"record_offset":38608,"parent_record_offset":38520,"ancestor_record_offset":38100,"inlinee":28683,"source_line":147,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":40588,"parent_record_offset":40556,"ancestor_record_offset":38100,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":40616,"parent_record_offset":40556,"ancestor_record_offset":38100,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":41116,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":39564,"parent_record_offset":39536,"ancestor_record_offset":38100,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38932,"parent_record_offset":38908,"ancestor_record_offset":38100,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38908,"parent_record_offset":38844,"ancestor_record_offset":38100,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38816,"parent_record_offset":38520,"ancestor_record_offset":38100,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38844,"parent_record_offset":38816,"ancestor_record_offset":38100,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":40428,"parent_record_offset":40400,"ancestor_record_offset":38100,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":40456,"parent_record_offset":40428,"ancestor_record_offset":38100,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::impl$1::flush_impl::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":505,"record_offset":38520,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":28682,"source_line":157,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":40644,"parent_record_offset":40556,"ancestor_record_offset":38100,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":40532,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":40556,"parent_record_offset":40532,"ancestor_record_offset":38100,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":40668,"parent_record_offset":40644,"ancestor_record_offset":38100,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":40400,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":39688,"parent_record_offset":39664,"ancestor_record_offset":38100,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":39280,"parent_record_offset":39256,"ancestor_record_offset":38100,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":39328,"parent_record_offset":39304,"ancestor_record_offset":38100,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":39304,"parent_record_offset":39280,"ancestor_record_offset":38100,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":505,"record_offset":41512,"parent_record_offset":41488,"ancestor_record_offset":38100,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":39256,"parent_record_offset":39232,"ancestor_record_offset":38100,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":39664,"parent_record_offset":39640,"ancestor_record_offset":38100,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":39232,"parent_record_offset":39208,"ancestor_record_offset":38100,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":39208,"parent_record_offset":39184,"ancestor_record_offset":38100,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::make_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":39156,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":28670,"source_line":742,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":39100,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":7537,"source_line":314,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::error::impl$2::with_err_path
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/error.rs
//     evidence={"module":505,"record_offset":39536,"parent_record_offset":39508,"ancestor_record_offset":38100,"inlinee":7527,"source_line":30,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::error::impl$2::with_err_path::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/error.rs
//     evidence={"module":505,"record_offset":39596,"parent_record_offset":39564,"ancestor_record_offset":38100,"inlinee":7529,"source_line":35,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::file::impl$22::write_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/file/mod.rs
//     evidence={"module":505,"record_offset":39508,"parent_record_offset":38252,"ancestor_record_offset":38100,"inlinee":7555,"source_line":983,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":39124,"parent_record_offset":39100,"ancestor_record_offset":38100,"inlinee":7539,"source_line":235,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::flush_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7006
// 首条位置: {"module":505,"inlinee":28678}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":39384,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] bitvec::slice::iter::impl$15::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/iter.rs
//     evidence={"module":505,"record_offset":38480,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":28680,"source_line":2158,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::BitVec::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":505,"record_offset":40828,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":28684,"source_line":759,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":40496,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":505,"record_offset":41936,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":41116,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::impl$1::flush_impl::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":505,"record_offset":38520,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":28682,"source_line":157,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":40532,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":40400,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] tempfile::Builder::make_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":39156,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":28670,"source_line":742,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":39100,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":7537,"source_line":314,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] tempfile::file::impl$22::write_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/file/mod.rs
//     evidence={"module":505,"record_offset":39508,"parent_record_offset":38252,"ancestor_record_offset":38252,"inlinee":7555,"source_line":983,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":505,"record_offset":41252,"parent_record_offset":41228,"ancestor_record_offset":38252,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":505,"record_offset":41228,"parent_record_offset":41200,"ancestor_record_offset":38252,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":505,"record_offset":41200,"parent_record_offset":41172,"ancestor_record_offset":38252,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":505,"record_offset":38748,"parent_record_offset":38700,"ancestor_record_offset":38252,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":38700,"parent_record_offset":38664,"ancestor_record_offset":38252,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":38664,"parent_record_offset":38520,"ancestor_record_offset":38252,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":40904,"parent_record_offset":40880,"ancestor_record_offset":38252,"inlinee":11197,"source_line":333,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":40960,"parent_record_offset":40936,"ancestor_record_offset":38252,"inlinee":25488,"source_line":350,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":505,"record_offset":40880,"parent_record_offset":40852,"ancestor_record_offset":38252,"inlinee":25242,"source_line":774,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":505,"record_offset":40936,"parent_record_offset":40852,"ancestor_record_offset":38252,"inlinee":25487,"source_line":585,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":505,"record_offset":40852,"parent_record_offset":40828,"ancestor_record_offset":38252,"inlinee":28645,"source_line":359,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":38788,"parent_record_offset":38520,"ancestor_record_offset":38252,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":38632,"parent_record_offset":38608,"ancestor_record_offset":38252,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$1::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":505,"record_offset":39184,"parent_record_offset":39156,"ancestor_record_offset":38252,"inlinee":14147,"source_line":715,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":505,"record_offset":39640,"parent_record_offset":39596,"ancestor_record_offset":38252,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":505,"record_offset":38880,"parent_record_offset":38844,"ancestor_record_offset":38252,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":505,"record_offset":38608,"parent_record_offset":38520,"ancestor_record_offset":38252,"inlinee":28683,"source_line":147,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":40588,"parent_record_offset":40556,"ancestor_record_offset":38252,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":40616,"parent_record_offset":40556,"ancestor_record_offset":38252,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":41144,"parent_record_offset":41116,"ancestor_record_offset":38252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":39564,"parent_record_offset":39536,"ancestor_record_offset":38252,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38932,"parent_record_offset":38908,"ancestor_record_offset":38252,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38908,"parent_record_offset":38844,"ancestor_record_offset":38252,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38816,"parent_record_offset":38520,"ancestor_record_offset":38252,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38844,"parent_record_offset":38816,"ancestor_record_offset":38252,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":40428,"parent_record_offset":40400,"ancestor_record_offset":38252,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":40456,"parent_record_offset":40428,"ancestor_record_offset":38252,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":40644,"parent_record_offset":40556,"ancestor_record_offset":38252,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":40556,"parent_record_offset":40532,"ancestor_record_offset":38252,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":40668,"parent_record_offset":40644,"ancestor_record_offset":38252,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":39688,"parent_record_offset":39664,"ancestor_record_offset":38252,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":39280,"parent_record_offset":39256,"ancestor_record_offset":38252,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":39328,"parent_record_offset":39304,"ancestor_record_offset":38252,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":39304,"parent_record_offset":39280,"ancestor_record_offset":38252,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":505,"record_offset":41512,"parent_record_offset":41488,"ancestor_record_offset":38252,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":39256,"parent_record_offset":39232,"ancestor_record_offset":38252,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":39664,"parent_record_offset":39640,"ancestor_record_offset":38252,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":39232,"parent_record_offset":39208,"ancestor_record_offset":38252,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":39208,"parent_record_offset":39184,"ancestor_record_offset":38252,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::error::impl$2::with_err_path
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/error.rs
//     evidence={"module":505,"record_offset":39536,"parent_record_offset":39508,"ancestor_record_offset":38252,"inlinee":7527,"source_line":30,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::error::impl$2::with_err_path::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/error.rs
//     evidence={"module":505,"record_offset":39596,"parent_record_offset":39564,"ancestor_record_offset":38252,"inlinee":7529,"source_line":35,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":505,"record_offset":39124,"parent_record_offset":39100,"ancestor_record_offset":38252,"inlinee":7539,"source_line":235,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::impl$1::flush_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E7E
// 首条位置: {"module":565,"inlinee":32382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::fs::OpenOptions::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":565,"record_offset":40556,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":32383,"source_line":1511,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":565,"record_offset":40472,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":565,"record_offset":40612,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":565,"record_offset":40500,"parent_record_offset":40448,"ancestor_record_offset":40448,"inlinee":12233,"source_line":1380,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":565,"record_offset":40580,"parent_record_offset":40556,"ancestor_record_offset":40448,"inlinee":32384,"source_line":221,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":565,"record_offset":40524,"parent_record_offset":40500,"ancestor_record_offset":40448,"inlinee":11596,"source_line":209,"archive":"_pdb/B/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::impl$1::flush_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x700A
// 首条位置: {"module":505,"inlinee":28682}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":38664,"parent_record_offset":38520,"ancestor_record_offset":38520,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":38788,"parent_record_offset":38520,"ancestor_record_offset":38520,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":505,"record_offset":38608,"parent_record_offset":38520,"ancestor_record_offset":38520,"inlinee":28683,"source_line":147,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38816,"parent_record_offset":38520,"ancestor_record_offset":38520,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":505,"record_offset":38748,"parent_record_offset":38700,"ancestor_record_offset":38520,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":38700,"parent_record_offset":38664,"ancestor_record_offset":38520,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":38632,"parent_record_offset":38608,"ancestor_record_offset":38520,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":505,"record_offset":38880,"parent_record_offset":38844,"ancestor_record_offset":38520,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38932,"parent_record_offset":38908,"ancestor_record_offset":38520,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38908,"parent_record_offset":38844,"ancestor_record_offset":38520,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":38844,"parent_record_offset":38816,"ancestor_record_offset":38520,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::sync_all
// 记录形式: compiled_procedure; instances=1
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":505,"record_offset":42252,"rva":9544240,"va":5378253360}
// 编译实例 RVAs: 0x91a230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4390985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e849004300","size":5,"soft_fail":false,"target_va":5382644528,"va":5378253538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3353114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e8e6d5ccff","size":5,"soft_fail":false,"target_va":5374900736,"va":5378253845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2737389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e8133bd6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5378253416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d597e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4453588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e8d4f44300","size":5,"soft_fail":false,"target_va":5382707168,"va":5378253575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33090331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e81bebf801","size":5,"soft_fail":false,"target_va":5411344000,"va":5378253664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4401298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e892284300","size":5,"soft_fail":false,"target_va":5382655024,"va":5378253721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33288100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e8a4effb01","size":5,"soft_fail":false,"target_va":5411541904,"va":5378253799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::flush
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e8e1f0ffff","size":5,"soft_fail":false,"target_va":5378249680,"va":5378253546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4441051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253360,"raw_hex":"e8dbc34300","size":5,"soft_fail":false,"target_va":5382694656,"va":5378253600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":42772,"parent_record_offset":42252,"ancestor_record_offset":42252,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":42508,"parent_record_offset":42252,"ancestor_record_offset":42252,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":42892,"parent_record_offset":42252,"ancestor_record_offset":42252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":42952,"parent_record_offset":42252,"ancestor_record_offset":42252,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowBackendFile::unwrap_open_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":505,"record_offset":42708,"parent_record_offset":42252,"ancestor_record_offset":42252,"inlinee":28686,"source_line":24,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":42540,"parent_record_offset":42252,"ancestor_record_offset":42252,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":42424,"parent_record_offset":42252,"ancestor_record_offset":42252,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":505,"record_offset":42852,"parent_record_offset":42804,"ancestor_record_offset":42252,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":505,"record_offset":42804,"parent_record_offset":42772,"ancestor_record_offset":42252,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":505,"record_offset":42740,"parent_record_offset":42708,"ancestor_record_offset":42252,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":42592,"parent_record_offset":42564,"ancestor_record_offset":42252,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":42620,"parent_record_offset":42564,"ancestor_record_offset":42252,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":42920,"parent_record_offset":42892,"ancestor_record_offset":42252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":42448,"parent_record_offset":42424,"ancestor_record_offset":42252,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":42472,"parent_record_offset":42448,"ancestor_record_offset":42252,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":42648,"parent_record_offset":42564,"ancestor_record_offset":42252,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":42564,"parent_record_offset":42540,"ancestor_record_offset":42252,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":42672,"parent_record_offset":42648,"ancestor_record_offset":42252,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::mark_dirty
// 记录形式: compiled_procedure; instances=1
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":505,"record_offset":42984,"rva":9544784,"va":5378253904}
// 编译实例 RVAs: 0x91a450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4390307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253904,"raw_hex":"e8a3fd4200","size":5,"soft_fail":false,"target_va":5382644528,"va":5378254216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2737998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253904,"raw_hex":"e8b238d6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5378254025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33592550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253904,"raw_hex":"e8e6940002","size":5,"soft_fail":false,"target_va":5411847200,"va":5378254645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bitvec::vec::BitVec::resize<usize,bitvec::order::Lsb0>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253904,"raw_hex":"e86fb6ffff","size":5,"soft_fail":false,"target_va":5378235424,"va":5378254252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33287096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378253904,"raw_hex":"e8b8ebfb01","size":5,"soft_fail":false,"target_va":5411541904,"va":5378254803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378253904,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378254574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bitvec::slice::BitSlice::fill
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":505,"record_offset":44324,"parent_record_offset":42984,"ancestor_record_offset":42984,"inlinee":28650,"source_line":2129,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":505,"record_offset":43532,"parent_record_offset":42984,"ancestor_record_offset":42984,"inlinee":25242,"source_line":774,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::ops::impl$16::index_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/ops.rs
//     evidence={"module":505,"record_offset":43588,"parent_record_offset":42984,"ancestor_record_offset":42984,"inlinee":28688,"source_line":249,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":43324,"parent_record_offset":42984,"ancestor_record_offset":42984,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":505,"record_offset":43208,"parent_record_offset":42984,"ancestor_record_offset":42984,"inlinee":28676,"source_line":3197,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":505,"record_offset":43160,"parent_record_offset":42984,"ancestor_record_offset":42984,"inlinee":28683,"source_line":147,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":43356,"parent_record_offset":42984,"ancestor_record_offset":42984,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":43240,"parent_record_offset":42984,"ancestor_record_offset":42984,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::access::BitAccess::set_bits
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/access.rs
//     evidence={"module":505,"record_offset":44740,"parent_record_offset":44716,"ancestor_record_offset":42984,"inlinee":25503,"source_line":72,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::domain::Domain::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/domain.rs
//     evidence={"module":505,"record_offset":44400,"parent_record_offset":44376,"ancestor_record_offset":42984,"inlinee":25495,"source_line":377,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::domain::PartialElement::store_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/domain.rs
//     evidence={"module":505,"record_offset":44716,"parent_record_offset":44324,"ancestor_record_offset":42984,"inlinee":25497,"source_line":827,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::BitEnd::span
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":505,"record_offset":44620,"parent_record_offset":44592,"ancestor_record_offset":42984,"inlinee":25481,"source_line":545,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::BitIdx::offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":505,"record_offset":43952,"parent_record_offset":43924,"ancestor_record_offset":42984,"inlinee":11342,"source_line":259,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::BitIdx::span
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":505,"record_offset":44592,"parent_record_offset":44564,"ancestor_record_offset":42984,"inlinee":25479,"source_line":295,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::impl$35::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":505,"record_offset":44680,"parent_record_offset":44400,"ancestor_record_offset":42984,"inlinee":25492,"source_line":34,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::mem::elts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/mem.rs
//     evidence={"module":505,"record_offset":44532,"parent_record_offset":44476,"ancestor_record_offset":42984,"inlinee":11199,"source_line":51,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":505,"record_offset":43924,"parent_record_offset":43856,"ancestor_record_offset":42984,"inlinee":11341,"source_line":545,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::span_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":505,"record_offset":44156,"parent_record_offset":43832,"ancestor_record_offset":42984,"inlinee":25256,"source_line":245,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":43888,"parent_record_offset":43856,"ancestor_record_offset":42984,"inlinee":11349,"source_line":251,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::elements
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":44476,"parent_record_offset":44400,"ancestor_record_offset":42984,"inlinee":11250,"source_line":400,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::head
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":44444,"parent_record_offset":44400,"ancestor_record_offset":42984,"inlinee":11339,"source_line":293,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":43556,"parent_record_offset":43532,"ancestor_record_offset":42984,"inlinee":11197,"source_line":333,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::new_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":44180,"parent_record_offset":44156,"ancestor_record_offset":42984,"inlinee":25257,"source_line":200,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::tail
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":44564,"parent_record_offset":44400,"ancestor_record_offset":42984,"inlinee":25478,"source_line":417,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::to_bitptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":505,"record_offset":43856,"parent_record_offset":43832,"ancestor_record_offset":42984,"inlinee":11338,"source_line":532,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::BitSlice::domain_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice.rs
//     evidence={"module":505,"record_offset":44376,"parent_record_offset":44324,"ancestor_record_offset":42984,"inlinee":25493,"source_line":1038,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::BitSlice::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":505,"record_offset":43688,"parent_record_offset":43652,"ancestor_record_offset":42984,"inlinee":11335,"source_line":87,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":505,"record_offset":43752,"parent_record_offset":43652,"ancestor_record_offset":42984,"inlinee":28692,"source_line":2637,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::get_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":505,"record_offset":43780,"parent_record_offset":43752,"ancestor_record_offset":42984,"inlinee":28694,"source_line":2682,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::get_unchecked_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":505,"record_offset":43808,"parent_record_offset":43752,"ancestor_record_offset":42984,"inlinee":28647,"source_line":2654,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::get_unchecked_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":505,"record_offset":43832,"parent_record_offset":43808,"ancestor_record_offset":42984,"inlinee":28649,"source_line":2687,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::index_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":505,"record_offset":43652,"parent_record_offset":43620,"ancestor_record_offset":42984,"inlinee":28691,"source_line":2670,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::index_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":505,"record_offset":44252,"parent_record_offset":44228,"ancestor_record_offset":42984,"inlinee":28696,"source_line":2673,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::ops::impl$6::index_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/ops.rs
//     evidence={"module":505,"record_offset":43620,"parent_record_offset":43588,"ancestor_record_offset":42984,"inlinee":28690,"source_line":190,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::store::impl$12::store_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/store.rs
//     evidence={"module":505,"record_offset":44944,"parent_record_offset":44324,"ancestor_record_offset":42984,"inlinee":25505,"source_line":121,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":505,"record_offset":44788,"parent_record_offset":44764,"ancestor_record_offset":42984,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":43180,"parent_record_offset":43160,"ancestor_record_offset":42984,"inlinee":7225,"source_line":1824,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":505,"record_offset":44276,"parent_record_offset":44252,"ancestor_record_offset":42984,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":505,"record_offset":44812,"parent_record_offset":44788,"ancestor_record_offset":42984,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":505,"record_offset":43976,"parent_record_offset":43952,"ancestor_record_offset":42984,"inlinee":11343,"source_line":1891,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":505,"record_offset":44228,"parent_record_offset":43652,"ancestor_record_offset":42984,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":43412,"parent_record_offset":43380,"ancestor_record_offset":42984,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":43440,"parent_record_offset":43380,"ancestor_record_offset":42984,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":505,"record_offset":44104,"parent_record_offset":44080,"ancestor_record_offset":42984,"inlinee":5445,"source_line":416,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":505,"record_offset":44912,"parent_record_offset":44884,"ancestor_record_offset":42984,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":44836,"parent_record_offset":44812,"ancestor_record_offset":42984,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":505,"record_offset":44884,"parent_record_offset":44324,"ancestor_record_offset":42984,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":43264,"parent_record_offset":43240,"ancestor_record_offset":42984,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":43288,"parent_record_offset":43264,"ancestor_record_offset":42984,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":43468,"parent_record_offset":43380,"ancestor_record_offset":42984,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":43380,"parent_record_offset":43356,"ancestor_record_offset":42984,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":43492,"parent_record_offset":43468,"ancestor_record_offset":42984,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] radium::impl$32::fetch_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"radium","version":"0.7.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/radium-0.7.0/src/lib.rs
//     evidence={"module":505,"record_offset":44764,"parent_record_offset":44740,"ancestor_record_offset":42984,"inlinee":8670,"source_line":563,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] tap::pipe::Pipe::pipe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tap","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tap-1.0.1/src/pipe.rs
//     evidence={"module":505,"record_offset":44056,"parent_record_offset":44032,"ancestor_record_offset":42984,"inlinee":9916,"source_line":73,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::Address::offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":505,"record_offset":44008,"parent_record_offset":43924,"ancestor_record_offset":42984,"inlinee":25255,"source_line":256,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::Address::with_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":505,"record_offset":44032,"parent_record_offset":44008,"ancestor_record_offset":42984,"inlinee":9978,"source_line":328,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::impl$10::offset::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":505,"record_offset":44080,"parent_record_offset":44056,"ancestor_record_offset":42984,"inlinee":9918,"source_line":259,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::path
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x702C
// 首条位置: {"module":1585,"inlinee":28716}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1585,"record_offset":491560,"parent_record_offset":491536,"ancestor_record_offset":491536,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":491608,"parent_record_offset":491584,"ancestor_record_offset":491536,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":491656,"parent_record_offset":491632,"ancestor_record_offset":491536,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":491632,"parent_record_offset":491608,"ancestor_record_offset":491536,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1585,"record_offset":491584,"parent_record_offset":491560,"ancestor_record_offset":491536,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::impl$2::drop
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6928
// 首条位置: {"module":485,"inlinee":26920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFileWriter::write_all
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7031
// 首条位置: {"module":545,"inlinee":28721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":545,"record_offset":33188,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":28698,"source_line":289,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::io::Write::write_all
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":545,"record_offset":33216,"parent_record_offset":33152,"ancestor_record_offset":33152,"inlinee":8631,"source_line":1754,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":33456,"parent_record_offset":33432,"ancestor_record_offset":33152,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":33576,"parent_record_offset":33548,"ancestor_record_offset":33152,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":33480,"parent_record_offset":33456,"ancestor_record_offset":33152,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":33636,"parent_record_offset":33604,"ancestor_record_offset":33152,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":545,"record_offset":33604,"parent_record_offset":33576,"ancestor_record_offset":33152,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":33432,"parent_record_offset":33332,"ancestor_record_offset":33152,"inlinee":7415,"source_line":1246,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":33516,"parent_record_offset":33332,"ancestor_record_offset":33152,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":33548,"parent_record_offset":33332,"ancestor_record_offset":33152,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":33816,"parent_record_offset":33332,"ancestor_record_offset":33152,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":33704,"parent_record_offset":33332,"ancestor_record_offset":33152,"inlinee":8644,"source_line":2876,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":545,"record_offset":33896,"parent_record_offset":33872,"ancestor_record_offset":33152,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":545,"record_offset":33404,"parent_record_offset":33332,"ancestor_record_offset":33152,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":545,"record_offset":33660,"parent_record_offset":33636,"ancestor_record_offset":33152,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":545,"record_offset":33728,"parent_record_offset":33704,"ancestor_record_offset":33152,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":545,"record_offset":33872,"parent_record_offset":33848,"ancestor_record_offset":33152,"inlinee":9440,"source_line":1455,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":545,"record_offset":33760,"parent_record_offset":33332,"ancestor_record_offset":33152,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":545,"record_offset":33784,"parent_record_offset":33760,"ancestor_record_offset":33152,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$9::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":545,"record_offset":33244,"parent_record_offset":33216,"ancestor_record_offset":33152,"inlinee":28700,"source_line":617,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":545,"record_offset":33332,"parent_record_offset":33272,"ancestor_record_offset":33152,"inlinee":8643,"source_line":443,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":545,"record_offset":33272,"parent_record_offset":33244,"ancestor_record_offset":33152,"inlinee":8642,"source_line":504,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":545,"record_offset":33848,"parent_record_offset":33272,"ancestor_record_offset":33152,"inlinee":8648,"source_line":486,"archive":"_pdb/B/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 289
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x701A
// 首条位置: {"module":505,"inlinee":28698}
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
// 完整结构及原始类型记录: shadow_file.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::shadow_file::ShadowBackendFile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::save::shadow_file::ShadowBackendFile>", "constructor": "enum2$", "arguments": ["country_core::systems::save::shadow_file::ShadowBackendFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::systems::save::shadow_file::ShadowBackendFile> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::systems::save::shadow_file::ShadowBackendFile>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::shadow_file::ShadowFile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::shadow_file::ShadowFile>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::shadow_file::ShadowFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::systems::save::shadow_file::ShadowFile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain_mut<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$0>", "constructor": "alloc::vec::Vec::retain_mut", "arguments": ["country_core::systems::save::shadow_file::ShadowFile", "alloc::alloc::Global", "country_core::systems::save::save_thread::save_thread::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::save::shadow_file::ShadowFile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::shadow_file::ShadowFileWriter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shadow_file.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shadow_file.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 99; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowBackendFile::unwrap_open_mut
// 信息 ID: 897339cce2ef0cc08419d5fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::sync_all; module=505, inline_record=42708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::create
// 信息 ID: 30ade2b279bb47ee4a33001c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::execute_save_cmd @ 0x172508c -> 0x9184d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::data
// 信息 ID: d6656d816ec4f261744de16e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1686, inline_record=15692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=59300; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=87940; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=92028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::flush
// 信息 ID: 01660ff67b5eb41e4682e4e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::shadow_file::ShadowFile::sync_all @ 0x91a2ea -> 0x9193d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::flush_impl
// 信息 ID: b749de750eefcc9d706de733
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::flush; module=505, inline_record=38252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::mark_dirty
// 信息 ID: b4cf4050666cfdeaf5f2e030
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_game::add_history_edit_to_save_file @ 0xa43a76 -> 0x91a450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_game::add_history_edit_to_save_file @ 0xa43b6b -> 0x91a450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::change_section @ 0x171f31e -> 0x91a450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::change_section @ 0x171f3b1 -> 0x91a450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::path
// 信息 ID: 03a4602b7f2567c692182a54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread::closure$0; module=1585, inline_record=491536; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::UnsavedFileList::contains_file; module=1686, inline_record=31516; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1686, inline_record=34488; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=90956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::sync_all
// 信息 ID: cbb6d0e0f4bedb1730eb1506
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<country_core::systems::save::shadow_file::ShadowFile> @ 0x88ed29 -> 0x91a230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread::closure$0 @ 0x130f926 -> 0x91a230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1720838 -> 0x91a230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::truncate
// 信息 ID: c1e639828f49aefba49f8336
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=78148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::write_at
// 信息 ID: 45740558f987521bbb81a535
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=75072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
// 信息 ID: 1de5f7d03fdcd31c75773cbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=545, inline_record=32948; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=545, inline_record=33188; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=545, inline_record=34868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1686, inline_record=17836; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1686, inline_record=18700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=77244; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=78116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFileWriter::write_all
// 信息 ID: c87597103c3ee98136b990d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=545, inline_record=33152; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=545, inline_record=34832; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1686, inline_record=17800; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1686, inline_record=18664; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=77204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::impl$1::create::closure$1
// 信息 ID: ca02152a69a34e358ee109c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::shadow_file::impl$1::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::create; module=505, inline_record=34816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::impl$1::flush_impl::closure$0
// 信息 ID: d6178aa228ffd34be6f40600
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::shadow_file::impl$1::flush_impl", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::shadow_file::impl$1::flush_impl::closure_env$0 <- tempfile::util::create_helper<tempfile::file::NamedTempFile<std::fs::File>,tempfile::impl$1::make_in::closure_env$0<country_core::systems::save::shadow_file::impl$1::flush_impl::closure_env$0,std::fs::File,ref_mut$<std::path::PathBuf> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] tempfile::util::create_helper<tempfile::file::NamedTempFile<std::fs::File>,tempfile::impl$1::make_in::closure_env$0<country_core::systems::save::shadow_file::impl$1::flush_impl::closure_env$0,std::fs::File,ref_mut$<std::path::PathBuf> > >; module=565, inline_record=40448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::impl$1::flush_impl::closure$1
// 信息 ID: 6a5ea5abcac4b385686492bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::shadow_file::impl$1::flush_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::flush; module=505, inline_record=38520; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::flush; module=505, inline_record=39848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::impl$2::drop
// 信息 ID: e66d07de18db82ea0ce18193
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::systems::save::shadow_file::ShadowFile>; module=485, inline_record=837880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
