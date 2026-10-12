// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/asset_usage_snapshot.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0c6afc974efb2cc2cefbd293e4838adc53e789753a2cccdbefda54d73ad1a5f4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: asset_usage_snapshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 8, 55, 81, 112, 114, 130, 159, 188, 228, 235, 242
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::asset_usage_snapshot::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 记录形式: compiled_procedure; instances=3
// 源码行: 9, 15, 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":114,"record_offset":248,"rva":3852864,"va":5372561984}
// 编译实例 RVAs: 0x3aca40, 0x3acb70, 0x3acd30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-734533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562288,"raw_hex":"e8bbcaf4ff","size":5,"soft_fail":false,"target_va":5371827984,"va":5372562512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403aea80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562736,"raw_hex":"e8bb1b0000","size":5,"soft_fail":false,"target_va":5372570240,"va":5372563136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403aeaa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562736,"raw_hex":"e8f51b0000","size":5,"soft_fail":false,"target_va":5372570272,"va":5372563110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403aeac0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372561984,"raw_hex":"e8891f0000","size":5,"soft_fail":false,"target_va":5372570304,"va":5372562226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140460160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t734612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562288,"raw_hex":"e894350b00","size":5,"soft_fail":false,"target_va":5373296992,"va":5372562375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404604c0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t735784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372561984,"raw_hex":"e8283a0b00","size":5,"soft_fail":false,"target_va":5373297856,"va":5372562067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404620c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t742860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372561984,"raw_hex":"e8cc550b00","size":5,"soft_fail":false,"target_va":5373305024,"va":5372562159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t753634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562288,"raw_hex":"e8e27f0b00","size":5,"soft_fail":false,"target_va":5373316080,"va":5372562441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140467d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t766651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372561984,"raw_hex":"e8bbb20b00","size":5,"soft_fail":false,"target_va":5373328768,"va":5372562112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14046a670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t777042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372561984,"raw_hex":"e852db0b00","size":5,"soft_fail":false,"target_va":5373339248,"va":5372562201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43790770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372561984,"raw_hex":"e8b2319c02","size":5,"soft_fail":false,"target_va":5416353040,"va":5372562265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43995655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372561984,"raw_hex":"e807529f02","size":5,"soft_fail":false,"target_va":5416557744,"va":5372562084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44494483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562288,"raw_hex":"e893eea602","size":5,"soft_fail":false,"target_va":5417057056,"va":5372562568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t43995324","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372562288,"raw_hex":"e9bc509f02","size":5,"soft_fail":false,"target_va":5416557744,"va":5372562415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":114,"record_offset":520,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":114,"record_offset":2084,"parent_record_offset":1196,"ancestor_record_offset":1196,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":114,"record_offset":1068,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":114,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":114,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":248,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":114,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1196,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":114,"record_offset":848,"parent_record_offset":816,"ancestor_record_offset":248,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":114,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":114,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":114,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":248,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":114,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":248,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":114,"record_offset":888,"parent_record_offset":720,"ancestor_record_offset":248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":114,"record_offset":664,"parent_record_offset":636,"ancestor_record_offset":248,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":114,"record_offset":1008,"parent_record_offset":720,"ancestor_record_offset":248,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":114,"record_offset":760,"parent_record_offset":720,"ancestor_record_offset":248,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":114,"record_offset":692,"parent_record_offset":664,"ancestor_record_offset":248,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":114,"record_offset":2332,"parent_record_offset":2304,"ancestor_record_offset":1196,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":114,"record_offset":2304,"parent_record_offset":2276,"ancestor_record_offset":1196,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":114,"record_offset":2276,"parent_record_offset":2240,"ancestor_record_offset":1196,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":114,"record_offset":2692,"parent_record_offset":2484,"ancestor_record_offset":1196,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::begin_object
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":114,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":248,"inlinee":12492,"source_line":2018,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::end_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":114,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1196,"inlinee":12493,"source_line":2028,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":114,"record_offset":548,"parent_record_offset":520,"ancestor_record_offset":248,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":114,"record_offset":2116,"parent_record_offset":2084,"ancestor_record_offset":1196,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":114,"record_offset":2240,"parent_record_offset":2144,"ancestor_record_offset":1196,"inlinee":8104,"source_line":2276,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":114,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":248,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":114,"record_offset":608,"parent_record_offset":576,"ancestor_record_offset":248,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00114.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":242,"record_offset":1552,"rva":6555664,"va":5375264784}
// 编译实例 RVAs: 0x640810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9424364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8eccd8f00","size":5,"soft_fail":false,"target_va":5384689616,"va":5375265247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9391293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8bd4c8f00","size":5,"soft_fail":false,"target_va":5384656512,"va":5375265214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t413390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8ce4e0600","size":5,"soft_fail":false,"target_va":5375678688,"va":5375265293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t413274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e85a4e0600","size":5,"soft_fail":false,"target_va":5375678704,"va":5375265425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6543091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e80d299cff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375265278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b6ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5807098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e80664a7ff","size":5,"soft_fail":false,"target_va":5369458384,"va":5375265477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c1c40
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5763262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8420fa8ff","size":5,"soft_fail":false,"target_va":5369502784,"va":5375266041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c39d0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5755807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8612ca8ff","size":5,"soft_fail":false,"target_va":5369510352,"va":5375266154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d32e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4643039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e82127b9ff","size":5,"soft_fail":false,"target_va":5370622688,"va":5375265722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d3ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4640942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8522fb9ff","size":5,"soft_fail":false,"target_va":5370624720,"va":5375265657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403acd30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2702655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8c1c2d6ff","size":5,"soft_fail":false,"target_va":5372562736,"va":5375265386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2311770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8a6b9dcff","size":5,"soft_fail":false,"target_va":5372953248,"va":5375265013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040c570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2310942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8e2bcdcff","size":5,"soft_fail":false,"target_va":5372953968,"va":5375264905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405490c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1014586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8c684f0ff","size":5,"soft_fail":false,"target_va":5374251200,"va":5375265781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1014214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e83a86f0ff","size":5,"soft_fail":false,"target_va":5374251600,"va":5375265809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405faaa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-286868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e86c9ffbff","size":5,"soft_fail":false,"target_va":5374978720,"va":5375265583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fb100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-284913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e80fa7fbff","size":5,"soft_fail":false,"target_va":5374980352,"va":5375265260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a13df0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4011216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8d0343d00","size":5,"soft_fail":false,"target_va":5379276272,"va":5375265051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: asset_usage_snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41056714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8ca797202","size":5,"soft_fail":false,"target_va":5416322176,"va":5375265457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41355033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e819077702","size":5,"soft_fail":false,"target_va":5416620794,"va":5375265756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::asset_count
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4000490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8ea0a3d00","size":5,"soft_fail":false,"target_va":5379265584,"va":5375265089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::never_accessed_assets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4005826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8c21f3d00","size":5,"soft_fail":false,"target_va":5379270784,"va":5375264953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::never_accessed_assets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4014838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e8f6423d00","size":5,"soft_fail":false,"target_va":5379279696,"va":5375264853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9486923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375264784,"raw_hex":"e84bc29000","size":5,"soft_fail":false,"target_va":5384752464,"va":5375265536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":242,"record_offset":1948,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":242,"record_offset":1720,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":242,"record_offset":1836,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":242,"record_offset":3852,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":242,"record_offset":2860,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":8087,"source_line":2264,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":2776,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":3880,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":1552,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":1552,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":1552,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":1552,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":1552,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":1552,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":3112,"parent_record_offset":3084,"ancestor_record_offset":1552,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":3460,"parent_record_offset":3404,"ancestor_record_offset":1552,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":3024,"parent_record_offset":2996,"ancestor_record_offset":1552,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":3432,"parent_record_offset":3404,"ancestor_record_offset":1552,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":1552,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":3084,"parent_record_offset":3052,"ancestor_record_offset":1552,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":1552,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":1552,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":242,"record_offset":1980,"parent_record_offset":1948,"ancestor_record_offset":1552,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":242,"record_offset":2968,"parent_record_offset":2908,"ancestor_record_offset":1552,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":242,"record_offset":2996,"parent_record_offset":2968,"ancestor_record_offset":1552,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":242,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1552,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":242,"record_offset":1912,"parent_record_offset":1860,"ancestor_record_offset":1552,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":242,"record_offset":2052,"parent_record_offset":2012,"ancestor_record_offset":1552,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":242,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1552,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":242,"record_offset":1748,"parent_record_offset":1720,"ancestor_record_offset":1552,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":242,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":1552,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":3332,"parent_record_offset":2908,"ancestor_record_offset":1552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":242,"record_offset":1804,"parent_record_offset":1720,"ancestor_record_offset":1552,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":242,"record_offset":2108,"parent_record_offset":2012,"ancestor_record_offset":1552,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":242,"record_offset":2012,"parent_record_offset":1980,"ancestor_record_offset":1552,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":242,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":1552,"inlinee":8091,"source_line":39,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":242,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":1552,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":242,"record_offset":2908,"parent_record_offset":2860,"ancestor_record_offset":1552,"inlinee":8088,"source_line":2229,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":242,"record_offset":3244,"parent_record_offset":2908,"ancestor_record_offset":1552,"inlinee":8089,"source_line":2197,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":2828,"parent_record_offset":2776,"ancestor_record_offset":1552,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1552,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156A
// 首条位置: {"module":8,"inlinee":5482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x24FA
// 首条位置: {"module":55,"inlinee":9466}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$19::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":55,"record_offset":10620,"parent_record_offset":10592,"ancestor_record_offset":10592,"inlinee":9468,"source_line":2457,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$84::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":55,"record_offset":10652,"parent_record_offset":10620,"ancestor_record_offset":10592,"inlinee":9450,"source_line":350,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":55,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":10592,"inlinee":9440,"source_line":4212,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::compare
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":55,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":10592,"inlinee":9435,"source_line":311,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$2::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":55,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":10592,"inlinee":9442,"source_line":37,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x157E
// 首条位置: {"module":8,"inlinee":5502}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=30
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x24DE
// 首条位置: {"module":55,"inlinee":9438}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$25::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":55,"record_offset":2976,"parent_record_offset":2956,"ancestor_record_offset":2956,"inlinee":9440,"source_line":4212,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$84::cmp
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":55,"record_offset":3320,"parent_record_offset":3300,"ancestor_record_offset":2956,"inlinee":9450,"source_line":350,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::cmp
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":55,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":2956,"inlinee":9440,"source_line":4212,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":55,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":2956,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":55,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":2956,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":55,"record_offset":3080,"parent_record_offset":3048,"ancestor_record_offset":2956,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":55,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":2956,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":55,"record_offset":3220,"parent_record_offset":3048,"ancestor_record_offset":2956,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":55,"record_offset":3440,"parent_record_offset":3048,"ancestor_record_offset":2956,"inlinee":5504,"source_line":115,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::chaining_impl
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":55,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":2956,"inlinee":9446,"source_line":206,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::compare
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":55,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2956,"inlinee":9435,"source_line":311,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$2::cmp
//     owner=rust_standard_library; count=52
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":55,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":2956,"inlinee":9442,"source_line":37,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$9::compare
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":55,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":2956,"inlinee":9444,"source_line":274,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$9::compare::closure$0
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":55,"record_offset":3300,"parent_record_offset":3048,"ancestor_record_offset":2956,"inlinee":9448,"source_line":275,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":55,"record_offset":3136,"parent_record_offset":3048,"ancestor_record_offset":2956,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":55,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2956,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":55,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2956,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/00055.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43C1
// 首条位置: {"module":235,"inlinee":17345}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":23400,"parent_record_offset":23380,"ancestor_record_offset":23380,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":23448,"parent_record_offset":23424,"ancestor_record_offset":23380,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":235,"record_offset":23424,"parent_record_offset":23400,"ancestor_record_offset":23380,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43BB
// 首条位置: {"module":235,"inlinee":17339}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":20120,"parent_record_offset":20100,"ancestor_record_offset":20100,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":20168,"parent_record_offset":20144,"ancestor_record_offset":20100,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":235,"record_offset":20144,"parent_record_offset":20120,"ancestor_record_offset":20100,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00235.jsonl.gz"}
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
// 完整结构及原始类型记录: asset_usage_snapshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::asset_usage_snapshot::AssetUsageSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::asset_usage_snapshot::AssetUsageSnapshot>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::asset_usage_snapshot::AssetUsageSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::asset_usage_snapshot::MeshSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insert_tail<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >", "constructor": "core::slice::sort::shared::smallsort::insert_tail", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global> >", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter", "alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,tiny_glade::asset_usage_snapshot::MeshSnapshot>", "constructor": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop", "arguments": ["country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset", "tiny_glade::asset_usage_snapshot::MeshSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,tiny_glade::asset_usage_snapshot::MeshSnapshot> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,tiny_glade::asset_usage_snapshot::MeshSnapshot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$2>,tiny_glade::asset_usage_snapshot::MeshSnapshot>", "constructor": "alloc::vec::in_place_collect::from_iter_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$2>", "tiny_glade::asset_usage_snapshot::MeshSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["tiny_glade::asset_usage_snapshot::MeshSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>", "alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::asset_usage_snapshot::TextureSnapshot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insert_tail<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insert_tail", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global> >", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter", "alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::texture_library::NeverAccessedTextureAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$0>,tiny_glade::asset_usage_snapshot::TextureSnapshot>", "constructor": "alloc::vec::in_place_collect::from_iter_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::texture_library::NeverAccessedTextureAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$0>", "tiny_glade::asset_usage_snapshot::TextureSnapshot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["tiny_glade::asset_usage_snapshot::TextureSnapshot", "alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>", "alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: asset_usage_snapshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: asset_usage_snapshot.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 23; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >", "ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 候选参数类型: ["ref$<tuple$<i64,usdl::video::DisplayMode> >", "ref$<tuple$<i64,usdl::video::DisplayMode> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >)
// 候选参数类型: ["ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >", "ref$<enum2$<rhapsody::raw::init::GraphicsLayerAllowlistItem> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::asset_usage_snapshot::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 信息 ID: 7c4e4f43927097043fe24a61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter>"]; 字段布局 = unknown。
// [类型关系] tiny_glade::asset_usage_snapshot::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global> > @ 0x4697fa -> 0x3aca40 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global> > @ 0x46777a -> 0x3acb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot @ 0x640a6a -> 0x3acd30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot
// 信息 ID: b474eef313e3ec88cefeb4e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$9,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68e330 -> 0x640810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$0
// 信息 ID: 2aa6a1020f5c70bd08bed3d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$0 <- alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::texture_library::NeverAccessedTextureAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$0>,tiny_glade::asset_usage_snapshot::TextureSnapshot>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::texture_library::NeverAccessedTextureAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$0>,tiny_glade::asset_usage_snapshot::TextureSnapshot>; module=130, inline_record=7684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$1
// 信息 ID: d4449e205f139cbf1eec145f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1 <- core::slice::sort::shared::smallsort::insert_tail<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1 <- core::slice::sort::stable::merge::merge<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1 <- core::slice::sort::stable::drift::sort<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1 <- core::slice::sort::stable::driftsort_main<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1>,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; module=55, inline_record=10592; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; module=55, inline_record=10948; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; module=55, inline_record=32684; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; module=55, inline_record=32944; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; module=55, inline_record=33340; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; module=55, inline_record=33612; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; module=55, inline_record=34000; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tiny_glade::asset_usage_snapshot::TextureSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::TextureSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$1> >; module=55, inline_record=57848; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$2
// 信息 ID: 38fc7de93ec71cfeb0c104e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$2 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$2 <- alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$2>,tiny_glade::asset_usage_snapshot::MeshSnapshot>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,alloc::alloc::Global>,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$2>,tiny_glade::asset_usage_snapshot::MeshSnapshot>; module=130, inline_record=5900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$3
// 信息 ID: 0483ec8aafbb34f42626d70d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3 <- core::slice::sort::shared::smallsort::insert_tail<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3 <- core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3 <- core::slice::sort::shared::pivot::median3_rec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3 <- core::slice::sort::stable::drift::sort<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3 <- core::slice::sort::stable::quicksort::quicksort<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3 <- core::slice::sort::stable::driftsort_main<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3>,alloc::vec::Vec<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; module=55, inline_record=2956; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; module=55, inline_record=3696; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; module=55, inline_record=26164; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; module=55, inline_record=26748; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; module=55, inline_record=27472; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; module=55, inline_record=28080; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; module=55, inline_record=28804; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tiny_glade::asset_usage_snapshot::MeshSnapshot,alloc::slice::impl$0::sort_by::closure_env$0<tiny_glade::asset_usage_snapshot::MeshSnapshot,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$3> >; module=55, inline_record=74148; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$4
// 信息 ID: 00d5ee2a7bd6abaf52c8917c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$4 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$4>; module=235, inline_record=23380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure$5
// 信息 ID: b09c411674544775f63dddf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$5 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot::closure_env$5>; module=235, inline_record=20100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
