// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/utils/load_json.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c8f0cfa2333f2c0447035192a57be11dc3f08de1c765ac549d1f49b77c2b5980
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_json.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 521, 560, 1253, 1590, 1643, 1682, 1770, 1774, 3212, 4053, 4808
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":521,"record_offset":29376,"rva":10329424,"va":5379038544}
// 编译实例 RVAs: 0x9d9d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5619999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e81fc15500","size":5,"soft_fail":false,"target_va":5384659264,"va":5379039260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5614808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8d8ac5500","size":5,"soft_fail":false,"target_va":5384653728,"va":5379038915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$$u5b$f32$u3b$$u20$2$u5d$$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4620351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e83f804600","size":5,"soft_fail":false,"target_va":5383661952,"va":5379041596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$$u5b$f32$u3b$$u20$3$u5d$$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4620562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e812814600","size":5,"soft_fail":false,"target_va":5383662064,"va":5379041497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$$u5b$f32$u3b$$u20$4$u5d$$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4620940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e88c824600","size":5,"soft_fail":false,"target_va":5383662176,"va":5379041231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$f32$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4620296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e808804600","size":5,"soft_fail":false,"target_va":5383661200,"va":5379040899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$i32$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4619912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8887e4600","size":5,"soft_fail":false,"target_va":5383661312,"va":5379041395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t-3360250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e806baccff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379038949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9378411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e895e570ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5379038774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-302977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e87f60fbff","size":5,"soft_fail":false,"target_va":5378736416,"va":5379039388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-301754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e84665fbff","size":5,"soft_fail":false,"target_va":5378737680,"va":5379039429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a9960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-198587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e845f8fcff","size":5,"soft_fail":false,"target_va":5378840928,"va":5379039510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409de280
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e86c360000","size":5,"soft_fail":false,"target_va":5379056256,"va":5379042319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac7170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t968551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e867c70e00","size":5,"soft_fail":false,"target_va":5380010352,"va":5379041796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0b510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1248276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8140c1300","size":5,"soft_fail":false,"target_va":5380289808,"va":5379041527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0b780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1248999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8e70e1300","size":5,"soft_fail":false,"target_va":5380290432,"va":5379041428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0bac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1250441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e889141300","size":5,"soft_fail":false,"target_va":5380291264,"va":5379040818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b0050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8217239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e897627d00","size":5,"soft_fail":false,"target_va":5387255888,"va":5379038644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ff770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12734285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e84d4fc200","size":5,"soft_fail":false,"target_va":5391775600,"va":5379041310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141607560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12768702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8bed5c200","size":5,"soft_fail":false,"target_va":5391807840,"va":5379039133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141610490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12804384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e82061c300","size":5,"soft_fail":false,"target_va":5391844496,"va":5379040107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141619c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12842177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8c1f4c300","size":5,"soft_fail":false,"target_va":5391883328,"va":5379041146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141651ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13074732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e82c81c700","size":5,"soft_fail":false,"target_va":5392113360,"va":5379038623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ad9ef0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17825749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8d5ff0f01","size":5,"soft_fail":false,"target_va":5396864752,"va":5379038998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37720552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8e8913f02","size":5,"soft_fail":false,"target_va":5416761616,"va":5379041059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd39c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37719216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8b08c3f02","size":5,"soft_fail":false,"target_va":5416761792,"va":5379042571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37720459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e88b913f02","size":5,"soft_fail":false,"target_va":5416762432,"va":5379041968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4a20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37726644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8b4a93f02","size":5,"soft_fail":false,"target_va":5416765984,"va":5379039335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8e2d1ffff","size":5,"soft_fail":false,"target_va":5379030304,"va":5379042105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t5755619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8e3d25700","size":5,"soft_fail":false,"target_va":5384794864,"va":5379039240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5581310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8fe295500","size":5,"soft_fail":false,"target_va":5384624080,"va":5379042765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::legacy_mod::flip_triangle_winding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17870293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8d5ad1001","size":5,"soft_fail":false,"target_va":5396910224,"va":5379039926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::legacy_mod::is_shipped_era_mod_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17870528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8c0ae1001","size":5,"soft_fail":false,"target_va":5396910432,"va":5379039899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::vertex::VertexAttributeValues> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e843380000","size":5,"soft_fail":false,"target_va":5379056576,"va":5379042168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37579560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e8286b3d02","size":5,"soft_fail":false,"target_va":5416619360,"va":5379039795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4921348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379038544,"raw_hex":"e804184b00","size":5,"soft_fail":false,"target_va":5383960112,"va":5379038759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":35372,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":521,"record_offset":35428,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":521,"record_offset":39744,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":17362,"source_line":58,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::with_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":521,"record_offset":30776,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":30730,"source_line":103,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":521,"record_offset":40544,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":521,"record_offset":31340,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":521,"record_offset":33068,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":15785,"source_line":3228,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":40420,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":33696,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":32028,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":29604,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":29640,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":521,"record_offset":31256,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":521,"record_offset":29752,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":521,"record_offset":33724,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":30734,"source_line":70,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_source_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":521,"record_offset":30156,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":30724,"source_line":54,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::load_json::read_attribute_from_json
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":521,"record_offset":35556,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":30737,"source_line":116,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":521,"record_offset":30748,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":30729,"source_line":425,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::get
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":521,"record_offset":33640,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":30733,"source_line":305,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/index.rs
//     evidence={"module":521,"record_offset":30580,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":30726,"source_line":212,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":521,"record_offset":30128,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":521,"record_offset":29696,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":521,"record_offset":29668,"parent_record_offset":29376,"ancestor_record_offset":29376,"inlinee":27983,"source_line":469,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":30452,"parent_record_offset":30428,"ancestor_record_offset":29376,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":30476,"parent_record_offset":30452,"ancestor_record_offset":29376,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":30500,"parent_record_offset":30476,"ancestor_record_offset":29376,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":30428,"parent_record_offset":30372,"ancestor_record_offset":29376,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":30852,"parent_record_offset":30828,"ancestor_record_offset":29376,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":30900,"parent_record_offset":30876,"ancestor_record_offset":29376,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":32136,"parent_record_offset":32112,"ancestor_record_offset":29376,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":30400,"parent_record_offset":30372,"ancestor_record_offset":29376,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":30372,"parent_record_offset":30348,"ancestor_record_offset":29376,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":32184,"parent_record_offset":32160,"ancestor_record_offset":29376,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":32160,"parent_record_offset":32136,"ancestor_record_offset":29376,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":30348,"parent_record_offset":30324,"ancestor_record_offset":29376,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":32112,"parent_record_offset":32088,"ancestor_record_offset":29376,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":31508,"parent_record_offset":31484,"ancestor_record_offset":29376,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":32088,"parent_record_offset":32056,"ancestor_record_offset":29376,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":521,"record_offset":40016,"parent_record_offset":39992,"ancestor_record_offset":29376,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":521,"record_offset":40064,"parent_record_offset":40040,"ancestor_record_offset":29376,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":521,"record_offset":30940,"parent_record_offset":30776,"ancestor_record_offset":29376,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":521,"record_offset":39920,"parent_record_offset":39896,"ancestor_record_offset":29376,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":521,"record_offset":39896,"parent_record_offset":39744,"ancestor_record_offset":29376,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":521,"record_offset":35936,"parent_record_offset":35556,"ancestor_record_offset":29376,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":521,"record_offset":29988,"parent_record_offset":29960,"ancestor_record_offset":29376,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":521,"record_offset":30040,"parent_record_offset":30016,"ancestor_record_offset":29376,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":521,"record_offset":30016,"parent_record_offset":29960,"ancestor_record_offset":29376,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":521,"record_offset":36288,"parent_record_offset":36264,"ancestor_record_offset":29376,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":521,"record_offset":33552,"parent_record_offset":33524,"ancestor_record_offset":29376,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":521,"record_offset":36888,"parent_record_offset":35556,"ancestor_record_offset":29376,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":521,"record_offset":37112,"parent_record_offset":35556,"ancestor_record_offset":29376,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":521,"record_offset":33296,"parent_record_offset":33124,"ancestor_record_offset":29376,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":521,"record_offset":33156,"parent_record_offset":33124,"ancestor_record_offset":29376,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":521,"record_offset":33496,"parent_record_offset":33324,"ancestor_record_offset":29376,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::try_process
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\mod.rs
//     evidence={"module":521,"record_offset":31412,"parent_record_offset":31376,"ancestor_record_offset":29376,"inlinee":11759,"source_line":152,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":521,"record_offset":31532,"parent_record_offset":31508,"ancestor_record_offset":29376,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":521,"record_offset":31484,"parent_record_offset":31460,"ancestor_record_offset":29376,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":521,"record_offset":33096,"parent_record_offset":33068,"ancestor_record_offset":29376,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":521,"record_offset":33524,"parent_record_offset":33496,"ancestor_record_offset":29376,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":521,"record_offset":33124,"parent_record_offset":33096,"ancestor_record_offset":29376,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":521,"record_offset":30064,"parent_record_offset":30040,"ancestor_record_offset":29376,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":521,"record_offset":33592,"parent_record_offset":33324,"ancestor_record_offset":29376,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":521,"record_offset":31704,"parent_record_offset":31672,"ancestor_record_offset":29376,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":521,"record_offset":33260,"parent_record_offset":33236,"ancestor_record_offset":29376,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":40508,"parent_record_offset":40452,"ancestor_record_offset":29376,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":33236,"parent_record_offset":33156,"ancestor_record_offset":29376,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":30876,"parent_record_offset":30852,"ancestor_record_offset":29376,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":30632,"parent_record_offset":30580,"ancestor_record_offset":29376,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":521,"record_offset":29932,"parent_record_offset":29904,"ancestor_record_offset":29376,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":521,"record_offset":33460,"parent_record_offset":33436,"ancestor_record_offset":29376,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":30180,"parent_record_offset":30156,"ancestor_record_offset":29376,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":521,"record_offset":31304,"parent_record_offset":31280,"ancestor_record_offset":29376,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":521,"record_offset":33436,"parent_record_offset":33412,"ancestor_record_offset":29376,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":521,"record_offset":33412,"parent_record_offset":33324,"ancestor_record_offset":29376,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":521,"record_offset":35496,"parent_record_offset":35428,"ancestor_record_offset":29376,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":521,"record_offset":33204,"parent_record_offset":33180,"ancestor_record_offset":29376,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":35520,"parent_record_offset":35496,"ancestor_record_offset":29376,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::from_iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":31376,"parent_record_offset":31340,"ancestor_record_offset":29376,"inlinee":23302,"source_line":2155,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::from_iter::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":31460,"parent_record_offset":31412,"ancestor_record_offset":29376,"inlinee":23304,"source_line":2156,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":36856,"parent_record_offset":35556,"ancestor_record_offset":29376,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":521,"record_offset":36312,"parent_record_offset":36288,"ancestor_record_offset":29376,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":521,"record_offset":36340,"parent_record_offset":36312,"ancestor_record_offset":29376,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":521,"record_offset":36716,"parent_record_offset":35556,"ancestor_record_offset":29376,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":521,"record_offset":31280,"parent_record_offset":31256,"ancestor_record_offset":29376,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":521,"record_offset":33324,"parent_record_offset":33296,"ancestor_record_offset":29376,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":521,"record_offset":33180,"parent_record_offset":33156,"ancestor_record_offset":29376,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":36264,"parent_record_offset":35556,"ancestor_record_offset":29376,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_str_as_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":521,"record_offset":30828,"parent_record_offset":30776,"ancestor_record_offset":29376,"inlinee":30732,"source_line":60,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_str_as_mesh::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":521,"record_offset":34208,"parent_record_offset":34156,"ancestor_record_offset":29376,"inlinee":30735,"source_line":79,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_str_as_mesh::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":521,"record_offset":39784,"parent_record_offset":39744,"ancestor_record_offset":29376,"inlinee":30740,"source_line":89,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_str_as_mesh::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":521,"record_offset":40452,"parent_record_offset":40420,"ancestor_record_offset":29376,"inlinee":30738,"source_line":111,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":521,"record_offset":35908,"parent_record_offset":35556,"ancestor_record_offset":29376,"inlinee":30729,"source_line":425,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_bool
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":521,"record_offset":40480,"parent_record_offset":40452,"ancestor_record_offset":29376,"inlinee":30739,"source_line":708,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/index.rs
//     evidence={"module":521,"record_offset":30604,"parent_record_offset":30580,"ancestor_record_offset":29376,"inlinee":30728,"source_line":123,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$5::index
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/index.rs
//     evidence={"module":521,"record_offset":35740,"parent_record_offset":35556,"ancestor_record_offset":29376,"inlinee":30726,"source_line":212,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":521,"record_offset":29788,"parent_record_offset":29752,"ancestor_record_offset":29376,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":521,"record_offset":29844,"parent_record_offset":29816,"ancestor_record_offset":29376,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":521,"record_offset":29960,"parent_record_offset":29904,"ancestor_record_offset":29376,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":521,"record_offset":29816,"parent_record_offset":29788,"ancestor_record_offset":29376,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":521,"record_offset":29904,"parent_record_offset":29872,"ancestor_record_offset":29376,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":521,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":29376,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":521,"record_offset":29720,"parent_record_offset":29696,"ancestor_record_offset":29376,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<alloc::string::String>
// 记录形式: compiled_procedure; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1590,"record_offset":5692,"rva":21885008,"va":5390594128}
// 编译实例 RVAs: 0x14df050, 0x1aedab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5985562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8e6aaa4ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5390594965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-14916446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8a2641cff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390595145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18210114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396945584,"raw_hex":"e8be22eafe","size":5,"soft_fail":false,"target_va":5378736416,"va":5396946525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414aa360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-217224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e878affcff","size":5,"soft_fail":false,"target_va":5390377824,"va":5390595043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414df7e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e826050000","size":5,"soft_fail":false,"target_va":5390596064,"va":5390594741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4166468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396945584,"raw_hex":"e8bc6cc0ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5396947231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aef020
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396945584,"raw_hex":"e8ec120000","size":5,"soft_fail":false,"target_va":5396951072,"va":5396946223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19438236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396945584,"raw_hex":"e89c9a2801","size":5,"soft_fail":false,"target_va":5416385024,"va":5396946783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5970950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8fae3a4ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5390595025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26026062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e84e208d01","size":5,"soft_fail":false,"target_va":5416621408,"va":5390595341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26026310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e846218d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390595285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e873020000","size":5,"soft_fail":false,"target_va":5390596592,"va":5390595960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5764451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e89d0aa8ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5390594526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e853090000","size":5,"soft_fail":false,"target_va":5390596624,"va":5390594232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26025939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8d31f8d01","size":5,"soft_fail":false,"target_va":5416621376,"va":5390595432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6630835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e84dd29aff","size":5,"soft_fail":false,"target_va":5383963792,"va":5390594622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6622331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e885f39aff","size":5,"soft_fail":false,"target_va":5383972224,"va":5390594550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6630299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e865d49aff","size":5,"soft_fail":false,"target_va":5383964912,"va":5390595206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6631380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e82cd09aff","size":5,"soft_fail":false,"target_va":5383962816,"va":5390594191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26122390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e896988e01","size":5,"soft_fail":false,"target_va":5416717696,"va":5390595301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-228500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e86c83fcff","size":5,"soft_fail":false,"target_va":5390367248,"va":5390595743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26024032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e860188d01","size":5,"soft_fail":false,"target_va":5416619360,"va":5390595323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7710279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8b9598aff","size":5,"soft_fail":false,"target_va":5382884432,"va":5390594706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7675037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e863e38aff","size":5,"soft_fail":false,"target_va":5382919648,"va":5390594680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1590,"record_offset":8372,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1590,"record_offset":7392,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1590,"record_offset":8540,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1590,"record_offset":8428,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":8288,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":7428,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":5880,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":6268,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1590,"record_offset":7280,"parent_record_offset":5692,"ancestor_record_offset":5692,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":8844,"parent_record_offset":8820,"ancestor_record_offset":5692,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":8868,"parent_record_offset":8844,"ancestor_record_offset":5692,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":5692,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":8820,"parent_record_offset":8764,"ancestor_record_offset":5692,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":8792,"parent_record_offset":8764,"ancestor_record_offset":5692,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":5692,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":8740,"parent_record_offset":8716,"ancestor_record_offset":5692,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":7808,"parent_record_offset":7780,"ancestor_record_offset":5692,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":7836,"parent_record_offset":7808,"ancestor_record_offset":5692,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":7748,"parent_record_offset":7716,"ancestor_record_offset":5692,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":7780,"parent_record_offset":7748,"ancestor_record_offset":5692,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":7896,"parent_record_offset":7780,"ancestor_record_offset":5692,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":5692,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1590,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":5692,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1590,"record_offset":7608,"parent_record_offset":7556,"ancestor_record_offset":5692,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1590,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":5692,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1590,"record_offset":6216,"parent_record_offset":6188,"ancestor_record_offset":5692,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1590,"record_offset":8244,"parent_record_offset":7556,"ancestor_record_offset":5692,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1590,"record_offset":7960,"parent_record_offset":7716,"ancestor_record_offset":5692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1590,"record_offset":6688,"parent_record_offset":6664,"ancestor_record_offset":5692,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1590,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":5692,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1590,"record_offset":6872,"parent_record_offset":6844,"ancestor_record_offset":5692,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1590,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":5692,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1590,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":5692,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1590,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":5692,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1590,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":5692,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1590,"record_offset":7000,"parent_record_offset":6972,"ancestor_record_offset":5692,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1590,"record_offset":6592,"parent_record_offset":6476,"ancestor_record_offset":5692,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1590,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":5692,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1590,"record_offset":6304,"parent_record_offset":6268,"ancestor_record_offset":5692,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1590,"record_offset":6444,"parent_record_offset":6412,"ancestor_record_offset":5692,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1590,"record_offset":6412,"parent_record_offset":6376,"ancestor_record_offset":5692,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1590,"record_offset":6352,"parent_record_offset":6328,"ancestor_record_offset":5692,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1590,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":5692,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1590,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":5692,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1590,"record_offset":6916,"parent_record_offset":6376,"ancestor_record_offset":5692,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":5984,"parent_record_offset":5952,"ancestor_record_offset":5692,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":5952,"parent_record_offset":5924,"ancestor_record_offset":5692,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":6108,"parent_record_offset":6076,"ancestor_record_offset":5692,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":6164,"parent_record_offset":6108,"ancestor_record_offset":5692,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":6076,"parent_record_offset":6048,"ancestor_record_offset":5692,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":5924,"parent_record_offset":5880,"ancestor_record_offset":5692,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":5692,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1590,"record_offset":6476,"parent_record_offset":6444,"ancestor_record_offset":5692,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1590,"record_offset":6760,"parent_record_offset":6476,"ancestor_record_offset":5692,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1590,"record_offset":6732,"parent_record_offset":6476,"ancestor_record_offset":5692,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1590,"record_offset":6536,"parent_record_offset":6476,"ancestor_record_offset":5692,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1590,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":5692,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1590,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":5692,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1590,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":5692,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":5692,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1590,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":5692,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":5692,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":7468,"parent_record_offset":7428,"ancestor_record_offset":5692,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":9028,"parent_record_offset":8996,"ancestor_record_offset":5692,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":9500,"parent_record_offset":9284,"ancestor_record_offset":5692,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1590,"record_offset":7640,"parent_record_offset":7608,"ancestor_record_offset":5692,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":7716,"parent_record_offset":7556,"ancestor_record_offset":5692,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":7932,"parent_record_offset":7716,"ancestor_record_offset":5692,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1590,"record_offset":7304,"parent_record_offset":7280,"ancestor_record_offset":5692,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1590,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":5692,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1590,"record_offset":7672,"parent_record_offset":7640,"ancestor_record_offset":5692,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1590,"record_offset":7556,"parent_record_offset":7508,"ancestor_record_offset":5692,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1590,"record_offset":7508,"parent_record_offset":7468,"ancestor_record_offset":5692,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":16248,"rva":28238432,"va":5396947552}
// 编译实例 RVAs: 0x1aee260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12338994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e8ceb843ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396948397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21269870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e89272bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396948569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18212056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e8281beafe","size":5,"soft_fail":false,"target_va":5378736416,"va":5396948467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4168228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e8dc65c0ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5396948991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aef020
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e84e0b0000","size":5,"soft_fail":false,"target_va":5396951072,"va":5396948173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19436326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e826932801","size":5,"soft_fail":false,"target_va":5416385024,"va":5396948693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12324374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e8eaf143ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5396948449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19672732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e89c2e2c01","size":5,"soft_fail":false,"target_va":5416621408,"va":5396948671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19672912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e8502f2c01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396948683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e803650000","size":5,"soft_fail":false,"target_va":5396975072,"va":5396949208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12117869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e8931847ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5396947944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e83d6b0000","size":5,"soft_fail":false,"target_va":5396975104,"va":5396947646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19672601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e8192e2c01","size":5,"soft_fail":false,"target_va":5416621376,"va":5396948770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12984257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e83fe039ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5396948044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12975758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e872013aff","size":5,"soft_fail":false,"target_va":5383972224,"va":5396947977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12983718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e85ae239ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5396948625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12984789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e82bde39ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5396947600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19670620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e85c262c01","size":5,"soft_fail":false,"target_va":5416619360,"va":5396948735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14063715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e89d6729ff","size":5,"soft_fail":false,"target_va":5382884432,"va":5396948142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14028473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396947552,"raw_hex":"e847f129ff","size":5,"soft_fail":false,"target_va":5382919648,"va":5396948116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":18908,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1774,"record_offset":16440,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":17980,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":19076,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":18964,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":18824,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":18016,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":16468,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":16856,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1774,"record_offset":17868,"parent_record_offset":16248,"ancestor_record_offset":16248,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":19380,"parent_record_offset":19356,"ancestor_record_offset":16248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":19404,"parent_record_offset":19380,"ancestor_record_offset":16248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":19428,"parent_record_offset":19404,"ancestor_record_offset":16248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":19356,"parent_record_offset":19300,"ancestor_record_offset":16248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":19328,"parent_record_offset":19300,"ancestor_record_offset":16248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":19300,"parent_record_offset":19276,"ancestor_record_offset":16248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":19276,"parent_record_offset":19252,"ancestor_record_offset":16248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":18344,"parent_record_offset":18316,"ancestor_record_offset":16248,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":18372,"parent_record_offset":18344,"ancestor_record_offset":16248,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":18292,"parent_record_offset":18264,"ancestor_record_offset":16248,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":18316,"parent_record_offset":18292,"ancestor_record_offset":16248,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":18432,"parent_record_offset":18316,"ancestor_record_offset":16248,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":18544,"parent_record_offset":18520,"ancestor_record_offset":16248,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1774,"record_offset":18396,"parent_record_offset":18372,"ancestor_record_offset":16248,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1774,"record_offset":18164,"parent_record_offset":18124,"ancestor_record_offset":16248,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":18932,"parent_record_offset":18908,"ancestor_record_offset":16248,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1774,"record_offset":16804,"parent_record_offset":16776,"ancestor_record_offset":16248,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1774,"record_offset":18780,"parent_record_offset":18124,"ancestor_record_offset":16248,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":18496,"parent_record_offset":18264,"ancestor_record_offset":16248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":17276,"parent_record_offset":17252,"ancestor_record_offset":16248,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":17252,"parent_record_offset":17228,"ancestor_record_offset":16248,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":17460,"parent_record_offset":17432,"ancestor_record_offset":16248,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1774,"record_offset":17560,"parent_record_offset":17532,"ancestor_record_offset":16248,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1774,"record_offset":17532,"parent_record_offset":17504,"ancestor_record_offset":16248,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":16248,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":16248,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":17588,"parent_record_offset":17560,"ancestor_record_offset":16248,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":17180,"parent_record_offset":17064,"ancestor_record_offset":16248,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1774,"record_offset":16596,"parent_record_offset":16572,"ancestor_record_offset":16248,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1774,"record_offset":16892,"parent_record_offset":16856,"ancestor_record_offset":16248,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1774,"record_offset":17032,"parent_record_offset":17000,"ancestor_record_offset":16248,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1774,"record_offset":17000,"parent_record_offset":16964,"ancestor_record_offset":16248,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":16940,"parent_record_offset":16916,"ancestor_record_offset":16248,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":16964,"parent_record_offset":16940,"ancestor_record_offset":16248,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":16916,"parent_record_offset":16892,"ancestor_record_offset":16248,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":17504,"parent_record_offset":16964,"ancestor_record_offset":16248,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":16572,"parent_record_offset":16540,"ancestor_record_offset":16248,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":16540,"parent_record_offset":16512,"ancestor_record_offset":16248,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":16696,"parent_record_offset":16664,"ancestor_record_offset":16248,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":16752,"parent_record_offset":16696,"ancestor_record_offset":16248,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":16664,"parent_record_offset":16636,"ancestor_record_offset":16248,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":16512,"parent_record_offset":16468,"ancestor_record_offset":16248,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":16776,"parent_record_offset":16752,"ancestor_record_offset":16248,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":17064,"parent_record_offset":17032,"ancestor_record_offset":16248,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":17348,"parent_record_offset":17064,"ancestor_record_offset":16248,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":17320,"parent_record_offset":17064,"ancestor_record_offset":16248,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":17124,"parent_record_offset":17064,"ancestor_record_offset":16248,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":18848,"parent_record_offset":18824,"ancestor_record_offset":16248,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":16248,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1774,"record_offset":17940,"parent_record_offset":17916,"ancestor_record_offset":16248,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":19012,"parent_record_offset":18988,"ancestor_record_offset":16248,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":16248,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":19036,"parent_record_offset":19012,"ancestor_record_offset":16248,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":18052,"parent_record_offset":18016,"ancestor_record_offset":16248,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":19564,"parent_record_offset":19532,"ancestor_record_offset":16248,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":20004,"parent_record_offset":19800,"ancestor_record_offset":16248,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1774,"record_offset":18192,"parent_record_offset":18164,"ancestor_record_offset":16248,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":18264,"parent_record_offset":18124,"ancestor_record_offset":16248,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":18468,"parent_record_offset":18264,"ancestor_record_offset":16248,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1774,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":16248,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1774,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":16248,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1774,"record_offset":18220,"parent_record_offset":18192,"ancestor_record_offset":16248,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1774,"record_offset":18124,"parent_record_offset":18088,"ancestor_record_offset":16248,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1774,"record_offset":18088,"parent_record_offset":18052,"ancestor_record_offset":16248,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":972,"rva":28234704,"va":5396943824}
// 编译实例 RVAs: 0x1aed3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12335266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e85ec743ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396944669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21266142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e82281bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396944841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18208328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8b829eafe","size":5,"soft_fail":false,"target_va":5378736416,"va":5396944739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4164500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e86c74c0ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5396945263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aef020
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8de190000","size":5,"soft_fail":false,"target_va":5396951072,"va":5396944445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19440054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8b6a12801","size":5,"soft_fail":false,"target_va":5416385024,"va":5396944965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12320646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e87a0044ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5396944721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19676460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e82c3d2c01","size":5,"soft_fail":false,"target_va":5416621408,"va":5396944943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19676640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8e03d2c01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396944955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e893730000","size":5,"soft_fail":false,"target_va":5396975072,"va":5396945480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12114141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8232747ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5396944216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8ce790000","size":5,"soft_fail":false,"target_va":5396975104,"va":5396943917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19676329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8a93c2c01","size":5,"soft_fail":false,"target_va":5416621376,"va":5396945042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12980529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8cfee39ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5396944316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12972030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e802103aff","size":5,"soft_fail":false,"target_va":5383972224,"va":5396944249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12979990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8eaf039ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5396944897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12981060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8bcec39ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5396943871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19674348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8ec342c01","size":5,"soft_fail":false,"target_va":5416619360,"va":5396945007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14059987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e82d7629ff","size":5,"soft_fail":false,"target_va":5382884432,"va":5396944414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14024745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8d7ff29ff","size":5,"soft_fail":false,"target_va":5382919648,"va":5396944388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":3604,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":2676,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":3772,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":3660,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":3520,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":2712,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":1164,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":1552,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1774,"record_offset":2564,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":972,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":972,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":972,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":4052,"parent_record_offset":3996,"ancestor_record_offset":972,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":4024,"parent_record_offset":3996,"ancestor_record_offset":972,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":972,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":3972,"parent_record_offset":3948,"ancestor_record_offset":972,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":972,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":3068,"parent_record_offset":3040,"ancestor_record_offset":972,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":2988,"parent_record_offset":2960,"ancestor_record_offset":972,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":972,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":3128,"parent_record_offset":3012,"ancestor_record_offset":972,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":972,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1774,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":972,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1774,"record_offset":2860,"parent_record_offset":2820,"ancestor_record_offset":972,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":3628,"parent_record_offset":3604,"ancestor_record_offset":972,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1774,"record_offset":1500,"parent_record_offset":1472,"ancestor_record_offset":972,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1774,"record_offset":3476,"parent_record_offset":2820,"ancestor_record_offset":972,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":3192,"parent_record_offset":2960,"ancestor_record_offset":972,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":972,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":972,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":2156,"parent_record_offset":2128,"ancestor_record_offset":972,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1774,"record_offset":2256,"parent_record_offset":2228,"ancestor_record_offset":972,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1774,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":972,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":972,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":972,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":2284,"parent_record_offset":2256,"ancestor_record_offset":972,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":1876,"parent_record_offset":1760,"ancestor_record_offset":972,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1774,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":972,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1774,"record_offset":1588,"parent_record_offset":1552,"ancestor_record_offset":972,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1774,"record_offset":1728,"parent_record_offset":1696,"ancestor_record_offset":972,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1774,"record_offset":1696,"parent_record_offset":1660,"ancestor_record_offset":972,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":972,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":972,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":972,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":2200,"parent_record_offset":1660,"ancestor_record_offset":972,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":1268,"parent_record_offset":1236,"ancestor_record_offset":972,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":972,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":1392,"parent_record_offset":1360,"ancestor_record_offset":972,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":1448,"parent_record_offset":1392,"ancestor_record_offset":972,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":972,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":1208,"parent_record_offset":1164,"ancestor_record_offset":972,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":972,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":972,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":2044,"parent_record_offset":1760,"ancestor_record_offset":972,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":2016,"parent_record_offset":1760,"ancestor_record_offset":972,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":1820,"parent_record_offset":1760,"ancestor_record_offset":972,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":972,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":972,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1774,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":972,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":972,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":972,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":972,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":2748,"parent_record_offset":2712,"ancestor_record_offset":972,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":4260,"parent_record_offset":4228,"ancestor_record_offset":972,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":4700,"parent_record_offset":4496,"ancestor_record_offset":972,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1774,"record_offset":2888,"parent_record_offset":2860,"ancestor_record_offset":972,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":2960,"parent_record_offset":2820,"ancestor_record_offset":972,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":3164,"parent_record_offset":2960,"ancestor_record_offset":972,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1774,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":972,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1774,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":972,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1774,"record_offset":2916,"parent_record_offset":2888,"ancestor_record_offset":972,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1774,"record_offset":2820,"parent_record_offset":2784,"ancestor_record_offset":972,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1774,"record_offset":2784,"parent_record_offset":2748,"ancestor_record_offset":972,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":25472,"rva":28240192,"va":5396949312}
// 编译实例 RVAs: 0x1aee940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12340754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e8eeb143ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396950157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21271630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e8b26bbbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396950329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18213816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e84814eafe","size":5,"soft_fail":false,"target_va":5378736416,"va":5396950227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4169988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e8fc5ec0ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5396950751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aef020
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e86e040000","size":5,"soft_fail":false,"target_va":5396951072,"va":5396949933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19434566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e8468c2801","size":5,"soft_fail":false,"target_va":5416385024,"va":5396950453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12326134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e80aeb43ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5396950209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19670972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e8bc272c01","size":5,"soft_fail":false,"target_va":5416621408,"va":5396950431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19671152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e870282c01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396950443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e8235e0000","size":5,"soft_fail":false,"target_va":5396975072,"va":5396950968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12119629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e8b31147ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5396949704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e85c640000","size":5,"soft_fail":false,"target_va":5396975104,"va":5396949407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19670841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e839272c01","size":5,"soft_fail":false,"target_va":5416621376,"va":5396950530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12986017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e85fd939ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5396949804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12977518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e892fa39ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5396949737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12985478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e87adb39ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5396950385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12986550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e84ad739ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5396949361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19668860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e87c1f2c01","size":5,"soft_fail":false,"target_va":5416619360,"va":5396950495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14065475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e8bd6029ff","size":5,"soft_fail":false,"target_va":5382884432,"va":5396949902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14030233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396949312,"raw_hex":"e867ea29ff","size":5,"soft_fail":false,"target_va":5382919648,"va":5396949876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":28136,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1774,"record_offset":25668,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":27208,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":28304,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":28192,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":28052,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":27244,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":25696,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":26084,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1774,"record_offset":27096,"parent_record_offset":25472,"ancestor_record_offset":25472,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":28608,"parent_record_offset":28584,"ancestor_record_offset":25472,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":28632,"parent_record_offset":28608,"ancestor_record_offset":25472,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":28656,"parent_record_offset":28632,"ancestor_record_offset":25472,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":28584,"parent_record_offset":28528,"ancestor_record_offset":25472,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":28556,"parent_record_offset":28528,"ancestor_record_offset":25472,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":28528,"parent_record_offset":28504,"ancestor_record_offset":25472,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":28504,"parent_record_offset":28480,"ancestor_record_offset":25472,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":27572,"parent_record_offset":27544,"ancestor_record_offset":25472,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":27600,"parent_record_offset":27572,"ancestor_record_offset":25472,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":27520,"parent_record_offset":27492,"ancestor_record_offset":25472,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":27544,"parent_record_offset":27520,"ancestor_record_offset":25472,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":27660,"parent_record_offset":27544,"ancestor_record_offset":25472,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":27772,"parent_record_offset":27748,"ancestor_record_offset":25472,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1774,"record_offset":27624,"parent_record_offset":27600,"ancestor_record_offset":25472,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1774,"record_offset":27392,"parent_record_offset":27352,"ancestor_record_offset":25472,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":28160,"parent_record_offset":28136,"ancestor_record_offset":25472,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1774,"record_offset":26032,"parent_record_offset":26004,"ancestor_record_offset":25472,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1774,"record_offset":28008,"parent_record_offset":27352,"ancestor_record_offset":25472,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":27724,"parent_record_offset":27492,"ancestor_record_offset":25472,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":26504,"parent_record_offset":26480,"ancestor_record_offset":25472,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":26480,"parent_record_offset":26456,"ancestor_record_offset":25472,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":26688,"parent_record_offset":26660,"ancestor_record_offset":25472,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1774,"record_offset":26788,"parent_record_offset":26760,"ancestor_record_offset":25472,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1774,"record_offset":26760,"parent_record_offset":26732,"ancestor_record_offset":25472,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":26432,"parent_record_offset":26408,"ancestor_record_offset":25472,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":26456,"parent_record_offset":26432,"ancestor_record_offset":25472,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":26816,"parent_record_offset":26788,"ancestor_record_offset":25472,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":26408,"parent_record_offset":26292,"ancestor_record_offset":25472,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1774,"record_offset":25824,"parent_record_offset":25800,"ancestor_record_offset":25472,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1774,"record_offset":26120,"parent_record_offset":26084,"ancestor_record_offset":25472,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1774,"record_offset":26260,"parent_record_offset":26228,"ancestor_record_offset":25472,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1774,"record_offset":26228,"parent_record_offset":26192,"ancestor_record_offset":25472,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":26168,"parent_record_offset":26144,"ancestor_record_offset":25472,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":26192,"parent_record_offset":26168,"ancestor_record_offset":25472,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":26144,"parent_record_offset":26120,"ancestor_record_offset":25472,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":26732,"parent_record_offset":26192,"ancestor_record_offset":25472,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":25800,"parent_record_offset":25768,"ancestor_record_offset":25472,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":25768,"parent_record_offset":25740,"ancestor_record_offset":25472,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":25924,"parent_record_offset":25892,"ancestor_record_offset":25472,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":25980,"parent_record_offset":25924,"ancestor_record_offset":25472,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":25892,"parent_record_offset":25864,"ancestor_record_offset":25472,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":25740,"parent_record_offset":25696,"ancestor_record_offset":25472,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":26004,"parent_record_offset":25980,"ancestor_record_offset":25472,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":26292,"parent_record_offset":26260,"ancestor_record_offset":25472,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":26576,"parent_record_offset":26292,"ancestor_record_offset":25472,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":26548,"parent_record_offset":26292,"ancestor_record_offset":25472,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":26352,"parent_record_offset":26292,"ancestor_record_offset":25472,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":28076,"parent_record_offset":28052,"ancestor_record_offset":25472,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":28100,"parent_record_offset":28076,"ancestor_record_offset":25472,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1774,"record_offset":27168,"parent_record_offset":27144,"ancestor_record_offset":25472,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":28240,"parent_record_offset":28216,"ancestor_record_offset":25472,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":28216,"parent_record_offset":28192,"ancestor_record_offset":25472,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":28264,"parent_record_offset":28240,"ancestor_record_offset":25472,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":27280,"parent_record_offset":27244,"ancestor_record_offset":25472,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":28792,"parent_record_offset":28760,"ancestor_record_offset":25472,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":29232,"parent_record_offset":29028,"ancestor_record_offset":25472,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1774,"record_offset":27420,"parent_record_offset":27392,"ancestor_record_offset":25472,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":27492,"parent_record_offset":27352,"ancestor_record_offset":25472,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":27696,"parent_record_offset":27492,"ancestor_record_offset":25472,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1774,"record_offset":27120,"parent_record_offset":27096,"ancestor_record_offset":25472,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1774,"record_offset":27144,"parent_record_offset":27120,"ancestor_record_offset":25472,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1774,"record_offset":27448,"parent_record_offset":27420,"ancestor_record_offset":25472,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1774,"record_offset":27352,"parent_record_offset":27316,"ancestor_record_offset":25472,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1774,"record_offset":27316,"parent_record_offset":27280,"ancestor_record_offset":25472,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":20880,"rva":28234704,"va":5396943824}
// 编译实例 RVAs: 0x1aed3d0, 0x28baee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12335266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e85ec743ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396944669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-21266142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e82281bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396944841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18208328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8b829eafe","size":5,"soft_fail":false,"target_va":5378736416,"va":5396944739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c60b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25121208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411417824,"raw_hex":"e848ae80fe","size":5,"soft_fail":false,"target_va":5386297520,"va":5411418723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4164500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e86c74c0ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5396945263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aef020
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8de190000","size":5,"soft_fail":false,"target_va":5396951072,"va":5396944445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19440054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8b6a12801","size":5,"soft_fail":false,"target_va":5416385024,"va":5396944965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12320646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e87a0044ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5396944721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19676460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e82c3d2c01","size":5,"soft_fail":false,"target_va":5416621408,"va":5396944943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19676640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8e03d2c01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396944955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e893730000","size":5,"soft_fail":false,"target_va":5396975072,"va":5396945480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12114141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8232747ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5396944216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8ce790000","size":5,"soft_fail":false,"target_va":5396975104,"va":5396943917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t19676329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8a93c2c01","size":5,"soft_fail":false,"target_va":5416621376,"va":5396945042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<str$> >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411417824,"raw_hex":"e86e040000","size":5,"soft_fail":false,"target_va":5411419568,"va":5411418429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12980529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8cfee39ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5396944316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12972030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e802103aff","size":5,"soft_fail":false,"target_va":5383972224,"va":5396944249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12979990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8eaf039ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5396944897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12981060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8bcec39ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5396943871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5526550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411417824,"raw_hex":"e816545400","size":5,"soft_fail":false,"target_va":5416945504,"va":5411418949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25013636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411417824,"raw_hex":"e87c5282fe","size":5,"soft_fail":false,"target_va":5386405616,"va":5411419247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19674348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8ec342c01","size":5,"soft_fail":false,"target_va":5416619360,"va":5396945007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14059987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e82d7629ff","size":5,"soft_fail":false,"target_va":5382884432,"va":5396944414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14024745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396943824,"raw_hex":"e8d7ff29ff","size":5,"soft_fail":false,"target_va":5382919648,"va":5396944388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":23500,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":22572,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":23668,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":23556,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":23416,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":22608,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":21060,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":21448,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1774,"record_offset":22460,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":23972,"parent_record_offset":23948,"ancestor_record_offset":20880,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":23996,"parent_record_offset":23972,"ancestor_record_offset":20880,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":24020,"parent_record_offset":23996,"ancestor_record_offset":20880,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":23948,"parent_record_offset":23892,"ancestor_record_offset":20880,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":23920,"parent_record_offset":23892,"ancestor_record_offset":20880,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":23892,"parent_record_offset":23868,"ancestor_record_offset":20880,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":23868,"parent_record_offset":23844,"ancestor_record_offset":20880,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":22936,"parent_record_offset":22908,"ancestor_record_offset":20880,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":22964,"parent_record_offset":22936,"ancestor_record_offset":20880,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":22884,"parent_record_offset":22856,"ancestor_record_offset":20880,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":22908,"parent_record_offset":22884,"ancestor_record_offset":20880,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":23024,"parent_record_offset":22908,"ancestor_record_offset":20880,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1774,"record_offset":23136,"parent_record_offset":23112,"ancestor_record_offset":20880,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1774,"record_offset":22988,"parent_record_offset":22964,"ancestor_record_offset":20880,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1774,"record_offset":22756,"parent_record_offset":22716,"ancestor_record_offset":20880,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":23524,"parent_record_offset":23500,"ancestor_record_offset":20880,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1774,"record_offset":21396,"parent_record_offset":21368,"ancestor_record_offset":20880,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1774,"record_offset":23372,"parent_record_offset":22716,"ancestor_record_offset":20880,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":23088,"parent_record_offset":22856,"ancestor_record_offset":20880,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":20880,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":20880,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1774,"record_offset":22052,"parent_record_offset":22024,"ancestor_record_offset":20880,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1774,"record_offset":22152,"parent_record_offset":22124,"ancestor_record_offset":20880,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1774,"record_offset":22124,"parent_record_offset":22096,"ancestor_record_offset":20880,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":21796,"parent_record_offset":21772,"ancestor_record_offset":20880,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":21820,"parent_record_offset":21796,"ancestor_record_offset":20880,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":22180,"parent_record_offset":22152,"ancestor_record_offset":20880,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1774,"record_offset":21772,"parent_record_offset":21656,"ancestor_record_offset":20880,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1774,"record_offset":21188,"parent_record_offset":21164,"ancestor_record_offset":20880,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1774,"record_offset":21484,"parent_record_offset":21448,"ancestor_record_offset":20880,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1774,"record_offset":21624,"parent_record_offset":21592,"ancestor_record_offset":20880,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1774,"record_offset":21592,"parent_record_offset":21556,"ancestor_record_offset":20880,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":20880,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":21556,"parent_record_offset":21532,"ancestor_record_offset":20880,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":20880,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1774,"record_offset":22096,"parent_record_offset":21556,"ancestor_record_offset":20880,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":21164,"parent_record_offset":21132,"ancestor_record_offset":20880,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":21132,"parent_record_offset":21104,"ancestor_record_offset":20880,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":21288,"parent_record_offset":21256,"ancestor_record_offset":20880,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":21344,"parent_record_offset":21288,"ancestor_record_offset":20880,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":21256,"parent_record_offset":21228,"ancestor_record_offset":20880,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":21104,"parent_record_offset":21060,"ancestor_record_offset":20880,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1774,"record_offset":21368,"parent_record_offset":21344,"ancestor_record_offset":20880,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":21656,"parent_record_offset":21624,"ancestor_record_offset":20880,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":21940,"parent_record_offset":21656,"ancestor_record_offset":20880,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":21912,"parent_record_offset":21656,"ancestor_record_offset":20880,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1774,"record_offset":21716,"parent_record_offset":21656,"ancestor_record_offset":20880,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":20880,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":23464,"parent_record_offset":23440,"ancestor_record_offset":20880,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1774,"record_offset":22532,"parent_record_offset":22508,"ancestor_record_offset":20880,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":23604,"parent_record_offset":23580,"ancestor_record_offset":20880,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":23580,"parent_record_offset":23556,"ancestor_record_offset":20880,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":23628,"parent_record_offset":23604,"ancestor_record_offset":20880,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":22644,"parent_record_offset":22608,"ancestor_record_offset":20880,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":24156,"parent_record_offset":24124,"ancestor_record_offset":20880,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":24596,"parent_record_offset":24392,"ancestor_record_offset":20880,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1774,"record_offset":22784,"parent_record_offset":22756,"ancestor_record_offset":20880,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":22856,"parent_record_offset":22716,"ancestor_record_offset":20880,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1774,"record_offset":23060,"parent_record_offset":22856,"ancestor_record_offset":20880,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1774,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":20880,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1774,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":20880,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1774,"record_offset":22812,"parent_record_offset":22784,"ancestor_record_offset":20880,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1774,"record_offset":22716,"parent_record_offset":22680,"ancestor_record_offset":20880,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1774,"record_offset":22680,"parent_record_offset":22644,"ancestor_record_offset":20880,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1590,"record_offset":272,"rva":21885008,"va":5390594128}
// 编译实例 RVAs: 0x14df050, 0x1aedab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5985562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8e6aaa4ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5390594965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-14916446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8a2641cff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390595145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18210114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396945584,"raw_hex":"e8be22eafe","size":5,"soft_fail":false,"target_va":5378736416,"va":5396946525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414aa360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-217224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e878affcff","size":5,"soft_fail":false,"target_va":5390377824,"va":5390595043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414df7e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e826050000","size":5,"soft_fail":false,"target_va":5390596064,"va":5390594741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4166468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396945584,"raw_hex":"e8bc6cc0ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5396947231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aef020
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396945584,"raw_hex":"e8ec120000","size":5,"soft_fail":false,"target_va":5396951072,"va":5396946223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19438236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396945584,"raw_hex":"e89c9a2801","size":5,"soft_fail":false,"target_va":5416385024,"va":5396946783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5970950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8fae3a4ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5390595025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26026062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e84e208d01","size":5,"soft_fail":false,"target_va":5416621408,"va":5390595341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26026310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e846218d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390595285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e873020000","size":5,"soft_fail":false,"target_va":5390596592,"va":5390595960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5764451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e89d0aa8ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5390594526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e853090000","size":5,"soft_fail":false,"target_va":5390596624,"va":5390594232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26025939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8d31f8d01","size":5,"soft_fail":false,"target_va":5416621376,"va":5390595432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6630835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e84dd29aff","size":5,"soft_fail":false,"target_va":5383963792,"va":5390594622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6622331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e885f39aff","size":5,"soft_fail":false,"target_va":5383972224,"va":5390594550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6630299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e865d49aff","size":5,"soft_fail":false,"target_va":5383964912,"va":5390595206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6631380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e82cd09aff","size":5,"soft_fail":false,"target_va":5383962816,"va":5390594191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26122390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e896988e01","size":5,"soft_fail":false,"target_va":5416717696,"va":5390595301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-228500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e86c83fcff","size":5,"soft_fail":false,"target_va":5390367248,"va":5390595743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26024032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e860188d01","size":5,"soft_fail":false,"target_va":5416619360,"va":5390595323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7710279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e8b9598aff","size":5,"soft_fail":false,"target_va":5382884432,"va":5390594706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7675037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390594128,"raw_hex":"e863e38aff","size":5,"soft_fail":false,"target_va":5382919648,"va":5390594680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1590,"record_offset":2952,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1590,"record_offset":1972,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1590,"record_offset":3120,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1590,"record_offset":3008,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":2868,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":2008,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":460,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":848,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1590,"record_offset":1860,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":3400,"parent_record_offset":3344,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":3372,"parent_record_offset":3344,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":2388,"parent_record_offset":2360,"ancestor_record_offset":272,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":2416,"parent_record_offset":2388,"ancestor_record_offset":272,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":2328,"parent_record_offset":2296,"ancestor_record_offset":272,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":2360,"parent_record_offset":2328,"ancestor_record_offset":272,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":2476,"parent_record_offset":2360,"ancestor_record_offset":272,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1590,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":272,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1590,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":272,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1590,"record_offset":2188,"parent_record_offset":2136,"ancestor_record_offset":272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1590,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":272,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1590,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1590,"record_offset":2824,"parent_record_offset":2136,"ancestor_record_offset":272,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1590,"record_offset":2540,"parent_record_offset":2296,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1590,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":272,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1590,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":272,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1590,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1590,"record_offset":1552,"parent_record_offset":1524,"ancestor_record_offset":272,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1590,"record_offset":1524,"parent_record_offset":1496,"ancestor_record_offset":272,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1590,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":272,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1590,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":272,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1590,"record_offset":1580,"parent_record_offset":1552,"ancestor_record_offset":272,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1590,"record_offset":1172,"parent_record_offset":1056,"ancestor_record_offset":272,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1590,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1590,"record_offset":884,"parent_record_offset":848,"ancestor_record_offset":272,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1590,"record_offset":1024,"parent_record_offset":992,"ancestor_record_offset":272,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1590,"record_offset":992,"parent_record_offset":956,"ancestor_record_offset":272,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1590,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":272,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1590,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":272,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1590,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":272,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1590,"record_offset":1496,"parent_record_offset":956,"ancestor_record_offset":272,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":564,"parent_record_offset":532,"ancestor_record_offset":272,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":532,"parent_record_offset":504,"ancestor_record_offset":272,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":688,"parent_record_offset":656,"ancestor_record_offset":272,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":744,"parent_record_offset":688,"ancestor_record_offset":272,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":656,"parent_record_offset":628,"ancestor_record_offset":272,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":504,"parent_record_offset":460,"ancestor_record_offset":272,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1590,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":272,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1590,"record_offset":1056,"parent_record_offset":1024,"ancestor_record_offset":272,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1590,"record_offset":1340,"parent_record_offset":1056,"ancestor_record_offset":272,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1590,"record_offset":1312,"parent_record_offset":1056,"ancestor_record_offset":272,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1590,"record_offset":1116,"parent_record_offset":1056,"ancestor_record_offset":272,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1590,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":272,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1590,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":272,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1590,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":272,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":272,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1590,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":272,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":272,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":2048,"parent_record_offset":2008,"ancestor_record_offset":272,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":3608,"parent_record_offset":3576,"ancestor_record_offset":272,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":4080,"parent_record_offset":3864,"ancestor_record_offset":272,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1590,"record_offset":2220,"parent_record_offset":2188,"ancestor_record_offset":272,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":2296,"parent_record_offset":2136,"ancestor_record_offset":272,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1590,"record_offset":2512,"parent_record_offset":2296,"ancestor_record_offset":272,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1590,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":272,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1590,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":272,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1590,"record_offset":2252,"parent_record_offset":2220,"ancestor_record_offset":272,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1590,"record_offset":2136,"parent_record_offset":2088,"ancestor_record_offset":272,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1590,"record_offset":2088,"parent_record_offset":2048,"ancestor_record_offset":272,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0<alloc::string::String>
// 记录形式: compiled_procedure; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1590,"record_offset":10944,"rva":21886944,"va":5390596064}
// 编译实例 RVAs: 0x14df7e0, 0x1aef020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5986960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e870a5a4ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5390596363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-14917762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e87e5f1cff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390596461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9698967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e869016cff","size":5,"soft_fail":false,"target_va":5387252208,"va":5396951170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14250bd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16958761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e829c50201","size":5,"soft_fail":false,"target_va":5407554928,"va":5390596162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26025101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e88d1c8d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390596494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5766305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e85f03a8ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5390596380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::load_json_str_as_mesh
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11557897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e8f7a34fff","size":5,"soft_fail":false,"target_va":5379038544,"va":5390596436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5844881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e86fd0a6ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5390596140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1590,"record_offset":11316,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1590,"record_offset":11596,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1590,"record_offset":11632,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1590,"record_offset":11172,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1590,"record_offset":11204,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1590,"record_offset":11372,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":11232,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1590,"record_offset":11144,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1590,"record_offset":11484,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":11808,"parent_record_offset":11784,"ancestor_record_offset":10944,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":11832,"parent_record_offset":11808,"ancestor_record_offset":10944,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":10944,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":11784,"parent_record_offset":11728,"ancestor_record_offset":10944,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":11756,"parent_record_offset":11728,"ancestor_record_offset":10944,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":11728,"parent_record_offset":11704,"ancestor_record_offset":10944,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":10944,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1590,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":10944,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1590,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":10944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1590,"record_offset":11256,"parent_record_offset":11232,"ancestor_record_offset":10944,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1590,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":10944,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1590,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":10944,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":10944,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1590,"record_offset":11396,"parent_record_offset":11372,"ancestor_record_offset":10944,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":10944,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1590,"record_offset":11508,"parent_record_offset":11484,"ancestor_record_offset":10944,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1590,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":10944,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":31368,"rva":28241952,"va":5396951072}
// 编译实例 RVAs: 0x1aef020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12341968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e830ad43ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396951371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21272770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e83e67bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396951469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9698967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e869016cff","size":5,"soft_fail":false,"target_va":5387252208,"va":5396951170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19670093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e84d242c01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396951502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12121313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e81f0b47ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5396951388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::load_json_str_as_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17912905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e8b7abeefe","size":5,"soft_fail":false,"target_va":5379038544,"va":5396951444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12199889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e82fd845ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5396951148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":31736,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":32016,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":32052,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":31592,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":31624,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":31792,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":31652,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1774,"record_offset":31564,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1774,"record_offset":31904,"parent_record_offset":31368,"ancestor_record_offset":31368,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":32228,"parent_record_offset":32204,"ancestor_record_offset":31368,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":32252,"parent_record_offset":32228,"ancestor_record_offset":31368,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":32276,"parent_record_offset":32252,"ancestor_record_offset":31368,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":32204,"parent_record_offset":32148,"ancestor_record_offset":31368,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":32176,"parent_record_offset":32148,"ancestor_record_offset":31368,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":32148,"parent_record_offset":32124,"ancestor_record_offset":31368,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":32124,"parent_record_offset":32100,"ancestor_record_offset":31368,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":31760,"parent_record_offset":31736,"ancestor_record_offset":31368,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":32076,"parent_record_offset":32052,"ancestor_record_offset":31368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":31676,"parent_record_offset":31652,"ancestor_record_offset":31368,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":31700,"parent_record_offset":31676,"ancestor_record_offset":31368,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1774,"record_offset":31976,"parent_record_offset":31952,"ancestor_record_offset":31368,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":31840,"parent_record_offset":31816,"ancestor_record_offset":31368,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":31816,"parent_record_offset":31792,"ancestor_record_offset":31368,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":31864,"parent_record_offset":31840,"ancestor_record_offset":31368,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1774,"record_offset":31928,"parent_record_offset":31904,"ancestor_record_offset":31368,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1774,"record_offset":31952,"parent_record_offset":31928,"ancestor_record_offset":31368,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<std::path::Path> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":30108,"rva":28241952,"va":5396951072}
// 编译实例 RVAs: 0x1aef020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12341968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e830ad43ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396951371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21272770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e83e67bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396951469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9698967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e869016cff","size":5,"soft_fail":false,"target_va":5387252208,"va":5396951170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19670093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e84d242c01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396951502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12121313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e81f0b47ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5396951388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::load_json_str_as_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17912905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e8b7abeefe","size":5,"soft_fail":false,"target_va":5379038544,"va":5396951444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12199889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e82fd845ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5396951148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":30480,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":30760,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":30796,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":30336,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":30368,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":30536,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":30396,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1774,"record_offset":30308,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1774,"record_offset":30648,"parent_record_offset":30108,"ancestor_record_offset":30108,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":30972,"parent_record_offset":30948,"ancestor_record_offset":30108,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":30996,"parent_record_offset":30972,"ancestor_record_offset":30108,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":31020,"parent_record_offset":30996,"ancestor_record_offset":30108,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":30948,"parent_record_offset":30892,"ancestor_record_offset":30108,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":30920,"parent_record_offset":30892,"ancestor_record_offset":30108,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":30892,"parent_record_offset":30868,"ancestor_record_offset":30108,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":30868,"parent_record_offset":30844,"ancestor_record_offset":30108,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":30504,"parent_record_offset":30480,"ancestor_record_offset":30108,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":30820,"parent_record_offset":30796,"ancestor_record_offset":30108,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":30420,"parent_record_offset":30396,"ancestor_record_offset":30108,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":30444,"parent_record_offset":30420,"ancestor_record_offset":30108,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1774,"record_offset":30720,"parent_record_offset":30696,"ancestor_record_offset":30108,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":30584,"parent_record_offset":30560,"ancestor_record_offset":30108,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":30560,"parent_record_offset":30536,"ancestor_record_offset":30108,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":30608,"parent_record_offset":30584,"ancestor_record_offset":30108,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1774,"record_offset":30672,"parent_record_offset":30648,"ancestor_record_offset":30108,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1774,"record_offset":30696,"parent_record_offset":30672,"ancestor_record_offset":30108,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<std::path::PathBuf> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":32624,"rva":28241952,"va":5396951072}
// 编译实例 RVAs: 0x1aef020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12341968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e830ad43ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396951371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21272770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e83e67bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396951469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9698967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e869016cff","size":5,"soft_fail":false,"target_va":5387252208,"va":5396951170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19670093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e84d242c01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396951502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12121313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e81f0b47ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5396951388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::load_json_str_as_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17912905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e8b7abeefe","size":5,"soft_fail":false,"target_va":5379038544,"va":5396951444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12199889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e82fd845ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5396951148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":33000,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":33280,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":33316,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":32856,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":32888,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":33056,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":32916,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1774,"record_offset":32828,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1774,"record_offset":33168,"parent_record_offset":32624,"ancestor_record_offset":32624,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":33492,"parent_record_offset":33468,"ancestor_record_offset":32624,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":33516,"parent_record_offset":33492,"ancestor_record_offset":32624,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":33540,"parent_record_offset":33516,"ancestor_record_offset":32624,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":33468,"parent_record_offset":33412,"ancestor_record_offset":32624,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":33440,"parent_record_offset":33412,"ancestor_record_offset":32624,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":33412,"parent_record_offset":33388,"ancestor_record_offset":32624,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":33388,"parent_record_offset":33364,"ancestor_record_offset":32624,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":33024,"parent_record_offset":33000,"ancestor_record_offset":32624,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":33340,"parent_record_offset":33316,"ancestor_record_offset":32624,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":32940,"parent_record_offset":32916,"ancestor_record_offset":32624,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":32964,"parent_record_offset":32940,"ancestor_record_offset":32624,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1774,"record_offset":33240,"parent_record_offset":33216,"ancestor_record_offset":32624,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":33104,"parent_record_offset":33080,"ancestor_record_offset":32624,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":33080,"parent_record_offset":33056,"ancestor_record_offset":32624,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":33128,"parent_record_offset":33104,"ancestor_record_offset":32624,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1774,"record_offset":33192,"parent_record_offset":33168,"ancestor_record_offset":32624,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1774,"record_offset":33216,"parent_record_offset":33192,"ancestor_record_offset":32624,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":33888,"rva":28241952,"va":5396951072}
// 编译实例 RVAs: 0x1aef020, 0x28bb5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12341968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e830ad43ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396951371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-21272770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e83e67bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396951469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9698967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e869016cff","size":5,"soft_fail":false,"target_va":5387252208,"va":5396951170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b20cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2512569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411419568,"raw_hex":"e8b9562600","size":5,"soft_fail":false,"target_va":5413932240,"va":5411419666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19670093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e84d242c01","size":5,"soft_fail":false,"target_va":5416621600,"va":5396951502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12121313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e81f0b47ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5396951388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::load_json_str_as_mesh
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17912905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e8b7abeefe","size":5,"soft_fail":false,"target_va":5379038544,"va":5396951444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12199889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e82fd845ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5396951148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":34252,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":34532,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":34568,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":34108,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":34140,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":34308,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":34168,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1774,"record_offset":34080,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1774,"record_offset":34420,"parent_record_offset":33888,"ancestor_record_offset":33888,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":34744,"parent_record_offset":34720,"ancestor_record_offset":33888,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":34768,"parent_record_offset":34744,"ancestor_record_offset":33888,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":34792,"parent_record_offset":34768,"ancestor_record_offset":33888,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":34720,"parent_record_offset":34664,"ancestor_record_offset":33888,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":34692,"parent_record_offset":34664,"ancestor_record_offset":33888,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":34664,"parent_record_offset":34640,"ancestor_record_offset":33888,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":34640,"parent_record_offset":34616,"ancestor_record_offset":33888,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":34276,"parent_record_offset":34252,"ancestor_record_offset":33888,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":34592,"parent_record_offset":34568,"ancestor_record_offset":33888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":34192,"parent_record_offset":34168,"ancestor_record_offset":33888,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1774,"record_offset":34216,"parent_record_offset":34192,"ancestor_record_offset":33888,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1774,"record_offset":34492,"parent_record_offset":34468,"ancestor_record_offset":33888,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":34356,"parent_record_offset":34332,"ancestor_record_offset":33888,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1774,"record_offset":34332,"parent_record_offset":34308,"ancestor_record_offset":33888,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1774,"record_offset":34380,"parent_record_offset":34356,"ancestor_record_offset":33888,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1774,"record_offset":34444,"parent_record_offset":34420,"ancestor_record_offset":33888,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1774,"record_offset":34468,"parent_record_offset":34444,"ancestor_record_offset":33888,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1590,"record_offset":12204,"rva":21886944,"va":5390596064}
// 编译实例 RVAs: 0x14df7e0, 0x1aef020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5986960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e870a5a4ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5390596363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-14917762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e87e5f1cff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390596461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9698967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951072,"raw_hex":"e869016cff","size":5,"soft_fail":false,"target_va":5387252208,"va":5396951170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14250bd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16958761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e829c50201","size":5,"soft_fail":false,"target_va":5407554928,"va":5390596162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26025101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e88d1c8d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390596494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5766305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e85f03a8ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5390596380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::load_json::load_json_str_as_mesh
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11557897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e8f7a34fff","size":5,"soft_fail":false,"target_va":5379038544,"va":5390596436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5844881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390596064,"raw_hex":"e86fd0a6ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5390596140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1590,"record_offset":12572,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1590,"record_offset":12852,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1590,"record_offset":12888,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1590,"record_offset":12428,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1590,"record_offset":12460,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1590,"record_offset":12628,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":12488,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1590,"record_offset":12400,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1590,"record_offset":12740,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":12204,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":12204,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":12204,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1590,"record_offset":13040,"parent_record_offset":12984,"ancestor_record_offset":12204,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":13012,"parent_record_offset":12984,"ancestor_record_offset":12204,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":12984,"parent_record_offset":12960,"ancestor_record_offset":12204,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1590,"record_offset":12960,"parent_record_offset":12936,"ancestor_record_offset":12204,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1590,"record_offset":12596,"parent_record_offset":12572,"ancestor_record_offset":12204,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1590,"record_offset":12912,"parent_record_offset":12888,"ancestor_record_offset":12204,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1590,"record_offset":12512,"parent_record_offset":12488,"ancestor_record_offset":12204,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1590,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":12204,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1590,"record_offset":12812,"parent_record_offset":12788,"ancestor_record_offset":12204,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":12676,"parent_record_offset":12652,"ancestor_record_offset":12204,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1590,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":12204,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1590,"record_offset":12700,"parent_record_offset":12676,"ancestor_record_offset":12204,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1590,"record_offset":12764,"parent_record_offset":12740,"ancestor_record_offset":12204,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1590,"record_offset":12788,"parent_record_offset":12764,"ancestor_record_offset":12204,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_as_mesh::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112C5
// 首条位置: {"module":1253,"inlinee":70341}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":29788,"parent_record_offset":29768,"ancestor_record_offset":29768,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":29836,"parent_record_offset":29812,"ancestor_record_offset":29768,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":29812,"parent_record_offset":29788,"ancestor_record_offset":29768,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112D6
// 首条位置: {"module":1253,"inlinee":70358}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":37296,"parent_record_offset":37276,"ancestor_record_offset":37276,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":37344,"parent_record_offset":37320,"ancestor_record_offset":37276,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":37320,"parent_record_offset":37296,"ancestor_record_offset":37276,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x780C
// 首条位置: {"module":521,"inlinee":30732}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":30852,"parent_record_offset":30828,"ancestor_record_offset":30828,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":30900,"parent_record_offset":30876,"ancestor_record_offset":30828,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":30876,"parent_record_offset":30852,"ancestor_record_offset":30828,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143A5
// 首条位置: {"module":1682,"inlinee":82853}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$1::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1682,"record_offset":160200,"parent_record_offset":160140,"ancestor_record_offset":160140,"inlinee":30730,"source_line":103,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_u64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1682,"record_offset":160172,"parent_record_offset":160140,"ancestor_record_offset":160140,"inlinee":82854,"source_line":650,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":160560,"parent_record_offset":160536,"ancestor_record_offset":160140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":160584,"parent_record_offset":160560,"ancestor_record_offset":160140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":160608,"parent_record_offset":160584,"ancestor_record_offset":160140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":160536,"parent_record_offset":160480,"ancestor_record_offset":160140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":160272,"parent_record_offset":160252,"ancestor_record_offset":160140,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":160320,"parent_record_offset":160296,"ancestor_record_offset":160140,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":160508,"parent_record_offset":160480,"ancestor_record_offset":160140,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":160480,"parent_record_offset":160456,"ancestor_record_offset":160140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":160456,"parent_record_offset":160432,"ancestor_record_offset":160140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1682,"record_offset":160356,"parent_record_offset":160200,"ancestor_record_offset":160140,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":160296,"parent_record_offset":160272,"ancestor_record_offset":160140,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":160384,"parent_record_offset":160200,"ancestor_record_offset":160140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_str_as_mesh::closure$2::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1682,"record_offset":160252,"parent_record_offset":160200,"ancestor_record_offset":160140,"inlinee":82856,"source_line":65,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$2::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143A8
// 首条位置: {"module":1682,"inlinee":82856}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":160272,"parent_record_offset":160252,"ancestor_record_offset":160252,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":160320,"parent_record_offset":160296,"ancestor_record_offset":160252,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":160296,"parent_record_offset":160272,"ancestor_record_offset":160252,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x780F
// 首条位置: {"module":521,"inlinee":30735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":34232,"parent_record_offset":34208,"ancestor_record_offset":34208,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":34280,"parent_record_offset":34256,"ancestor_record_offset":34208,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":34256,"parent_record_offset":34232,"ancestor_record_offset":34208,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143B6
// 首条位置: {"module":1682,"inlinee":82870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$1::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1682,"record_offset":188572,"parent_record_offset":188504,"ancestor_record_offset":188504,"inlinee":30730,"source_line":103,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1682,"record_offset":188540,"parent_record_offset":188504,"ancestor_record_offset":188504,"inlinee":31538,"source_line":492,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":188932,"parent_record_offset":188908,"ancestor_record_offset":188504,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":188956,"parent_record_offset":188932,"ancestor_record_offset":188504,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":188980,"parent_record_offset":188956,"ancestor_record_offset":188504,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":188908,"parent_record_offset":188852,"ancestor_record_offset":188504,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":188644,"parent_record_offset":188624,"ancestor_record_offset":188504,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":188692,"parent_record_offset":188668,"ancestor_record_offset":188504,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":188880,"parent_record_offset":188852,"ancestor_record_offset":188504,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":188852,"parent_record_offset":188828,"ancestor_record_offset":188504,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":188828,"parent_record_offset":188804,"ancestor_record_offset":188504,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1682,"record_offset":188728,"parent_record_offset":188572,"ancestor_record_offset":188504,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":188668,"parent_record_offset":188644,"ancestor_record_offset":188504,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":188756,"parent_record_offset":188572,"ancestor_record_offset":188504,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::load_json_str_as_mesh::closure$5::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1682,"record_offset":188624,"parent_record_offset":188572,"ancestor_record_offset":188504,"inlinee":82872,"source_line":83,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$5::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143B8
// 首条位置: {"module":1682,"inlinee":82872}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":188644,"parent_record_offset":188624,"ancestor_record_offset":188624,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":188692,"parent_record_offset":188668,"ancestor_record_offset":188624,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":188668,"parent_record_offset":188644,"ancestor_record_offset":188624,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7814
// 首条位置: {"module":521,"inlinee":30740}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":39808,"parent_record_offset":39784,"ancestor_record_offset":39784,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":39856,"parent_record_offset":39832,"ancestor_record_offset":39784,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":39832,"parent_record_offset":39808,"ancestor_record_offset":39784,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::load_json_str_as_mesh::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7812
// 首条位置: {"module":521,"inlinee":30738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":40508,"parent_record_offset":40452,"ancestor_record_offset":40452,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_bool
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":521,"record_offset":40480,"parent_record_offset":40452,"ancestor_record_offset":40452,"inlinee":30739,"source_line":708,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7811
// 首条位置: {"module":521,"inlinee":30737}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":36180,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":521,"record_offset":35936,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":521,"record_offset":36888,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":521,"record_offset":37112,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":521,"record_offset":36800,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":38888,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":36856,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":521,"record_offset":36716,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":36264,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":521,"record_offset":35908,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":30729,"source_line":425,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/index.rs
//     evidence={"module":521,"record_offset":35740,"parent_record_offset":35556,"ancestor_record_offset":35556,"inlinee":30726,"source_line":212,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":37644,"parent_record_offset":37620,"ancestor_record_offset":35556,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":37668,"parent_record_offset":37644,"ancestor_record_offset":35556,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":37692,"parent_record_offset":37668,"ancestor_record_offset":35556,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":37620,"parent_record_offset":37528,"ancestor_record_offset":35556,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":36228,"parent_record_offset":36204,"ancestor_record_offset":35556,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":37556,"parent_record_offset":37528,"ancestor_record_offset":35556,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":37528,"parent_record_offset":37504,"ancestor_record_offset":35556,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":37504,"parent_record_offset":37480,"ancestor_record_offset":35556,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":37392,"parent_record_offset":37368,"ancestor_record_offset":35556,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":521,"record_offset":35980,"parent_record_offset":35936,"ancestor_record_offset":35556,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":521,"record_offset":36288,"parent_record_offset":36264,"ancestor_record_offset":35556,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::try_process
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\mod.rs
//     evidence={"module":521,"record_offset":37296,"parent_record_offset":37260,"ancestor_record_offset":35556,"inlinee":11759,"source_line":152,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":521,"record_offset":37416,"parent_record_offset":37392,"ancestor_record_offset":35556,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":521,"record_offset":37368,"parent_record_offset":37344,"ancestor_record_offset":35556,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":521,"record_offset":37588,"parent_record_offset":37556,"ancestor_record_offset":35556,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":36204,"parent_record_offset":36180,"ancestor_record_offset":35556,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":35792,"parent_record_offset":35740,"ancestor_record_offset":35556,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":37456,"parent_record_offset":37296,"ancestor_record_offset":35556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":521,"record_offset":36764,"parent_record_offset":36740,"ancestor_record_offset":35556,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":36824,"parent_record_offset":36800,"ancestor_record_offset":35556,"inlinee":23302,"source_line":2155,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::from_iter::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":37344,"parent_record_offset":37296,"ancestor_record_offset":35556,"inlinee":23304,"source_line":2156,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":521,"record_offset":36312,"parent_record_offset":36288,"ancestor_record_offset":35556,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":521,"record_offset":36340,"parent_record_offset":36312,"ancestor_record_offset":35556,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":521,"record_offset":36740,"parent_record_offset":36716,"ancestor_record_offset":35556,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/index.rs
//     evidence={"module":521,"record_offset":35764,"parent_record_offset":35740,"ancestor_record_offset":35556,"inlinee":30728,"source_line":123,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8946
// 首条位置: {"module":560,"inlinee":35142}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1643,"record_offset":387936,"parent_record_offset":387840,"ancestor_record_offset":387840,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_i64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1643,"record_offset":387872,"parent_record_offset":387840,"ancestor_record_offset":387840,"inlinee":35143,"source_line":631,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1643,"record_offset":387980,"parent_record_offset":387936,"ancestor_record_offset":387840,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_i64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":1643,"record_offset":387896,"parent_record_offset":387872,"ancestor_record_offset":387840,"inlinee":35144,"source_line":132,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_attribute_from_json::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8943
// 首条位置: {"module":560,"inlinee":35139}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1643,"record_offset":461028,"parent_record_offset":460900,"ancestor_record_offset":460900,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":461000,"parent_record_offset":460900,"ancestor_record_offset":460900,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1643,"record_offset":460936,"parent_record_offset":460900,"ancestor_record_offset":460900,"inlinee":35140,"source_line":669,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1643,"record_offset":461064,"parent_record_offset":461028,"ancestor_record_offset":460900,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":1643,"record_offset":460960,"parent_record_offset":460936,"ancestor_record_offset":460900,"inlinee":35141,"source_line":162,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_float_element<2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":38528,"rva":28243088,"va":5396952208}
// 编译实例 RVAs: 0x1aef490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12293093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952208,"raw_hex":"e81b6c44ff","size":5,"soft_fail":false,"target_va":5384659264,"va":5396952352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21274017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952208,"raw_hex":"e85f62bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396952716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19808954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952208,"raw_hex":"e8ba422e01","size":5,"soft_fail":false,"target_va":5416761616,"va":5396952657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19809837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952208,"raw_hex":"e82d462e01","size":5,"soft_fail":false,"target_va":5416762432,"va":5396952590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19813581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952208,"raw_hex":"e8cd542e01","size":5,"soft_fail":false,"target_va":5416765984,"va":5396952398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952208,"raw_hex":"e8f2dcffff","size":5,"soft_fail":false,"target_va":5396943680,"va":5396952649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12157480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952208,"raw_hex":"e8d87d46ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5396952339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1774,"record_offset":39280,"parent_record_offset":38528,"ancestor_record_offset":38528,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1774,"record_offset":38712,"parent_record_offset":38528,"ancestor_record_offset":38528,"inlinee":30730,"source_line":103,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":39348,"parent_record_offset":38528,"ancestor_record_offset":38528,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1774,"record_offset":38684,"parent_record_offset":38528,"ancestor_record_offset":38528,"inlinee":30729,"source_line":425,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1774,"record_offset":39188,"parent_record_offset":38528,"ancestor_record_offset":38528,"inlinee":35140,"source_line":669,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":39076,"parent_record_offset":39052,"ancestor_record_offset":38528,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":39100,"parent_record_offset":39076,"ancestor_record_offset":38528,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":39124,"parent_record_offset":39100,"ancestor_record_offset":38528,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":39052,"parent_record_offset":38996,"ancestor_record_offset":38528,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1774,"record_offset":38788,"parent_record_offset":38768,"ancestor_record_offset":38528,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1774,"record_offset":38836,"parent_record_offset":38812,"ancestor_record_offset":38528,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":39024,"parent_record_offset":38996,"ancestor_record_offset":38528,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":38996,"parent_record_offset":38972,"ancestor_record_offset":38528,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":38972,"parent_record_offset":38948,"ancestor_record_offset":38528,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1774,"record_offset":38872,"parent_record_offset":38712,"ancestor_record_offset":38528,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":38812,"parent_record_offset":38788,"ancestor_record_offset":38528,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":38900,"parent_record_offset":38712,"ancestor_record_offset":38528,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::read_float_element::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1774,"record_offset":38768,"parent_record_offset":38712,"ancestor_record_offset":38528,"inlinee":85992,"source_line":183,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":1774,"record_offset":39224,"parent_record_offset":39188,"ancestor_record_offset":38528,"inlinee":35141,"source_line":162,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_float_element<3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":37656,"rva":28242480,"va":5396951600}
// 编译实例 RVAs: 0x1aef230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12292485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951600,"raw_hex":"e87b6e44ff","size":5,"soft_fail":false,"target_va":5384659264,"va":5396951744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21273489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951600,"raw_hex":"e86f64bbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396952188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19809479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951600,"raw_hex":"e8c7442e01","size":5,"soft_fail":false,"target_va":5416761616,"va":5396952132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19810362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951600,"raw_hex":"e83a482e01","size":5,"soft_fail":false,"target_va":5416762432,"va":5396952065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19814189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951600,"raw_hex":"e82d572e01","size":5,"soft_fail":false,"target_va":5416765984,"va":5396951790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951600,"raw_hex":"e8ffdeffff","size":5,"soft_fail":false,"target_va":5396943680,"va":5396952124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12156872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396951600,"raw_hex":"e8388046ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5396951731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1774,"record_offset":38428,"parent_record_offset":37656,"ancestor_record_offset":37656,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1774,"record_offset":37840,"parent_record_offset":37656,"ancestor_record_offset":37656,"inlinee":30730,"source_line":103,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":38496,"parent_record_offset":37656,"ancestor_record_offset":37656,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1774,"record_offset":37812,"parent_record_offset":37656,"ancestor_record_offset":37656,"inlinee":30729,"source_line":425,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1774,"record_offset":38316,"parent_record_offset":37656,"ancestor_record_offset":37656,"inlinee":35140,"source_line":669,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":37656,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":38228,"parent_record_offset":38204,"ancestor_record_offset":37656,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":38252,"parent_record_offset":38228,"ancestor_record_offset":37656,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":38180,"parent_record_offset":38124,"ancestor_record_offset":37656,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1774,"record_offset":37916,"parent_record_offset":37896,"ancestor_record_offset":37656,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1774,"record_offset":37964,"parent_record_offset":37940,"ancestor_record_offset":37656,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":38152,"parent_record_offset":38124,"ancestor_record_offset":37656,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":38124,"parent_record_offset":38100,"ancestor_record_offset":37656,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":38100,"parent_record_offset":38076,"ancestor_record_offset":37656,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1774,"record_offset":38000,"parent_record_offset":37840,"ancestor_record_offset":37656,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":37940,"parent_record_offset":37916,"ancestor_record_offset":37656,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":38028,"parent_record_offset":37840,"ancestor_record_offset":37656,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::read_float_element::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1774,"record_offset":37896,"parent_record_offset":37840,"ancestor_record_offset":37656,"inlinee":85992,"source_line":183,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":1774,"record_offset":38360,"parent_record_offset":38316,"ancestor_record_offset":37656,"inlinee":35141,"source_line":162,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_float_element<4>
// 记录形式: compiled_procedure; instances=1
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":39380,"rva":28243616,"va":5396952736}
// 编译实例 RVAs: 0x1aef6a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12293621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952736,"raw_hex":"e80b6a44ff","size":5,"soft_fail":false,"target_va":5384659264,"va":5396952880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21274721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952736,"raw_hex":"e89f5fbbfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396953420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19808260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952736,"raw_hex":"e804402e01","size":5,"soft_fail":false,"target_va":5416761616,"va":5396953351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19809143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952736,"raw_hex":"e877432e01","size":5,"soft_fail":false,"target_va":5416762432,"va":5396953284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_json.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19813053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952736,"raw_hex":"e8bd522e01","size":5,"soft_fail":false,"target_va":5416765984,"va":5396952926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952736,"raw_hex":"e83cdaffff","size":5,"soft_fail":false,"target_va":5396943680,"va":5396953343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12158008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396952736,"raw_hex":"e8c87b46ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5396952867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1774,"record_offset":40172,"parent_record_offset":39380,"ancestor_record_offset":39380,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1774,"record_offset":39564,"parent_record_offset":39380,"ancestor_record_offset":39380,"inlinee":30730,"source_line":103,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":40240,"parent_record_offset":39380,"ancestor_record_offset":39380,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_array
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1774,"record_offset":39536,"parent_record_offset":39380,"ancestor_record_offset":39380,"inlinee":30729,"source_line":425,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_f64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1774,"record_offset":40040,"parent_record_offset":39380,"ancestor_record_offset":39380,"inlinee":35140,"source_line":669,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":39928,"parent_record_offset":39904,"ancestor_record_offset":39380,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":39952,"parent_record_offset":39928,"ancestor_record_offset":39380,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":39976,"parent_record_offset":39952,"ancestor_record_offset":39380,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1774,"record_offset":39904,"parent_record_offset":39848,"ancestor_record_offset":39380,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1774,"record_offset":39640,"parent_record_offset":39620,"ancestor_record_offset":39380,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1774,"record_offset":39688,"parent_record_offset":39664,"ancestor_record_offset":39380,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":39876,"parent_record_offset":39848,"ancestor_record_offset":39380,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":39848,"parent_record_offset":39824,"ancestor_record_offset":39380,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1774,"record_offset":39824,"parent_record_offset":39800,"ancestor_record_offset":39380,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1774,"record_offset":39724,"parent_record_offset":39564,"ancestor_record_offset":39380,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":39664,"parent_record_offset":39640,"ancestor_record_offset":39380,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1774,"record_offset":39752,"parent_record_offset":39564,"ancestor_record_offset":39380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::load_json::read_float_element::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/load_json.rs
//     evidence={"module":1774,"record_offset":39620,"parent_record_offset":39564,"ancestor_record_offset":39380,"inlinee":85992,"source_line":183,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::as_f64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/number.rs
//     evidence={"module":1774,"record_offset":40088,"parent_record_offset":40040,"ancestor_record_offset":39380,"inlinee":35141,"source_line":162,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::load_json::read_float_element::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14FE8
// 首条位置: {"module":1770,"inlinee":85992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1774,"record_offset":37916,"parent_record_offset":37896,"ancestor_record_offset":37896,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1774,"record_offset":37964,"parent_record_offset":37940,"ancestor_record_offset":37896,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1774,"record_offset":37940,"parent_record_offset":37916,"ancestor_record_offset":37896,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01774.jsonl.gz"}
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
// 完整结构及原始类型记录: load_json.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_json.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_json.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(char)
// 候选参数类型: ["char"]
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0::closure$0
// 信息 ID: c494e8a9c6dcf011056e0eb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0 <- anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0<ref$<std::path::PathBuf> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0 <- anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0<std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0 <- anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0<ref$<str$> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0<ref$<std::path::PathBuf> > >; module=1253, inline_record=29768; 不是独立调用边。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0<std::path::PathBuf> >; module=3212, inline_record=564; 不是独立调用边。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,country_core::utils::load_json::load_json_as_mesh::closure$0::closure_env$0<ref$<str$> > >; module=4808, inline_record=552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0<alloc::string::String>
// 信息 ID: 1f2010b705b44db7db6f2e7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::load_json::load_json_as_mesh::closure$0 -> ["alloc::string::String"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> | country_core::utils::load_json::load_json_as_mesh<alloc::string::String> @ 0x14df2b5 -> 0x14df7e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> | country_core::utils::load_json::load_json_as_mesh<alloc::string::String> @ 0x14df3c4 -> 0x14df7e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed63d -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed744 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aedd2f -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aede3e -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee4cd -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee5d4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeebad -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeecb4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<ref$<str$> > >
// 信息 ID: 5bd243a0603ba21723d1d3db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["ref$<str$>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh::closure$0 -> ["ref$<ref$<str$> >"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed63d -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed744 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aedd2f -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aede3e -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee4cd -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee5d4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeebad -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeecb4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<std::path::Path> >
// 信息 ID: 1924b29b745ffb039a82666e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::Path"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh::closure$0 -> ["ref$<std::path::Path>"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed63d -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed744 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aedd2f -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aede3e -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee4cd -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee5d4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeebad -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeecb4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<std::path::PathBuf> >
// 信息 ID: 32b3aed3599c42c88a69b278
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh::closure$0 -> ["ref$<std::path::PathBuf>"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed63d -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed744 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aedd2f -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aede3e -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee4cd -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee5d4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeebad -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeecb4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<str$> >
// 信息 ID: 17ef5d6eb7684d03728d145c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh::closure$0 -> ["ref$<str$>"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed63d -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed744 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aedd2f -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aede3e -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee4cd -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee5d4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeebad -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeecb4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x28bb13d -> 0x28bb5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x28bb244 -> 0x28bb5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh::closure$0<std::path::PathBuf>
// 信息 ID: 80a1888b3f192f7d2e2107a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::load_json::load_json_as_mesh::closure$0 -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> | country_core::utils::load_json::load_json_as_mesh<alloc::string::String> @ 0x14df2b5 -> 0x14df7e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> | country_core::utils::load_json::load_json_as_mesh<alloc::string::String> @ 0x14df3c4 -> 0x14df7e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed63d -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh<ref$<str$> > @ 0x1aed744 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aedd2f -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf> @ 0x1aede3e -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee4cd -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > > @ 0x1aee5d4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeebad -> 0x1aef020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> > @ 0x1aeecb4 -> 0x1aef020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<alloc::string::String>
// 信息 ID: 72753ca7b3d101de1b60e850
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x872720 -> 0x14df050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$1 @ 0x876d49 -> 0x14df050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa66d78 -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xba8b33 -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_meadow::init_meadow @ 0xc7d8de -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::spawn_brush_previews::spawn_brush_previews @ 0x151308e -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > > @ 0x1812bf2 -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > > @ 0x1812ce5 -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<ref$<str$> > >
// 信息 ID: 473d9416668531b610ff9d56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["ref$<str$>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::bake_layer @ 0xc6a39a -> 0x1aee260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<std::path::Path> >
// 信息 ID: 57436db43b2d809a4d3077a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::Path"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<std::path::Path>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 66 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f252d -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2898 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2d06 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2fd4 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f31dc -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2d6a -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2aab -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::reload_mesh_src @ 0xa121ad -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_garden_brush::startup_garden_brush @ 0xa1cb0d -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_garden_brush::startup_garden_brush @ 0xa1cd60 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa5032e -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa503f5 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa504be -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa5105b -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa51102 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_terrain::startup_terrain @ 0xa53182 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >
// 信息 ID: 201cc5ee0511dd1b899825b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<std::path::PathBuf>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_and_prepare_nani_mesh @ 0xa8b6ab -> 0x1aee940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
// 信息 ID: ddec8d1feb9fb73be667996f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 68 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f252d -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2898 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2d06 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2fd4 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f31dc -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2d6a -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x9f2aab -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::reload_mesh_src @ 0xa121ad -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_garden_brush::startup_garden_brush @ 0xa1cb0d -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_garden_brush::startup_garden_brush @ 0xa1cd60 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa5032e -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa503f5 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa504be -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa5105b -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sheep::startup_sheep @ 0xa51102 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_terrain::startup_terrain @ 0xa53182 -> 0x1aed3d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>
// 信息 ID: 08fbec600d7179367edb552e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::load_json::load_json_as_mesh -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x872720 -> 0x14df050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$1 @ 0x876d49 -> 0x14df050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa66d78 -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xba8b33 -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_meadow::init_meadow @ 0xc7d8de -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::spawn_brush_previews::spawn_brush_previews @ 0x151308e -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > > @ 0x1812bf2 -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > > @ 0x1812ce5 -> 0x1aedab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh
// 信息 ID: 077aef43a959c9ecd63bd4d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh::closure$0<alloc::string::String> | country_core::utils::load_json::load_json_as_mesh::closure$0<std::path::PathBuf> @ 0x14df954 -> 0x9d9d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<std::path::Path> > | country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<ref$<str$> > > | country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<std::path::PathBuf> > | country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<str$> > | 其余 2 个同地址别名见共享索引 @ 0x1aef194 -> 0x9d9d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::load_json::load_json_as_mesh::closure$0<ref$<str$> > @ 0x28bb724 -> 0x9d9d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$0
// 信息 ID: 673cba8aa5cfef5432d06c88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::load_json_str_as_mesh::closure_env$0 <- anyhow::context::impl$0::with_context<enum2$<serde_json::value::Value>,serde_json::error::Error,alloc::string::String,country_core::utils::load_json::load_json_str_as_mesh::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<enum2$<serde_json::value::Value>,serde_json::error::Error,alloc::string::String,country_core::utils::load_json::load_json_str_as_mesh::closure_env$0>; module=1253, inline_record=37276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$1
// 信息 ID: 5440fd3ef43d53b4abf9b769
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_str_as_mesh; module=521, inline_record=30828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$2
// 信息 ID: c08931dbbfe819aca7bc63b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0461f908e9ab558deddf5b05: enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > (*)(u32)
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::load_json_str_as_mesh::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::load_json_str_as_mesh::closure_env$2 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<u32,anyhow::Error> >,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<u32,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > (*)(u32)>,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::load_json_str_as_mesh::closure_env$2 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<u32,anyhow::Error> >,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<u32,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > (*)(u32)>,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<u32,anyhow::Error> >,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<u32,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > (*)(u32)>,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=160140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$2::closure$1
// 信息 ID: 9372ffcbc78c836231564a65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0461f908e9ab558deddf5b05: enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > (*)(u32)
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<u32,anyhow::Error> >,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$2>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<u32,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > (*)(u32)>,enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<u32,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=160252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$4
// 信息 ID: 38dbb10a995a367bf5bfddad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_str_as_mesh; module=521, inline_record=34208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$5
// 信息 ID: 724a408f5c951cb970485606
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dbbafca988cafc9d3cdb5eb2: enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > (*)(ref$<str$>)
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::load_json_str_as_mesh::closure_env$5 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::load_json_str_as_mesh::closure_env$5 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<ref$<str$>,anyhow::Error> >,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<ref$<str$>,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > (*)(ref$<str$>)>,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::load_json_str_as_mesh::closure_env$5 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<ref$<str$>,anyhow::Error> >,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<ref$<str$>,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > (*)(ref$<str$>)>,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<ref$<str$>,anyhow::Error> >,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<ref$<str$>,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > (*)(ref$<str$>)>,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=188504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$5::closure$0
// 信息 ID: a718f0ca01543a6ad9a8a8fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dbbafca988cafc9d3cdb5eb2: enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > (*)(ref$<str$>)
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<ref$<str$>,anyhow::Error> >,core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::load_json_str_as_mesh::closure_env$5>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<ref$<str$>,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > (*)(ref$<str$>)>,enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<ref$<str$>,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=188624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$6
// 信息 ID: d81381c4b51976316951b0e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_str_as_mesh; module=521, inline_record=39784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::load_json_str_as_mesh::closure$7
// 信息 ID: 8c73d3795e0e52791691b7f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "country_core::utils::load_json::load_json_str_as_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_str_as_mesh; module=521, inline_record=40452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json
// 信息 ID: f8c70e677c252c95baaeee73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_str_as_mesh; module=521, inline_record=35556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$0
// 信息 ID: a69d3301080651c4f24cde54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<i32,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<i32,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; module=1643, inline_record=387840; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<i32,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; module=1643, inline_record=388916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_attribute_from_json::closure$1
// 信息 ID: 8e235c54f488d65f5a62c6de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::load_json::read_attribute_from_json", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1>,f32,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,core::result::impl$26::from_iter::closure_env$0<f32,anyhow::Error,alloc::vec::Vec<f32,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> >,alloc::vec::Vec<f32,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1>,f32,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,core::result::impl$26::from_iter::closure_env$0<f32,anyhow::Error,alloc::vec::Vec<f32,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1> >,alloc::vec::Vec<f32,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::load_json::read_attribute_from_json::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; module=1643, inline_record=460900; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,country_core::utils::load_json::read_attribute_from_json::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; module=1643, inline_record=462060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_float_element::closure$0
// 信息 ID: 37b8de7bf243dbdebe491fdc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::load_json::read_float_element", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::read_float_element<3>; module=1774, inline_record=37896; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::read_float_element<2>; module=1774, inline_record=38768; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::read_float_element<4>; module=1774, inline_record=39620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_float_element<2>
// 信息 ID: 425126f9c1063657902fc5d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 154002d2f5bfe7fc76a38620: enum2$<core::result::Result<array$<f32,2>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)
// [类型关系] country_core::utils::load_json::read_float_element -> ["2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,2>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,enum2$<core::result::Result<array$<f32,2>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > @ 0x1618117 -> 0x1aef490 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,2>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,enum2$<core::result::Result<array$<f32,2>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > @ 0x16181d5 -> 0x1aef490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_float_element<3>
// 信息 ID: f27092a587b63ada0cab4808
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d80d0b3c28330a8b288f1fa4: enum2$<core::result::Result<array$<f32,3>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)
// [类型关系] country_core::utils::load_json::read_float_element -> ["3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,enum2$<core::result::Result<array$<f32,3>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > @ 0x1615797 -> 0x1aef230 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,enum2$<core::result::Result<array$<f32,3>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > @ 0x1615857 -> 0x1aef230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::load_json::read_float_element<4>
// 信息 ID: 723a86c9269d70429c7481d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 18509fc59ad6f43fd91822bc: enum2$<core::result::Result<array$<f32,4>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)
// [类型关系] country_core::utils::load_json::read_float_element -> ["4"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,4>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,enum2$<core::result::Result<array$<f32,4>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > @ 0x1619c78 -> 0x1aef6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,4>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde_json::value::Value> >,enum2$<core::result::Result<array$<f32,4>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > @ 0x1619d3f -> 0x1aef6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
