// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/vram_snapshot.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 190f0789b9bc08c8b174450f02f4a7292eb6543932b19e32e03a801c6862811f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: vram_snapshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 8, 39, 91, 92, 144, 185, 224, 235
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use gpu_allocator; // namespace candidate
//   package identities: [{"name":"gpu-allocator","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.26.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3741
// 首条位置: {"module":144,"inlinee":14145}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$11::serialize_unit_variant
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":202416,"parent_record_offset":202392,"ancestor_record_offset":202392,"inlinee":14151,"source_line":809,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":158488,"parent_record_offset":158464,"ancestor_record_offset":158464,"inlinee":7408,"source_line":214,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":144,"record_offset":202520,"parent_record_offset":202496,"ancestor_record_offset":202392,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":158512,"parent_record_offset":158488,"ancestor_record_offset":158464,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::impl$3::cmp
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1555
// 首条位置: {"module":8,"inlinee":5461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$71::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":8,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":16788,"inlinee":5463,"source_line":2010,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 记录形式: compiled_procedure; instances=4
// 源码行: 20, 28, 38, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":185,"record_offset":536,"rva":5455824,"va":5374164944}
// 编译实例 RVAs: 0x533fd0, 0x534160, 0x534290, 0x5344f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403aeac0
//     owner=unresolved_namespace; count=4
//     target_aliases=9 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1594981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164944,"raw_hex":"e89ba9e7ff","size":5,"soft_fail":false,"target_va":5372570304,"va":5374165280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045d420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-879755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164944,"raw_hex":"e87593f2ff","size":5,"soft_fail":false,"target_va":5373285408,"va":5374165158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404604c0
//     owner=unresolved_namespace; count=18
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-867268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164944,"raw_hex":"e83cc4f2ff","size":5,"soft_fail":false,"target_va":5373297856,"va":5374165119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140460b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-865621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164944,"raw_hex":"e8abcaf2ff","size":5,"soft_fail":false,"target_va":5373299456,"va":5374165072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404620c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-860712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374165648,"raw_hex":"e8d8ddf2ff","size":5,"soft_fail":false,"target_va":5373305024,"va":5374165731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-848951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164944,"raw_hex":"e8c90bf3ff","size":5,"soft_fail":false,"target_va":5373316080,"va":5374165026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140468d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-832718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374165344,"raw_hex":"e8324bf3ff","size":5,"soft_fail":false,"target_va":5373332848,"va":5374165561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140536180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374165648,"raw_hex":"e8131d0000","size":5,"soft_fail":false,"target_va":5374173568,"va":5374166120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405361a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374165648,"raw_hex":"e8031d0000","size":5,"soft_fail":false,"target_va":5374173600,"va":5374166168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405361c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374165648,"raw_hex":"e83d1d0000","size":5,"soft_fail":false,"target_va":5374173632,"va":5374166142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42187713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164944,"raw_hex":"e8c1bb8302","size":5,"soft_fail":false,"target_va":5416353040,"va":5374165322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=4
//     target_aliases=9 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42392696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374164944,"raw_hex":"e878dc8602","size":5,"soft_fail":false,"target_va":5416557744,"va":5374165043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":185,"record_offset":800,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":185,"record_offset":1348,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":185,"record_offset":1200,"parent_record_offset":1168,"ancestor_record_offset":536,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":185,"record_offset":1068,"parent_record_offset":1040,"ancestor_record_offset":536,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":185,"record_offset":1128,"parent_record_offset":1096,"ancestor_record_offset":536,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":185,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":536,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":185,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":536,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":185,"record_offset":1096,"parent_record_offset":1068,"ancestor_record_offset":536,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":185,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":536,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":185,"record_offset":1168,"parent_record_offset":1000,"ancestor_record_offset":536,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":185,"record_offset":944,"parent_record_offset":916,"ancestor_record_offset":536,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":185,"record_offset":1288,"parent_record_offset":1000,"ancestor_record_offset":536,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":185,"record_offset":1040,"parent_record_offset":1000,"ancestor_record_offset":536,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":185,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":536,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::begin_object
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":185,"record_offset":856,"parent_record_offset":828,"ancestor_record_offset":536,"inlinee":12492,"source_line":2018,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":185,"record_offset":828,"parent_record_offset":800,"ancestor_record_offset":536,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":185,"record_offset":916,"parent_record_offset":888,"ancestor_record_offset":536,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":185,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":536,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00185.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::dump_vram_snapshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":1136,"rva":1666288,"va":5370375408}
// 编译实例 RVAs: 0x196cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14310611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8d35cda00","size":5,"soft_fail":false,"target_va":5384689616,"va":5370379000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14269308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e87cbbd900","size":5,"soft_fail":false,"target_va":5384644912,"va":5370375599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14277540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8a4dbd900","size":5,"soft_fail":false,"target_va":5384656512,"va":5370378967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13087592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e868b3c700","size":5,"soft_fail":false,"target_va":5383464432,"va":5370376835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..texture..TextureSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13088565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e835b7c700","size":5,"soft_fail":false,"target_va":5383464496,"va":5370375926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5300941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8cde25000","size":5,"soft_fail":false,"target_va":5375678688,"va":5370377742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5301127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e887e35000","size":5,"soft_fail":false,"target_va":5375678736,"va":5370377604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t5299521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e841dd5000","size":5,"soft_fail":false,"target_va":5375678704,"va":5370379178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1655404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e894bde6ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370377591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1084583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e85973efff","size":5,"soft_fail":false,"target_va":5369296544,"va":5370381122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1076161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e83f94efff","size":5,"soft_fail":false,"target_va":5369305584,"va":5370381740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ac4a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-964423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8b948f1ff","size":5,"soft_fail":false,"target_va":5369414816,"va":5370379234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c1380
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-880407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8e990f2ff","size":5,"soft_fail":false,"target_va":5369500544,"va":5370380946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d62e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t254927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8cfe30300","size":5,"soft_fail":false,"target_va":5370634976,"va":5370380044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fa410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1452798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8fe2a1600","size":5,"soft_fail":false,"target_va":5371831312,"va":5370378509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ce1e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2321534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e87e6c2300","size":5,"soft_fail":false,"target_va":5372699104,"va":5370377565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3105671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e887632f00","size":5,"soft_fail":false,"target_va":5373481648,"va":5370375972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140534290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3786504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e808c73900","size":5,"soft_fail":false,"target_va":5374165648,"va":5370379139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140548aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3869537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8610b3b00","size":5,"soft_fail":false,"target_va":5374249632,"va":5370380090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4598866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8522c4600","size":5,"soft_fail":false,"target_va":5374978400,"va":5370379529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fafc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4601014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8b6344600","size":5,"soft_fail":false,"target_va":5374980032,"va":5370379013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45942957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8ad08bd02","size":5,"soft_fail":false,"target_va":5416322176,"va":5370379214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6cb30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45963731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8d359bd02","size":5,"soft_fail":false,"target_va":5416340272,"va":5370376536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45972006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8267abd02","size":5,"soft_fail":false,"target_va":5416350384,"va":5370378373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fa90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45975931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e87b89bd02","size":5,"soft_fail":false,"target_va":5416352400,"va":5370376464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: vram_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46680911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e84f4bc802","size":5,"soft_fail":false,"target_va":5417057040,"va":5370376124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t46243160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8589dc102","size":5,"soft_fail":false,"target_va":5416620794,"va":5370377629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t46242408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8689ac102","size":5,"soft_fail":false,"target_va":5416622320,"va":5370379907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::generate_report
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t13628902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8e6f5cf00","size":5,"soft_fail":false,"target_va":5384004464,"va":5370375557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t46196875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e88be8c002","size":5,"soft_fail":false,"target_va":5416576832,"va":5370379952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t46196172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8cce5c002","size":5,"soft_fail":false,"target_va":5416576144,"va":5370379967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13609302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e856a9cf00","size":5,"soft_fail":false,"target_va":5383984848,"va":5370375541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13609782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e836abcf00","size":5,"soft_fail":false,"target_va":5383985360,"va":5370375573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14372976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e87050db00","size":5,"soft_fail":false,"target_va":5384752464,"va":5370379483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14346696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370375408,"raw_hex":"e8c8e9da00","size":5,"soft_fail":false,"target_va":5384726688,"va":5370379987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::map::BTreeMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":39,"record_offset":11928,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":7768,"source_line":651,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":12096,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8431,"source_line":247,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":3544,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10308,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10044,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":11092,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":11620,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":11956,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":3432,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":39,"record_offset":11008,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::count
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":39,"record_offset":12920,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8436,"source_line":135,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":12012,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":9824,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":11592,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":14432,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":14404,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":11748,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":10696,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":3780,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::map::impl$65::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":39,"record_offset":2920,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8383,"source_line":3357,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::hash_map::HashMap::values
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/hash_map.rs
//     evidence={"module":39,"record_offset":2628,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8378,"source_line":479,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferHandleExt::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":39,"record_offset":7216,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8399,"source_line":113,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::with_global_allocator
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":39,"record_offset":1388,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8375,"source_line":877,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureHandleExt::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":39,"record_offset":3836,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8329,"source_line":125,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":13404,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8087,"source_line":2264,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":4972,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8395,"source_line":2645,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":3864,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8388,"source_line":975,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":1896,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":9388,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":13320,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":15840,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":1136,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":1136,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":1136,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":14060,"parent_record_offset":14036,"ancestor_record_offset":1136,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":10636,"parent_record_offset":10612,"ancestor_record_offset":1136,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":1136,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":1136,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":10564,"parent_record_offset":10412,"ancestor_record_offset":1136,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":10176,"parent_record_offset":10144,"ancestor_record_offset":1136,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":14012,"parent_record_offset":13956,"ancestor_record_offset":1136,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":1136,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":1136,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":10092,"parent_record_offset":10068,"ancestor_record_offset":1136,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":13984,"parent_record_offset":13956,"ancestor_record_offset":1136,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":1136,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":3704,"parent_record_offset":3680,"ancestor_record_offset":1136,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":3680,"parent_record_offset":3656,"ancestor_record_offset":1136,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":10144,"parent_record_offset":10116,"ancestor_record_offset":1136,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":10380,"parent_record_offset":10356,"ancestor_record_offset":1136,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":1136,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":13932,"parent_record_offset":13908,"ancestor_record_offset":1136,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":10452,"parent_record_offset":10412,"ancestor_record_offset":1136,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":12128,"parent_record_offset":12096,"ancestor_record_offset":1136,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":39,"record_offset":13192,"parent_record_offset":13164,"ancestor_record_offset":1136,"inlinee":5433,"source_line":1880,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":39,"record_offset":1464,"parent_record_offset":1420,"ancestor_record_offset":1136,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":39,"record_offset":1420,"parent_record_offset":1388,"ancestor_record_offset":1136,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":3632,"parent_record_offset":3580,"ancestor_record_offset":1136,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":3580,"parent_record_offset":3544,"ancestor_record_offset":1136,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":13520,"parent_record_offset":13456,"ancestor_record_offset":1136,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":1136,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":12036,"parent_record_offset":12012,"ancestor_record_offset":1136,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":20688,"parent_record_offset":20664,"ancestor_record_offset":1136,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":39,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":1136,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":39,"record_offset":10476,"parent_record_offset":10452,"ancestor_record_offset":1136,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":39,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":1136,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":39,"record_offset":2380,"parent_record_offset":2176,"ancestor_record_offset":1136,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":39,"record_offset":1644,"parent_record_offset":1420,"ancestor_record_offset":1136,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":39,"record_offset":4544,"parent_record_offset":4516,"ancestor_record_offset":1136,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":1136,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":1136,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":1136,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":39,"record_offset":4708,"parent_record_offset":3992,"ancestor_record_offset":1136,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":39,"record_offset":4928,"parent_record_offset":4896,"ancestor_record_offset":1136,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::count::to_usize::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":39,"record_offset":13136,"parent_record_offset":13108,"ancestor_record_offset":1136,"inlinee":8438,"source_line":138,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":39,"record_offset":9880,"parent_record_offset":9852,"ancestor_record_offset":1136,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":39,"record_offset":11484,"parent_record_offset":11304,"ancestor_record_offset":1136,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":39,"record_offset":11248,"parent_record_offset":11220,"ancestor_record_offset":1136,"inlinee":8420,"source_line":49,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":39,"record_offset":11512,"parent_record_offset":11484,"ancestor_record_offset":1136,"inlinee":8422,"source_line":53,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":39,"record_offset":9852,"parent_record_offset":9824,"ancestor_record_offset":1136,"inlinee":8405,"source_line":49,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":39,"record_offset":13232,"parent_record_offset":13108,"ancestor_record_offset":1136,"inlinee":8441,"source_line":53,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":39,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":1136,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":12944,"parent_record_offset":12920,"ancestor_record_offset":1136,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":39,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":1136,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":10500,"parent_record_offset":10476,"ancestor_record_offset":1136,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":11180,"parent_record_offset":11148,"ancestor_record_offset":1136,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":18104,"parent_record_offset":18076,"ancestor_record_offset":1136,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":11548,"parent_record_offset":11304,"ancestor_record_offset":1136,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":14564,"parent_record_offset":14532,"ancestor_record_offset":1136,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":5756,"parent_record_offset":5728,"ancestor_record_offset":1136,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":2508,"parent_record_offset":2452,"ancestor_record_offset":1136,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":39,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":1136,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":39,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":1136,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":39,"record_offset":2088,"parent_record_offset":2052,"ancestor_record_offset":1136,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":2208,"parent_record_offset":2176,"ancestor_record_offset":1136,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":2868,"parent_record_offset":2784,"ancestor_record_offset":1136,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":11448,"parent_record_offset":11424,"ancestor_record_offset":1136,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":1136,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":1704,"parent_record_offset":1388,"ancestor_record_offset":1136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":1136,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":1136,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":1136,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":2296,"parent_record_offset":2176,"ancestor_record_offset":1136,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":1136,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":1136,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":11860,"parent_record_offset":11816,"ancestor_record_offset":1136,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":1136,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":11400,"parent_record_offset":11304,"ancestor_record_offset":1136,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":1136,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":1136,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":3744,"parent_record_offset":3580,"ancestor_record_offset":1136,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":39,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":1136,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":39,"record_offset":4656,"parent_record_offset":4628,"ancestor_record_offset":1136,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":1136,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":39,"record_offset":11120,"parent_record_offset":11092,"ancestor_record_offset":1136,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":39,"record_offset":11648,"parent_record_offset":11620,"ancestor_record_offset":1136,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":39,"record_offset":11148,"parent_record_offset":11120,"ancestor_record_offset":1136,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":39,"record_offset":11676,"parent_record_offset":11648,"ancestor_record_offset":1136,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":39,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":1136,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":11304,"parent_record_offset":11276,"ancestor_record_offset":1136,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":11032,"parent_record_offset":11008,"ancestor_record_offset":1136,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":11816,"parent_record_offset":11776,"ancestor_record_offset":1136,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":39,"record_offset":12160,"parent_record_offset":12128,"ancestor_record_offset":1136,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$12::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":39,"record_offset":11776,"parent_record_offset":11748,"ancestor_record_offset":1136,"inlinee":8430,"source_line":52,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":4572,"parent_record_offset":4544,"ancestor_record_offset":1136,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":1824,"parent_record_offset":1760,"ancestor_record_offset":1136,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":1556,"parent_record_offset":1520,"ancestor_record_offset":1136,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":1136,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":1136,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":39,"record_offset":3068,"parent_record_offset":3036,"ancestor_record_offset":1136,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":39,"record_offset":3100,"parent_record_offset":3068,"ancestor_record_offset":1136,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":39,"record_offset":3188,"parent_record_offset":3036,"ancestor_record_offset":1136,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":39,"record_offset":3036,"parent_record_offset":2996,"ancestor_record_offset":1136,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":39,"record_offset":4104,"parent_record_offset":3992,"ancestor_record_offset":1136,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":39,"record_offset":2812,"parent_record_offset":2784,"ancestor_record_offset":1136,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":39,"record_offset":4736,"parent_record_offset":3992,"ancestor_record_offset":1136,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":39,"record_offset":5336,"parent_record_offset":5300,"ancestor_record_offset":1136,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":39,"record_offset":4188,"parent_record_offset":3992,"ancestor_record_offset":1136,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":39,"record_offset":4048,"parent_record_offset":3992,"ancestor_record_offset":1136,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":39,"record_offset":5880,"parent_record_offset":5068,"ancestor_record_offset":1136,"inlinee":7447,"source_line":28,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":39,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":1136,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":39,"record_offset":4868,"parent_record_offset":3900,"ancestor_record_offset":1136,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":3900,"parent_record_offset":3864,"ancestor_record_offset":1136,"inlinee":8389,"source_line":34,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::values
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":39,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":1136,"inlinee":8379,"source_line":682,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::fold
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":39,"record_offset":9964,"parent_record_offset":9936,"ancestor_record_offset":1136,"inlinee":8411,"source_line":3234,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$47::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":39,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":1136,"inlinee":8385,"source_line":3171,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":39,"record_offset":3940,"parent_record_offset":3900,"ancestor_record_offset":1136,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":15476,"parent_record_offset":14876,"ancestor_record_offset":1136,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":1136,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":3276,"parent_record_offset":2996,"ancestor_record_offset":1136,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5908,"parent_record_offset":5068,"ancestor_record_offset":1136,"inlinee":7448,"source_line":561,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":4828,"parent_record_offset":3992,"ancestor_record_offset":1136,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":2784,"parent_record_offset":2724,"ancestor_record_offset":1136,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":2996,"parent_record_offset":2968,"ancestor_record_offset":1136,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":1136,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":3968,"parent_record_offset":3900,"ancestor_record_offset":1136,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5068,"parent_record_offset":5036,"ancestor_record_offset":1136,"inlinee":7438,"source_line":1114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":1136,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":1136,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":18016,"parent_record_offset":17984,"ancestor_record_offset":1136,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5796,"parent_record_offset":5068,"ancestor_record_offset":1136,"inlinee":7446,"source_line":2382,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":9564,"parent_record_offset":9500,"ancestor_record_offset":1136,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":2756,"parent_record_offset":2724,"ancestor_record_offset":1136,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":14876,"parent_record_offset":14820,"ancestor_record_offset":1136,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":14820,"parent_record_offset":14796,"ancestor_record_offset":1136,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":1136,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5160,"parent_record_offset":5116,"ancestor_record_offset":1136,"inlinee":7440,"source_line":1992,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index_in_group
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5300,"parent_record_offset":5160,"ancestor_record_offset":1136,"inlinee":7441,"source_line":1787,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5484,"parent_record_offset":5160,"ancestor_record_offset":1136,"inlinee":7443,"source_line":1750,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":17984,"parent_record_offset":17960,"ancestor_record_offset":1136,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5520,"parent_record_offset":5484,"ancestor_record_offset":1136,"inlinee":7444,"source_line":2685,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":14848,"parent_record_offset":14820,"ancestor_record_offset":1136,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":1136,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5116,"parent_record_offset":5068,"ancestor_record_offset":1136,"inlinee":7439,"source_line":1950,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":4076,"parent_record_offset":3992,"ancestor_record_offset":1136,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":1136,"inlinee":7463,"source_line":2606,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5704,"parent_record_offset":5116,"ancestor_record_offset":1136,"inlinee":7445,"source_line":2540,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":18048,"parent_record_offset":18016,"ancestor_record_offset":1136,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":14796,"parent_record_offset":14772,"ancestor_record_offset":1136,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":39,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":1136,"inlinee":5194,"source_line":3661,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::fold
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":9992,"parent_record_offset":9964,"ancestor_record_offset":1136,"inlinee":8412,"source_line":3872,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":15084,"parent_record_offset":14876,"ancestor_record_offset":1136,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":4408,"parent_record_offset":3992,"ancestor_record_offset":1136,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":5036,"parent_record_offset":5004,"ancestor_record_offset":1136,"inlinee":7470,"source_line":512,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":4516,"parent_record_offset":4408,"ancestor_record_offset":1136,"inlinee":8391,"source_line":36,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":39,"record_offset":1732,"parent_record_offset":1704,"ancestor_record_offset":1136,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":39,"record_offset":1520,"parent_record_offset":1420,"ancestor_record_offset":1136,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":39,"record_offset":1760,"parent_record_offset":1732,"ancestor_record_offset":1136,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":39,"record_offset":1612,"parent_record_offset":1520,"ancestor_record_offset":1136,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":39,"record_offset":1796,"parent_record_offset":1760,"ancestor_record_offset":1136,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":13824,"parent_record_offset":13796,"ancestor_record_offset":1136,"inlinee":8091,"source_line":39,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":13848,"parent_record_offset":13824,"ancestor_record_offset":1136,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":13456,"parent_record_offset":13404,"ancestor_record_offset":1136,"inlinee":8088,"source_line":2229,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":13796,"parent_record_offset":13456,"ancestor_record_offset":1136,"inlinee":8089,"source_line":2197,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":9416,"parent_record_offset":9388,"ancestor_record_offset":1136,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":5004,"parent_record_offset":4972,"ancestor_record_offset":1136,"inlinee":8397,"source_line":2881,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":9936,"parent_record_offset":9908,"ancestor_record_offset":1136,"inlinee":8409,"source_line":2089,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$52::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":9908,"parent_record_offset":9880,"ancestor_record_offset":1136,"inlinee":8407,"source_line":2248,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":1136,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":13372,"parent_record_offset":13320,"ancestor_record_offset":1136,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":1136,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":39,"record_offset":1996,"parent_record_offset":1952,"ancestor_record_offset":1136,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":39,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":1136,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":39,"record_offset":2116,"parent_record_offset":2088,"ancestor_record_offset":1136,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":39,"record_offset":2452,"parent_record_offset":2052,"ancestor_record_offset":1136,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":39,"record_offset":1952,"parent_record_offset":1924,"ancestor_record_offset":1136,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":39,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1136,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":39,"record_offset":2176,"parent_record_offset":2144,"ancestor_record_offset":1136,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":39,"record_offset":2052,"parent_record_offset":2024,"ancestor_record_offset":1136,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":39,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":1136,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::vram_snapshot::dump_vram_snapshot::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/vram_snapshot.rs
//     evidence={"module":39,"record_offset":1676,"parent_record_offset":1388,"ancestor_record_offset":1136,"inlinee":8376,"source_line":67,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::vram_snapshot::dump_vram_snapshot::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/vram_snapshot.rs
//     evidence={"module":39,"record_offset":13164,"parent_record_offset":13136,"ancestor_record_offset":1136,"inlinee":8439,"source_line":147,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20B8
// 首条位置: {"module":39,"inlinee":8376}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1538
// 首条位置: {"module":8,"inlinee":5432}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::btree::map::BTreeMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":8,"record_offset":16508,"parent_record_offset":15140,"ancestor_record_offset":15140,"inlinee":5452,"source_line":1499,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":8,"record_offset":15212,"parent_record_offset":15140,"ancestor_record_offset":15140,"inlinee":5433,"source_line":1880,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":8,"record_offset":17028,"parent_record_offset":15140,"ancestor_record_offset":15140,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":8,"record_offset":16296,"parent_record_offset":15140,"ancestor_record_offset":15140,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":8,"record_offset":17124,"parent_record_offset":15140,"ancestor_record_offset":15140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":8,"record_offset":15240,"parent_record_offset":15140,"ancestor_record_offset":15140,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":8,"record_offset":17300,"parent_record_offset":17276,"ancestor_record_offset":15140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":8,"record_offset":17324,"parent_record_offset":17300,"ancestor_record_offset":15140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":8,"record_offset":17348,"parent_record_offset":17324,"ancestor_record_offset":15140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":8,"record_offset":17276,"parent_record_offset":17220,"ancestor_record_offset":15140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":8,"record_offset":16908,"parent_record_offset":16584,"ancestor_record_offset":15140,"inlinee":4921,"source_line":1102,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":8,"record_offset":16852,"parent_record_offset":16584,"ancestor_record_offset":15140,"inlinee":4920,"source_line":1681,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":8,"record_offset":16556,"parent_record_offset":16508,"ancestor_record_offset":15140,"inlinee":5453,"source_line":640,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::find_key_index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":8,"record_offset":16644,"parent_record_offset":16620,"ancestor_record_offset":15140,"inlinee":5456,"source_line":217,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":8,"record_offset":16876,"parent_record_offset":16852,"ancestor_record_offset":15140,"inlinee":4916,"source_line":725,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::keys
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":8,"record_offset":16676,"parent_record_offset":16644,"ancestor_record_offset":15140,"inlinee":5457,"source_line":394,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_node
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":8,"record_offset":16620,"parent_record_offset":16584,"ancestor_record_offset":15140,"inlinee":5455,"source_line":195,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_tree
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":8,"record_offset":16584,"parent_record_offset":16508,"ancestor_record_offset":15140,"inlinee":5454,"source_line":49,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":16408,"parent_record_offset":16376,"ancestor_record_offset":15140,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":17248,"parent_record_offset":17220,"ancestor_record_offset":15140,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":17220,"parent_record_offset":17196,"ancestor_record_offset":15140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":16456,"parent_record_offset":16432,"ancestor_record_offset":15140,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":16432,"parent_record_offset":16408,"ancestor_record_offset":15140,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":8,"record_offset":17196,"parent_record_offset":17172,"ancestor_record_offset":15140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":8,"record_offset":16376,"parent_record_offset":16344,"ancestor_record_offset":15140,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":8,"record_offset":16344,"parent_record_offset":16320,"ancestor_record_offset":15140,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$71::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":8,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":15140,"inlinee":5463,"source_line":2010,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":8,"record_offset":15952,"parent_record_offset":15924,"ancestor_record_offset":15140,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":8,"record_offset":15508,"parent_record_offset":15484,"ancestor_record_offset":15140,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":8,"record_offset":15592,"parent_record_offset":15568,"ancestor_record_offset":15140,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":8,"record_offset":16124,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":8,"record_offset":16704,"parent_record_offset":16644,"ancestor_record_offset":15140,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":8,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":15140,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":8,"record_offset":15744,"parent_record_offset":15716,"ancestor_record_offset":15140,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":8,"record_offset":15716,"parent_record_offset":15688,"ancestor_record_offset":15140,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":8,"record_offset":16320,"parent_record_offset":16296,"ancestor_record_offset":15140,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":8,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":15140,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":8,"record_offset":15532,"parent_record_offset":15508,"ancestor_record_offset":15140,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":8,"record_offset":17148,"parent_record_offset":17124,"ancestor_record_offset":15140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":8,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":15140,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":8,"record_offset":16752,"parent_record_offset":16728,"ancestor_record_offset":15140,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":8,"record_offset":16980,"parent_record_offset":16956,"ancestor_record_offset":15140,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":8,"record_offset":16036,"parent_record_offset":16008,"ancestor_record_offset":15140,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":8,"record_offset":16068,"parent_record_offset":16036,"ancestor_record_offset":15140,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":8,"record_offset":17056,"parent_record_offset":17028,"ancestor_record_offset":15140,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":8,"record_offset":17084,"parent_record_offset":17056,"ancestor_record_offset":15140,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":8,"record_offset":16728,"parent_record_offset":16704,"ancestor_record_offset":15140,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":8,"record_offset":15980,"parent_record_offset":15952,"ancestor_record_offset":15140,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":8,"record_offset":15652,"parent_record_offset":15624,"ancestor_record_offset":15140,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":8,"record_offset":15688,"parent_record_offset":15652,"ancestor_record_offset":15140,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":8,"record_offset":15624,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":8,"record_offset":15484,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":8,"record_offset":16152,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":8,"record_offset":15568,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":8,"record_offset":15428,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":8,"record_offset":15924,"parent_record_offset":15896,"ancestor_record_offset":15140,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":8,"record_offset":15264,"parent_record_offset":15240,"ancestor_record_offset":15140,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":8,"record_offset":15896,"parent_record_offset":15788,"ancestor_record_offset":15140,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":8,"record_offset":15296,"parent_record_offset":15264,"ancestor_record_offset":15140,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":15836,"parent_record_offset":15812,"ancestor_record_offset":15140,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":16244,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":15812,"parent_record_offset":15788,"ancestor_record_offset":15140,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":15348,"parent_record_offset":15324,"ancestor_record_offset":15140,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":15324,"parent_record_offset":15264,"ancestor_record_offset":15140,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":15372,"parent_record_offset":15348,"ancestor_record_offset":15140,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":15456,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":8,"record_offset":15788,"parent_record_offset":15372,"ancestor_record_offset":15140,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::vram_snapshot::impl$3::cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/vram_snapshot.rs
//     evidence={"module":8,"record_offset":16788,"parent_record_offset":16644,"ancestor_record_offset":15140,"inlinee":5461,"source_line":12,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F7
// 首条位置: {"module":39,"inlinee":8439}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":39,"record_offset":13192,"parent_record_offset":13164,"ancestor_record_offset":13164,"inlinee":5433,"source_line":1880,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43C0
// 首条位置: {"module":235,"inlinee":17344}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":22676,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":22724,"parent_record_offset":22700,"ancestor_record_offset":22656,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":235,"record_offset":22700,"parent_record_offset":22676,"ancestor_record_offset":22656,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43BA
// 首条位置: {"module":235,"inlinee":17338}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":19396,"parent_record_offset":19376,"ancestor_record_offset":19376,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":19444,"parent_record_offset":19420,"ancestor_record_offset":19376,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":235,"record_offset":19420,"parent_record_offset":19396,"ancestor_record_offset":19376,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00235.jsonl.gz"}
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
// 完整结构及原始类型记录: vram_snapshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::vram_snapshot::AllocationCategory
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::BTreeMap", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::IntoIter::dying_next<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::IntoIter::dying_next", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::entry::Entry::or_default<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::entry::Entry::or_default", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::node::Handle::insert_recursing<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global,alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >", "constructor": "alloc::collections::btree::node::Handle::insert_recursing", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global", "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["ref$<str$>", "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >", "constructor": "std::collections::hash::map::Values", "arguments": ["ref$<str$>", "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >", "usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> >", "constructor": "core::iter::traits::accum::impl$48::sum::closure_env$0", "arguments": ["core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize,usize,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >),core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >", "usize", "usize", "usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)", "core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$52::fold::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,usize,core::iter::adapters::map::map_fold::closure_env$0<ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize,usize,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >),core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> > > >", "constructor": "std::collections::hash::map::impl$52::fold::closure_env$0", "arguments": ["ref$<str$>", "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>", "usize", "core::iter::adapters::map::map_fold::closure_env$0<ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize,usize,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >),core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$44::fold::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,usize,std::collections::hash::map::impl$52::fold::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,usize,core::iter::adapters::map::map_fold::closure_env$0<ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize,usize,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >),core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> > > > >", "constructor": "hashbrown::map::impl$44::fold::closure_env$0", "arguments": ["ref$<str$>", "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>", "usize", "std::collections::hash::map::impl$52::fold::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,usize,core::iter::adapters::map::map_fold::closure_env$0<ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize,usize,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >),core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,hashbrown::map::impl$44::fold::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,usize,std::collections::hash::map::impl$52::fold::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,usize,core::iter::adapters::map::map_fold::closure_env$0<ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize,usize,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >),core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> > > > >,usize>", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >", "hashbrown::map::impl$44::fold::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,usize,std::collections::hash::map::impl$52::fold::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,usize,core::iter::adapters::map::map_fold::closure_env$0<ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize,usize,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >),core::iter::traits::accum::impl$48::sum::closure_env$0<core::iter::adapters::map::Map<std::collections::hash::map::Values<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,usize (*)(ref$<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >)> > > > >", "usize"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["ref$<str$>", "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tiny_glade::vram_snapshot::AllocationCategory,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,tiny_glade::vram_snapshot::AllocationCategory>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "tiny_glade::vram_snapshot::AllocationCategory"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter,alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter", "alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::vram_snapshot::AllocationSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<usize,gpu_allocator::allocator::AllocationReport>", "tiny_glade::vram_snapshot::AllocationSnapshot", "tuple$<>", "tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::impl$1::fold::enumerate::closure_env$0<gpu_allocator::allocator::AllocationReport,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > > >", "constructor": "core::iter::adapters::enumerate::impl$1::fold::enumerate::closure_env$0", "arguments": ["gpu_allocator::allocator::AllocationReport", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global,tuple$<>,core::iter::adapters::enumerate::impl$1::fold::enumerate::closure_env$0<gpu_allocator::allocator::AllocationReport,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["gpu_allocator::allocator::AllocationReport", "alloc::alloc::Global", "tuple$<>", "core::iter::adapters::enumerate::impl$1::fold::enumerate::closure_env$0<gpu_allocator::allocator::AllocationReport,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4>", "constructor": "alloc::slice::impl$0::sort_by_key::closure_env$0", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "core::cmp::Reverse<u64>", "tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<tiny_glade::vram_snapshot::AllocationSnapshot,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global> >", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter", "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4>,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["tiny_glade::vram_snapshot::AllocationSnapshot", "alloc::slice::impl$0::sort_by_key::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,core::cmp::Reverse<u64>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$4>", "alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::vram_snapshot::CategorySummary
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::BTreeMap", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::IntoIter::dying_next<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::IntoIter::dying_next", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::entry::Entry::or_default<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::entry::Entry::or_default", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::node::Handle::insert_recursing<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global,alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >", "constructor": "alloc::collections::btree::node::Handle::insert_recursing", "arguments": ["tiny_glade::vram_snapshot::AllocationCategory", "tiny_glade::vram_snapshot::CategorySummary", "alloc::alloc::Global", "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter,alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter", "alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::vram_snapshot::MemoryBlockSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::vram_snapshot::MemoryBlockSnapshot,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::vram_snapshot::MemoryBlockSnapshot", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<tiny_glade::vram_snapshot::MemoryBlockSnapshot,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "alloc::vec::Vec<tiny_glade::vram_snapshot::MemoryBlockSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter,alloc::vec::Vec<tiny_glade::vram_snapshot::MemoryBlockSnapshot,alloc::alloc::Global> >", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter", "alloc::vec::Vec<tiny_glade::vram_snapshot::MemoryBlockSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::vram_snapshot::VramSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::vram_snapshot::VramSnapshot>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::vram_snapshot::VramSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: vram_snapshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: vram_snapshot.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 候选参数类型: ["ref$<tuple$<i64,usdl::video::DisplayMode> >", "ref$<tuple$<i64,usdl::video::DisplayMode> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>,bevy_ecs::change_detection::ResMut<enum2$<system_decorator::audio::op_preview_audio::PreviousFrameDecoratorPreviewOp> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>", "bevy_ecs::change_detection::ResMut<enum2$<system_decorator::audio::op_preview_audio::PreviousFrameDecoratorPreviewOp> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<system_decorator::audio::op_preview_audio::PreviousFrameDecoratorPreviewOp> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::_::impl$0::serialize
// 信息 ID: 66eebaca1b28190e17f6e66b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,tiny_glade::vram_snapshot::AllocationCategory>; module=144, inline_record=158464; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> >; module=144, inline_record=202392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 信息 ID: 2753e17c39a13e76c1bd2d00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter>"]; 字段布局 = unknown。
// [类型关系] tiny_glade::vram_snapshot::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global> > @ 0x45efaa -> 0x533fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<tiny_glade::vram_snapshot::MemoryBlockSnapshot,alloc::alloc::Global> > @ 0x45bb5a -> 0x534160 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram_snapshot::dump_vram_snapshot @ 0x197b83 -> 0x534290 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::collections::btree::map::BTreeMap<tiny_glade::vram_snapshot::AllocationCategory,tiny_glade::vram_snapshot::CategorySummary,alloc::alloc::Global> > @ 0x462c28 -> 0x5344f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::dump_vram_snapshot
// 信息 ID: 1783d89275797409d65f544d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$9,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68df82 -> 0x196cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$0
// 信息 ID: 7a97f98097e35f5dc9e92271
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::vram_snapshot::dump_vram_snapshot", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::vram_snapshot::dump_vram_snapshot; module=39, inline_record=1676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$3
// 信息 ID: 5a26d31c3122da5532b637ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::vram_snapshot::dump_vram_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3 <- alloc::vec::into_iter::impl$6::fold<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global,tuple$<>,core::iter::adapters::enumerate::impl$1::fold::enumerate::closure_env$0<gpu_allocator::allocator::AllocationReport,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3 <- alloc::vec::into_iter::impl$6::fold<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global,tuple$<>,core::iter::adapters::enumerate::impl$1::fold::enumerate::closure_env$0<gpu_allocator::allocator::AllocationReport,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3 <- alloc::vec::in_place_collect::impl$1::from_iter<tiny_glade::vram_snapshot::AllocationSnapshot,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global,tuple$<>,core::iter::adapters::enumerate::impl$1::fold::enumerate::closure_env$0<gpu_allocator::allocator::AllocationReport,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > > > >; module=8, inline_record=15140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$5
// 信息 ID: f43f0898e1f67240284b7042
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::vram_snapshot::dump_vram_snapshot", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::vram_snapshot::dump_vram_snapshot; module=39, inline_record=13164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$6
// 信息 ID: cbb04f8277070ea5665fa20e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::vram_snapshot::dump_vram_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$6 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$6>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$6>; module=235, inline_record=22656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::dump_vram_snapshot::closure$7
// 信息 ID: aa2154d470c97e0d9752faf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::vram_snapshot::dump_vram_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$7 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$7>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$7>; module=235, inline_record=19376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_snapshot::impl$3::cmp
// 信息 ID: a5be5136474bef03c5749b38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global,tuple$<>,core::iter::adapters::enumerate::impl$1::fold::enumerate::closure_env$0<gpu_allocator::allocator::AllocationReport,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<usize,gpu_allocator::allocator::AllocationReport>,tiny_glade::vram_snapshot::AllocationSnapshot,tuple$<>,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::vram_snapshot::AllocationSnapshot,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<gpu_allocator::allocator::AllocationReport,alloc::alloc::Global> >,tiny_glade::vram_snapshot::dump_vram_snapshot::closure_env$3> > > > > >; module=8, inline_record=16788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
